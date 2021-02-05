mod channel_map;
mod join;
mod turbulence_impl;

use async_trait::async_trait;
use channel_map::Connections;
use fg_netcode::{HostError, JoinError, NetworkingSubsytem};
use futures::FutureExt;
use join::{JoinRequest, JoinResponse};
use smol::{
    future::yield_now,
    lock::RwLock,
    net::{AsyncToSocketAddrs, UdpSocket},
    Executor, Task,
};
use std::{net::SocketAddr, ops::DerefMut, sync::Arc};

type BackendHandle = Arc<RwLock<BackendInner>>;

pub struct UdpBackend {
    handle: BackendHandle,
    _task: Task<()>,
}

pub struct BackendInner {
    mode: Mode,
    socket: UdpSocket,
    connections: Connections,
    self_addr: SocketAddr,
    runtime: Arc<Executor<'static>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Mode {
    Disconnected,
    Host,
    Client(SocketAddr),
}

pub type LobbyId = SocketAddr;
pub type PlayerId = SocketAddr;
pub type GameId = (LobbyId, usize);

#[async_trait]
impl NetworkingSubsytem for UdpBackend {
    type LobbyId = LobbyId;

    type GameId = GameId;

    type PlayerId = PlayerId;

    async fn request_host(&mut self) -> Result<Self::LobbyId, HostError> {
        let mut inner = self.handle.write().await;
        if inner.mode != Mode::Disconnected {
            Err(HostError::AlreadyHosting)
        } else {
            inner.mode = Mode::Host;
            Ok(inner.socket.local_addr().unwrap())
        }
    }

    async fn request_join(&mut self, lobby: Self::LobbyId) -> Result<Self::LobbyId, JoinError> {
        let handle = self.handle.clone();
        let (incoming, outgoing) = {
            let mut inner = self.handle.write().await;
            let inner = inner.deref_mut();
            let connection = inner.connections.get_connection(
                lobby,
                inner.socket.clone(),
                inner.runtime.clone(),
            );
            (
                connection.incoming.join_response.recv.clone(),
                connection.outgoing.join_request.send.clone(),
            )
        };

        outgoing.send(JoinRequest { addr: lobby }).await.unwrap();
        let response = incoming.recv().await.unwrap();

        match response {
            JoinResponse::Denied => Err(JoinError::Denied),
            JoinResponse::Accepted { self_addr } => {
                let mut lock = handle.write().await;
                lock.mode = Mode::Client(lobby);
                lock.self_addr = self_addr;
                Ok(lobby)
            }
        }
    }
}

async fn main_loop(handle: BackendHandle) {
    let mut temp_buffer = vec![0; 1024];
    loop {
        {
            let mut handle = handle.write().await;
            let mut handle = handle.deref_mut();
            let mut updated_address = None;
            for (_, connection) in handle.connections.connections.iter() {
                if let Ok(request) = connection.incoming.join_request.recv.try_recv() {
                    updated_address = Some(request.addr);

                    connection
                        .outgoing
                        .join_response
                        .send
                        .send(JoinResponse::Denied)
                        .await
                        .unwrap();
                }
            }

            if let Some(Ok((_, addr))) = handle.socket.peek_from(&mut temp_buffer).now_or_never() {
                if !handle.connections.connections.contains_key(&addr) {
                    handle.connections.get_connection(
                        addr,
                        handle.socket.clone(),
                        handle.runtime.clone(),
                    );
                }
            }

            if let Some(updated_address) = updated_address {
                handle.self_addr = updated_address;
            }
        }
        yield_now().await
    }
}

impl UdpBackend {
    pub fn new<A: AsyncToSocketAddrs>(addr: A, runtime: Arc<Executor<'static>>) -> Self {
        let socket = smol::block_on(UdpSocket::bind(addr)).unwrap();

        let handle = Arc::new(RwLock::new(BackendInner {
            mode: Mode::Disconnected,
            self_addr: socket.local_addr().unwrap(),
            runtime: runtime.clone(),
            socket,
            connections: Connections::default(),
        }));

        let task = runtime.spawn(main_loop(handle.clone()));

        Self {
            handle,
            _task: task,
        }
    }
}

#[cfg(test)]
mod test {
    use std::{net::ToSocketAddrs, sync::Arc};

    use fg_netcode::{JoinError, NetworkingSubsytem};
    use smol::Executor;

    use crate::UdpBackend;

    #[test]
    fn integ_test() {
        let exec = Arc::new(Executor::new());
        let _server = UdpBackend::new("127.0.0.1:10800", exec.clone());
        let mut client = UdpBackend::new("127.0.0.1:10801", exec.clone());

        smol::block_on(exec.run(exec.spawn(async move {
            assert_eq!(
                client
                    .request_join("127.0.0.1:10800".to_socket_addrs().unwrap().next().unwrap())
                    .await,
                Err(JoinError::Denied)
            );
        })));
    }
}
