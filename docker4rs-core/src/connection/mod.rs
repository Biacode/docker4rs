use std::io::ErrorKind;

pub trait Connectable<T: Connection, C: ConnectionConfig> {
    fn connect(config: &C) -> Result<T, ErrorKind>;
}

pub trait Connection {}

pub trait ConnectionConfig {}