use std::{collections::HashMap, net::SocketAddr};

use futures::SinkExt;
use smol::{
    channel::{unbounded, Receiver, Sender},
    future::yield_now,
    net::UdpSocket,
    stream::StreamExt,
};
use turbulence::{
    message_channels::ChannelMessage, BufferPacket, BufferPacketPool, IncomingMultiplexedPackets,
    MessageChannelSettings, MessageChannels, MessageChannelsBuilder, OutgoingMultiplexedPackets,
    Packet, PacketMultiplexer, PacketPool, Runtime,
};

use crate::{
    join::{self, JoinRequest, JoinResponse},
    turbulence_impl::SimpleBufferPool,
};

#[derive(Clone)]
pub struct Channel<T> {
    pub send: Sender<T>,
    pub recv: Receiver<T>,
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self { send: tx, recv: rx }
    }
}

#[derive(Clone)]
pub struct ChannelList {
    pub join_request: Channel<JoinRequest>,
    pub join_response: Channel<JoinResponse>,
}

impl Default for ChannelList {
    fn default() -> Self {
        Self {
            join_request: Channel::default(),
            join_response: Channel::default(),
        }
    }
}
#[derive(Clone, Default)]
pub struct Connection {
    pub incoming: ChannelList,
    pub outgoing: ChannelList,
}

#[derive(Clone, Default)]
pub struct Connections {
    pub connections: HashMap<SocketAddr, Connection>,
}

impl Connections {
    pub fn get_connection<R: Runtime + 'static>(
        &mut self,
        addr: SocketAddr,
        socket: UdpSocket,
        executor: R,
    ) -> &mut Connection {
        let exists = self.connections.contains_key(&addr);
        let connection = self.connections.entry(addr).or_default();
        if !exists {
            let Connection { outgoing, incoming } = connection;
            let (messages, raw_in, raw_out) = build_message_channels(executor.clone());

            executor.spawn(out_loop(raw_out, addr, socket.clone()));
            executor.spawn(in_loop(raw_in, addr, socket));

            executor.spawn(messages_loop(outgoing.clone(), incoming.clone(), messages));
        }

        connection
    }
}

async fn messages_loop(
    outgoing: ChannelList,
    incoming: ChannelList,
    mut messages: MessageChannels,
) {
    let messages = &mut messages;
    loop {
        incoming.join_request.try_forward_incoming(messages).await;
        incoming.join_response.try_forward_incoming(messages).await;

        outgoing.join_request.try_forward_outgoing(messages).await;
        outgoing.join_response.try_forward_outgoing(messages).await;

        yield_now().await
    }
}
#[cfg(test)]
mod test {
    use std::{net::ToSocketAddrs, sync::Arc};

    use smol::{net::UdpSocket, Executor};

    use crate::{join::JoinResponse, turbulence_impl::TurbulenceRuntime};

    use super::Connections;

    #[test]
    fn integ_test_channel_map() {
        let exec = Arc::new(Executor::new());
        let client_addr = "127.0.0.1:10801".to_socket_addrs().unwrap().next().unwrap();
        let server_addr = "127.0.0.1:10800".to_socket_addrs().unwrap().next().unwrap();

        let server = {
            let mut conn = Connections::default();
            let socket = smol::block_on(UdpSocket::bind("127.0.0.1:10800")).unwrap();
            conn.get_connection(client_addr, socket, TurbulenceRuntime::from(exec.clone()));
            conn
        };
        let client = {
            let mut conn = Connections::default();
            let socket = smol::block_on(UdpSocket::bind("127.0.0.1:10801")).unwrap();
            conn.get_connection(server_addr, socket, TurbulenceRuntime::from(exec.clone()));
            conn
        };

        smol::block_on(exec.run(async move {
            server.connections[&client_addr]
                .outgoing
                .join_response
                .send
                .send(JoinResponse::Denied)
                .await
                .unwrap();
            let response = client.connections[&server_addr]
                .incoming
                .join_response
                .recv
                .recv()
                .await
                .unwrap();

            assert_eq!(response, JoinResponse::Denied);
        }));
    }
}

impl<T: ChannelMessage> Channel<T> {
    async fn try_forward_outgoing(&self, messages: &mut MessageChannels) {
        if let Ok(value) = self.recv.try_recv() {
            messages.async_send(value).await.unwrap();
            messages.flush::<T>();
        }
    }

    async fn try_forward_incoming(&self, messages: &mut MessageChannels) {
        if let Some(value) = messages.recv::<T>() {
            self.send.send(value).await.unwrap()
        }
    }
}

type PacketType = BufferPacket<Box<[u8]>>;

async fn out_loop(
    mut raw_out: OutgoingMultiplexedPackets<PacketType>,
    addr: SocketAddr,
    socket: UdpSocket,
) {
    while let Some(value) = raw_out.next().await {
        if socket.send_to(&value, addr).await.is_err() {
            break;
        }
        yield_now().await
    }
}

async fn in_loop(
    mut raw_in: IncomingMultiplexedPackets<PacketType>,
    addr: SocketAddr,
    socket: UdpSocket,
) {
    let pool = BufferPacketPool::new(SimpleBufferPool(1024));
    loop {
        let mut packet = pool.acquire();
        packet.resize(1024, 0);
        match socket.peek_from(&mut packet).await {
            Ok((size, from_addr)) => {
                if addr == from_addr {
                    packet.truncate(size);
                    socket.recv_from(&mut packet).await.unwrap();
                    if raw_in.send(packet).await.is_err() {
                        break;
                    };
                }
            }
            Err(_e) => {}
        }
        yield_now().await
    }
}

fn build_message_channels<R: Runtime + 'static>(
    executor: R,
) -> (
    MessageChannels,
    IncomingMultiplexedPackets<PacketType>,
    OutgoingMultiplexedPackets<PacketType>,
) {
    let mut multiplexer = PacketMultiplexer::new();

    let mut messages =
        MessageChannelsBuilder::new(executor, BufferPacketPool::new(SimpleBufferPool(1024)));
    let mut x = 0;
    let mut new_channel = move || {
        let val = x;
        x += 1;
        val
    };
    messages
        .register::<JoinRequest>(MessageChannelSettings {
            channel: new_channel(),
            ..join::SETTINGS
        })
        .unwrap();
    messages
        .register::<JoinResponse>(MessageChannelSettings {
            channel: new_channel(),
            ..join::SETTINGS
        })
        .unwrap();

    let messages = messages.build(&mut multiplexer);
    let (incoming, outgoing) = multiplexer.start();

    (messages, incoming, outgoing)
}
