// use std::{net::SocketAddr, time::Duration};

// use serde::{Deserialize, Serialize};
// use turbulence::{reliable_channel::Settings, MessageChannelMode, MessageChannelSettings};

// use crate::PlayerId;
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// pub struct JoinRequest {
//     pub addr: PlayerId,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// pub enum JoinResponse {
//     Denied,
//     Accepted { self_addr: SocketAddr },
// }

// pub const SETTINGS: MessageChannelSettings = MessageChannelSettings {
//     channel: 0,
//     channel_mode: MessageChannelMode::Reliable {
//         max_message_len: 1024,
//         reliability_settings: Settings {
//             bandwidth: 4096,
//             recv_window_size: 1024,
//             send_window_size: 1024,
//             burst_bandwidth: 1024,
//             init_send: 512,
//             wakeup_time: Duration::from_millis(100),
//             initial_rtt: Duration::from_millis(200),
//             max_rtt: Duration::from_secs(2),
//             rtt_update_factor: 0.1,
//             rtt_resend_factor: 1.5,
//         },
//     },
//     message_buffer_size: 4,
//     packet_buffer_size: 4,
// };
