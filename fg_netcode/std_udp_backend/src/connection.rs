use crate::{
    lobby_state::LobbyStateAction,
    request::{ClientPacket, Disconnected},
    util,
};
use bytes::Bytes;
use fg_netcode::{
    lobby::{lobby_state::LobbyState, LobbyAction},
    player_list::Player,
};
use futures_util::StreamExt;
use quinn::{
    Connection, ConnectionError, Datagrams, IncomingBiStreams, IncomingUniStreams, NewConnection,
    RecvStream, SendStream,
};
use tokio::{
    select,
    sync::{broadcast, mpsc, watch},
    task::JoinHandle,
};

pub struct Peer {
    pub task: JoinHandle<()>,
}

pub fn handle_incoming(
    conn: NewConnection,
    peer_id: Player,
    lobby_state: watch::Receiver<LobbyState>,
    connection_type: ConnectionType,
    incoming: mpsc::Sender<LobbyStateAction>,
    outgoing: broadcast::Receiver<LobbyStateAction>,
) -> Peer {
    let back_conn = BackendConnection::new(
        conn,
        peer_id,
        lobby_state,
        incoming,
        outgoing,
        connection_type,
    );
    Peer {
        task: tokio::spawn(async move {
            let _ = main_loop(back_conn).await;
        }),
    }
}

pub struct BackendConnection {
    connection: Connection,
    uni_streams: IncomingUniStreams,
    bi_streams: IncomingBiStreams,
    datagrams: Datagrams,

    incoming: mpsc::Sender<LobbyStateAction>,
    outgoing: broadcast::Receiver<LobbyStateAction>,

    lobby_state: watch::Receiver<LobbyState>,

    peer_id: Player,
    connection_type: ConnectionType,
}

impl ConnectionType {
    pub fn from_peer(peer_id: Player, lobby_state: &LobbyState) -> Self {
        if lobby_state.is_user_host() {
            ConnectionType::HostToPeer
        } else if lobby_state.is_host(peer_id) {
            ConnectionType::PeerToHost
        } else {
            ConnectionType::PeerToPeer
        }
    }
}

impl BackendConnection {
    fn new(
        conn: NewConnection,
        peer_id: Player,
        lobby_state: watch::Receiver<LobbyState>,
        incoming: mpsc::Sender<LobbyStateAction>,
        outgoing: broadcast::Receiver<LobbyStateAction>,
        connection_type: ConnectionType,
    ) -> Self {
        Self {
            connection: conn.connection,
            uni_streams: conn.uni_streams,
            bi_streams: conn.bi_streams,
            datagrams: conn.datagrams,
            peer_id,
            incoming,
            outgoing,
            lobby_state,
            connection_type,
        }
    }

    fn update_connection_type(&mut self) {
        let lobby_state = self.lobby_state.borrow();
        self.connection_type = ConnectionType::from_peer(self.peer_id, &lobby_state);
    }

    async fn handle_uni(
        &mut self,
        stream: Option<Result<RecvStream, ConnectionError>>,
    ) -> Result<(), Disconnected> {
        let stream = stream.ok_or(Disconnected)??;

        match self.connection_type {
            ConnectionType::HostToPeer | ConnectionType::PeerToHost => {
                let value = util::read_from::<LobbyStateAction>(1000, stream).await?;
                self.incoming.send(value).await.map_err(|_| Disconnected)?;
            }
            ConnectionType::PeerToPeer => {}
        }

        Ok(())
    }

    async fn handle_bi(
        &mut self,
        (send, recv): (SendStream, RecvStream),
    ) -> Result<(), Disconnected> {
        Ok(())
    }

    async fn handle_datagram(&mut self, _bytes: Bytes) -> Result<(), Disconnected> {
        Ok(())
    }

    async fn handle_outgoing(&mut self, outgoing: LobbyStateAction) -> Result<(), Disconnected> {
        match self.connection_type {
            ConnectionType::PeerToHost | ConnectionType::HostToPeer => {
                let send = self.connection.open_uni().await?;
                util::write_to(&outgoing, send).await?;
            }
            ConnectionType::PeerToPeer => {
                // TODO warn
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    HostToPeer,
    PeerToHost,
    PeerToPeer,
}

async fn main_loop(mut connection: BackendConnection) -> Result<(), Disconnected> {
    loop {
        let status = select! {
            Ok(()) = connection.lobby_state.changed() => {
                connection.update_connection_type();
                Ok(())
            },
            Ok(outgoing) = connection.outgoing.recv() => connection.handle_outgoing(outgoing).await,
            incoming = connection.uni_streams.next() => connection.handle_uni(incoming).await,
            // Some(Ok(incoming)) = connection.bi_streams.next() => connection.handle_bi(incoming).await,
            // Some(Ok(incoming)) = connection.datagrams.next() => connection.handle_datagram(incoming).await,
            else => Err(Disconnected),
        };
        match status {
            Ok(_) => {}
            Err(_) => {
                match connection.connection_type {
                    ConnectionType::HostToPeer => {
                        let _ = connection
                            .incoming
                            .send(LobbyStateAction::Disconnect(connection.peer_id))
                            .await;
                    }
                    ConnectionType::PeerToHost => {
                        let _ = connection.incoming.send(LobbyStateAction::Kill).await;
                    }
                    ConnectionType::PeerToPeer => {}
                }
                break;
            }
        }
    }

    Err(Disconnected)
}
