use crate::managers::connections::connection::Connection;
use tokio::net::TcpStream;
use crate::managers::logging::LoggingManager;
use std::net::SocketAddr;
use tokio::time::Instant;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;

impl Connection {
    pub fn new(logger: &LoggingManager, (stream, remote): (TcpStream, SocketAddr), receiver: Receiver<ConnectionProtocolMessage>, transmitter: Sender<ConnectionProtocolMessage>) -> Connection {
        logger.debug_message(format!("Incoming connection from {}", remote.ip()));
        Connection {
            remote,
            stream,
            logger: logger.clone(),
            last_ping: Instant::now(),
            receiver,
            transmitter
        }
    }
}