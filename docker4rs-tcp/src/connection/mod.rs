use std::io::ErrorKind;

use docker4rs_core::connection::{Connectable, Connection, ConnectionConfig};

/// Holds the success connection
pub struct TcpConnection {}

/// tcp connection configuration
pub struct TcpConnectionConfig {}

/// connects to the docker tcp socket
pub struct TcpConnector {}

impl ConnectionConfig for TcpConnectionConfig {}

impl Connection for TcpConnection {}

impl Connectable<TcpConnection, TcpConnectionConfig> for TcpConnector {
    fn connect(config: &TcpConnectionConfig) -> Result<TcpConnection, ErrorKind> {
        unimplemented!()
    }
}