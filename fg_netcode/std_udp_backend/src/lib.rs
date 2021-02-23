mod backend;
mod cert;
mod connection;
mod lobby_state;
mod request;

use backend::NetworkBackend;

use backend::State;
use fg_netcode::{player_info::PlayerInfo, NetworkingMessage};

use quinn::{
    crypto::ServerConfig, Certificate, CertificateChain, Endpoint, Incoming, PrivateKey,
    ServerConfigBuilder, TransportConfig,
};

use std::{collections::HashSet, net::SocketAddr, time::Duration};
use tokio::{
    runtime::{Builder as TokioBuilder, Handle},
    select,
    sync::{mpsc, watch},
    task::JoinHandle,
};

#[derive(Debug)]
pub enum NetworkingAction {
    Host(PlayerInfo),
    ConnectTo(PlayerInfo, SocketAddr),
}
struct QuinnHandle {
    pub(crate) endpoint: Endpoint,
    pub(crate) incoming: Incoming,
}

pub struct BackendInterface {
    pub messages: async_channel::Receiver<NetworkingMessage>,
    pub actions: mpsc::Sender<NetworkingAction>,
    pub shutdown: mpsc::Receiver<()>,
    _task: JoinHandle<Option<()>>,
}

pub fn start(addr: SocketAddr, handle: Handle) -> BackendInterface {
    let (message_tx, message_rx) = async_channel::bounded(4);
    let (action_tx, action_rx) = mpsc::channel(4);

    let (disconnect_tx, disconnect_rx) = mpsc::channel(1);

    let state = State::Disconnected(NetworkBackend {
        messages: message_tx,
        actions: action_rx,
        handle: handle.clone(),
    });
    BackendInterface {
        messages: message_rx,
        actions: action_tx,
        shutdown: disconnect_rx,
        _task: handle.spawn(main_loop(addr, state, disconnect_tx)),
    }
}

async fn main_loop(addr: SocketAddr, mut state: State, shutdown: mpsc::Sender<()>) -> Option<()> {
    let mut builder = quinn::EndpointBuilder::default();
    builder.default_client_config(cert::configure_client());
    {
        let mut server_config = ServerConfigBuilder::default();
        let mut transport_config = TransportConfig::default();
        transport_config.keep_alive_interval(Some(Duration::from_secs(2)));
        transport_config
            .max_idle_timeout(Some(Duration::from_secs(5)))
            .unwrap();

        let cert = rcgen::generate_simple_self_signed(vec!["lobby_server".into()]).unwrap();
        let cert_der = cert.serialize_der().unwrap();
        let priv_key = cert.serialize_private_key_der();
        let priv_key = PrivateKey::from_der(&priv_key).unwrap();

        server_config
            .certificate(
                CertificateChain::from_certs(std::iter::once(
                    Certificate::from_der(&cert_der).unwrap(),
                )),
                priv_key,
            )
            .unwrap();

        builder.listen(server_config.build());
    }
    let (endpoint, incoming) = builder.bind(&addr).ok()?;
    let mut quinn = QuinnHandle { endpoint, incoming };
    loop {
        state = select! {
            value = state.main_loop(&mut quinn) => value,
            _ = shutdown.closed() => {
                quinn.endpoint.close(0u16.into(), &[]);
                break None;
            },

        };
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use fg_netcode::{player_info::PlayerInfo, NetworkingMessage};
    use tokio::{
        sync::mpsc,
        task::{yield_now, JoinHandle},
    };

    use crate::{start, NetworkingAction};

    #[test]
    fn integ() {
        let mut builder = tokio::runtime::Builder::new_current_thread();
        builder.enable_all();
        let rt = builder.build().unwrap();
        let handle = rt.handle().clone();

        let _task = std::thread::spawn(move || {
            rt.enter();
            rt.block_on(async move {
                loop {
                    yield_now().await
                }
            })
        });

        let host_addr = "127.0.0.1:10800".parse().unwrap();
        let host = start(host_addr, handle.clone());
        host.actions
            .blocking_send(NetworkingAction::Host(PlayerInfo {
                name: "Host".to_string(),
                character: Default::default(),
                addr: host_addr,
            }))
            .unwrap();
        let host_lobby = loop {
            if let Ok(NetworkingMessage::Host(Ok(lobby))) = host.messages.try_recv() {
                break lobby;
            }
        };

        let client_addr = "127.0.0.1:10801".parse().unwrap();
        let mut client = start(client_addr, handle.clone());
        client
            .actions
            .blocking_send(NetworkingAction::ConnectTo(
                PlayerInfo {
                    name: "Client".to_string(),
                    character: Default::default(),
                    addr: client_addr,
                },
                host_addr,
            ))
            .unwrap();
        let client_lobby = loop {
            match client.messages.try_recv() {
                Ok(NetworkingMessage::Join(Ok(lobby))) => break lobby,
                Ok(NetworkingMessage::Join(Err(err))) => panic!("{:?}", err),
                _ => (),
            }
        };

        let client_addr2 = "127.0.0.1:10802".parse().unwrap();
        let client2 = start(client_addr2, handle.clone());
        client2
            .actions
            .blocking_send(NetworkingAction::ConnectTo(
                PlayerInfo {
                    name: "Client 2".to_string(),
                    character: Default::default(),
                    addr: client_addr2,
                },
                host_addr,
            ))
            .unwrap();
        let client_lobby2 = loop {
            match client2.messages.try_recv() {
                Ok(NetworkingMessage::Join(Ok(lobby))) => break lobby,
                Ok(NetworkingMessage::Join(Err(err))) => panic!("{:?}", err),
                _ => {}
            }
        };

        std::thread::sleep(Duration::from_millis(1));

        assert_eq!(
            host_lobby.state().player_list,
            client_lobby.state().player_list
        );
        assert_eq!(
            host_lobby.state().player_list,
            client_lobby2.state().player_list
        );
        assert_eq!(
            client_lobby.state().player_list,
            client_lobby2.state().player_list
        );

        assert_eq!(host_lobby.state().games, client_lobby.state().games);
        assert_eq!(host_lobby.state().games, client_lobby2.state().games);
        assert_eq!(client_lobby.state().games, client_lobby2.state().games);

        assert_ne!(host_lobby.state().user, client_lobby.state().user);

        client.shutdown.close();

        std::thread::sleep(Duration::from_secs(1));

        assert_ne!(
            host_lobby.state().player_list,
            client_lobby.state().player_list
        );
        assert_eq!(
            host_lobby.state().player_list,
            client_lobby2.state().player_list
        );
        assert_ne!(
            client_lobby.state().player_list,
            client_lobby2.state().player_list
        );
    }
}

/*
when disconnected:
    - connections list should be empty
    - all incoming connections should be recv'd and denied
    - actions should recv'd and properly handled
    - when hosting or joining is successful move to InLobby
when in_lobby(host):
    - connections list should be equal to the amount of people in lobby
    - incoming connections should be recv'd and assumed to be joinrequests
    - when lobby state changes, every outgoing connection should receieve an update
when in_lobby(client):
    - connections list should be equal to the host only
    = incoming connections should be recv'd and denied
    - incoming uni-streams from host should be recv'd and assume to indicate lobby state updates

*/
