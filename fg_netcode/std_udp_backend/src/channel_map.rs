// use std::{
//     collections::{hash_map::Entry, HashMap},
//     io::ErrorKind,
//     net::SocketAddr,
//     sync::Arc,
// };

// use futures::{Future, SinkExt};
// use smol::{
//     channel::{unbounded, Receiver, SendError, Sender},
//     future::yield_now,
//     net::UdpSocket,
//     stream::StreamExt,
//     Executor, Task,
// };
// use turbulence::{
//     message_channels::ChannelMessage, BufferPacket, BufferPacketPool, IncomingMultiplexedPackets,
//     MessageChannelSettings, MessageChannels, MessageChannelsBuilder, OutgoingMultiplexedPackets,
//     Packet, PacketMultiplexer, PacketPool, Runtime,
// };

// use crate::{
//     join::{self, JoinRequest, JoinResponse},
//     turbulence_impl::{SimpleBufferPool, TurbulenceRuntime},
// };

// #[derive(Clone)]
// pub struct Channel<T> {
//     pub outgoing: Sender<T>,
//     pub incoming: Receiver<T>,
// }

// impl<T> Channel<T> {
//     fn make() -> (Self, Self) {
//         let (in_tx, in_rx) = unbounded();
//         let (out_tx, out_rx) = unbounded();
//         (
//             Self {
//                 outgoing: out_tx,
//                 incoming: in_rx,
//             },
//             Self {
//                 outgoing: in_tx,
//                 incoming: out_rx,
//             },
//         )
//     }
// }

// #[derive(Clone)]
// pub struct ChannelList {
//     pub join_request: Channel<JoinRequest>,
//     pub join_response: Channel<JoinResponse>,
// }

// impl ChannelList {
//     fn make() -> (Self, Self) {
//         let (left_join_request, right_join_request) = Channel::make();
//         let (left_join_response, right_join_response) = Channel::make();

//         (
//             Self {
//                 join_request: left_join_request,
//                 join_response: left_join_response,
//             },
//             Self {
//                 join_request: right_join_request,
//                 join_response: right_join_response,
//             },
//         )
//     }
// }

// pub struct Connection {
//     pub channels: ChannelList,
//     _main_loop: Task<()>,
//     _outgoing: Task<()>,
//     _incoming: Task<()>,
// }

// impl Connection {
//     pub fn send_request(
//         &self,
//         value: JoinRequest,
//     ) -> impl Future<Output = Result<(), SendError<JoinRequest>>> {
//         let outgoing = self.channels.join_request.outgoing.clone();
//         async move { outgoing.send(value).await }
//     }
// }

// #[derive(Default)]
// pub struct Connections {
//     pub connections: HashMap<SocketAddr, Connection>,
// }

// impl Connections {
//     pub fn get_or_create_connection(
//         &mut self,
//         addr: SocketAddr,
//         socket: UdpSocket,
//         executor: Arc<Executor<'static>>,
//     ) -> &Connection {
//         if let Entry::Vacant(vacant) = self.connections.entry(addr) {
//             let (left, right) = ChannelList::make();
//             let (messages, raw_in, raw_out) =
//                 build_message_channels(TurbulenceRuntime::from(executor.clone()));

//             let outgoing = executor.spawn(out_loop(raw_out, addr, socket.clone()));

//             let incoming = executor.spawn(in_loop(raw_in, addr, socket));

//             let main_loop = executor.spawn(messages_loop(right, messages));

//             vacant.insert(Connection {
//                 channels: left,
//                 _outgoing: outgoing,
//                 _incoming: incoming,
//                 _main_loop: main_loop,
//             });
//         }

//         self.connections.get(&addr).unwrap()
//     }
// }

// async fn messages_loop(channels: ChannelList, mut messages: MessageChannels) {
//     let messages = &mut messages;
//     loop {
//         // TODO make it so only the channels that are actually used appear here.
//         // that way when the other ends are all dropped this task ends up failing.
//         channels.join_request.try_forward_incoming(messages).await;
//         channels.join_response.try_forward_incoming(messages).await;

//         channels.join_request.try_forward_outgoing(messages).await;
//         channels.join_response.try_forward_outgoing(messages).await;

//         yield_now().await
//     }
// }

// #[cfg(test)]
// mod test {
//     use std::{net::ToSocketAddrs, sync::Arc};

//     use smol::{net::UdpSocket, Executor};

//     use crate::join::JoinResponse;

//     use super::Connections;

//     #[test]
//     fn integ_test_channel_map() {
//         let exec = Arc::new(Executor::new());
//         let client_addr = "127.0.0.1:10801".to_socket_addrs().unwrap().next().unwrap();
//         let server_addr = "127.0.0.1:10800".to_socket_addrs().unwrap().next().unwrap();

//         let server = {
//             let mut conn = Connections::default();
//             let socket = smol::block_on(UdpSocket::bind("127.0.0.1:10800")).unwrap();
//             conn.get_or_create_connection(client_addr, socket, exec.clone());
//             conn
//         };
//         let client = {
//             let mut conn = Connections::default();
//             let socket = smol::block_on(UdpSocket::bind("127.0.0.1:10801")).unwrap();
//             conn.get_or_create_connection(server_addr, socket, exec.clone());
//             conn
//         };

//         smol::block_on(exec.run(async move {
//             server.connections[&client_addr]
//                 .channels
//                 .join_response
//                 .outgoing
//                 .send(JoinResponse::Denied)
//                 .await
//                 .unwrap();
//             let response = client.connections[&server_addr]
//                 .channels
//                 .join_response
//                 .incoming
//                 .recv()
//                 .await
//                 .unwrap();

//             assert_eq!(response, JoinResponse::Denied);
//         }));
//     }
// }

// impl<T: ChannelMessage> Channel<T> {
//     async fn try_forward_outgoing(&self, messages: &mut MessageChannels) {
//         if let Ok(value) = self.incoming.try_recv() {
//             messages.async_send(value).await.unwrap();
//             messages.flush::<T>();
//         }
//     }

//     async fn try_forward_incoming(&self, messages: &mut MessageChannels) {
//         if let Some(value) = messages.recv::<T>() {
//             self.outgoing.send(value).await.unwrap()
//         }
//     }
// }

// type PacketType = BufferPacket<Box<[u8]>>;

// async fn out_loop(
//     mut raw_out: OutgoingMultiplexedPackets<PacketType>,
//     addr: SocketAddr,
//     socket: UdpSocket,
// ) {
//     while let Some(value) = raw_out.next().await {
//         if socket.send_to(&value, addr).await.is_err() {
//             break;
//         }
//         yield_now().await
//     }
// }

// async fn in_loop(
//     mut raw_in: IncomingMultiplexedPackets<PacketType>,
//     addr: SocketAddr,
//     socket: UdpSocket,
// ) {
//     let pool = BufferPacketPool::new(SimpleBufferPool(1024));
//     loop {
//         let mut packet = pool.acquire();
//         packet.resize(1024, 0);
//         match socket.peek_from(&mut packet).await {
//             Ok((size, from_addr)) => {
//                 if addr == from_addr {
//                     packet.truncate(size);
//                     socket.recv_from(&mut packet).await.unwrap();
//                     if raw_in.send(packet).await.is_err() {
//                         break;
//                     };
//                 }
//             }
//             Err(e) if e.kind() == ErrorKind::WouldBlock => (),
//             x => {
//                 x.unwrap();
//             }
//         }
//         yield_now().await
//     }
// }

// fn build_message_channels<R: Runtime + 'static>(
//     executor: R,
// ) -> (
//     MessageChannels,
//     IncomingMultiplexedPackets<PacketType>,
//     OutgoingMultiplexedPackets<PacketType>,
// ) {
//     let mut multiplexer = PacketMultiplexer::new();

//     let mut messages =
//         MessageChannelsBuilder::new(executor, BufferPacketPool::new(SimpleBufferPool(1024)));
//     let mut x = 0;
//     let mut new_channel = move || {
//         let val = x;
//         x += 1;
//         val
//     };
//     messages
//         .register::<JoinRequest>(MessageChannelSettings {
//             channel: new_channel(),
//             ..join::SETTINGS
//         })
//         .unwrap();
//     messages
//         .register::<JoinResponse>(MessageChannelSettings {
//             channel: new_channel(),
//             ..join::SETTINGS
//         })
//         .unwrap();

//     let messages = messages.build(&mut multiplexer);
//     let (incoming, outgoing) = multiplexer.start();

//     (messages, incoming, outgoing)
// }
