extern crate futures;
extern crate tokio_core;

use futures::Stream;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let listener = TcpListener::bind(
        &"127.0.0.1:8080".parse().unwrap(),
        &core.handle()
    ).unwrap();

    let server = listener.incoming().for_each(|(client, client_addr)| {
        // process `client` by spawning a new task ...

        Ok(()) // keep accepting connections
    });

    core.run(server).unwrap();
}