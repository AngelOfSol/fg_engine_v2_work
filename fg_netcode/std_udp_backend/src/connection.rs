use crate::{
    lobby_state::{LobbyStateAction, LobbyStateInterface},
    request::{Disconnected, HostPacket},
};
use bytes::Bytes;
use fg_netcode::{lobby::lobby_state::LobbyState, player_list::Player};
use futures_util::StreamExt;
use quinn::{
    Connection, ConnectionError, Datagrams, IncomingBiStreams, IncomingUniStreams, NewConnection,
    RecvStream, SendStream,
};
use tokio::{select, task::JoinHandle};

pub fn handle_incoming(
    conn: NewConnection,
    peer_id: Player,
    lsi: LobbyStateInterface,
    connection_type: ConnectionType,
) -> JoinHandle<()> {
    let back_conn = BackendConnection::new(conn, peer_id, lsi, connection_type);

    tokio::spawn(async move {
        let data = main_loop(back_conn).await;
    })
}

pub struct BackendConnection {
    pub connection: Connection,
    pub uni_streams: IncomingUniStreams,
    pub bi_streams: IncomingBiStreams,
    pub datagrams: Datagrams,

    pub lsi: LobbyStateInterface,
    pub peer_id: Player,
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
        lsi: LobbyStateInterface,
        connection_type: ConnectionType,
    ) -> Self {
        Self {
            connection: conn.connection,
            uni_streams: conn.uni_streams,
            bi_streams: conn.bi_streams,
            datagrams: conn.datagrams,
            peer_id,
            lsi,
            connection_type,
        }
    }

    fn update_connection_type(&mut self) {
        let lobby_state = self.lsi.state.borrow();
        self.connection_type = ConnectionType::from_peer(self.peer_id, &lobby_state);
    }

    async fn handle_uni(
        &mut self,
        stream: Option<Result<RecvStream, ConnectionError>>,
    ) -> Result<(), Disconnected> {
        let stream = stream.ok_or(Disconnected)??;

        match self.connection_type {
            ConnectionType::PeerToHost => {
                let value =
                    match bincode::deserialize::<LobbyState>(&stream.read_to_end(1000).await?) {
                        Ok(value) => value,
                        Err(_) => return Ok(()),
                    };
                self.lsi
                    .actions
                    .send(LobbyStateAction::RawUpdate(value))
                    .await
                    .unwrap()
            }
            ConnectionType::HostToPeer | ConnectionType::PeerToPeer => {}
        }

        Ok(())
    }

    async fn handle_bi(
        &mut self,
        (send, recv): (SendStream, RecvStream),
    ) -> Result<(), Disconnected> {
        Ok(())
    }

    async fn handle_datagram(&mut self, bytes: Bytes) -> Result<(), Disconnected> {
        Ok(())
    }

    async fn lsi_changed(&mut self) -> Result<(), Disconnected> {
        self.update_connection_type();

        match self.connection_type {
            ConnectionType::HostToPeer => {
                let mut send = self.connection.open_uni().await?;
                let mut data = self.lsi.state.borrow().clone();
                data.user = self.peer_id;
                send.write_all(&bincode::serialize(&data).unwrap()).await?;
                send.finish().await?;
            }
            ConnectionType::PeerToHost | ConnectionType::PeerToPeer => {}
        }

        Ok(())
    }
}

pub enum ConnectionType {
    HostToPeer,
    PeerToHost,
    PeerToPeer,
}

async fn main_loop(mut connection: BackendConnection) -> Result<(), Disconnected> {
    loop {
        let status = select! {
            Ok(()) = connection.lsi.state.changed() => connection.lsi_changed().await,
            incoming = connection.uni_streams.next() => connection.handle_uni(incoming).await,
            Some(Ok(incoming)) = connection.bi_streams.next() => connection.handle_bi(incoming).await,
            Some(Ok(incoming)) = connection.datagrams.next() => connection.handle_datagram(incoming).await,
            else => Err(Disconnected),
        };
        match status {
            Ok(_) => {}
            Err(_) => {
                match connection.connection_type {
                    ConnectionType::HostToPeer => {
                        let _ = connection
                            .lsi
                            .actions
                            .send(LobbyStateAction::Disconnect(connection.peer_id))
                            .await;
                    }
                    ConnectionType::PeerToHost => {
                        let _ = connection.lsi.actions.send(LobbyStateAction::Kill).await;
                    }
                    ConnectionType::PeerToPeer => {}
                }
                break;
            }
        }
    }

    Err(Disconnected)
}
