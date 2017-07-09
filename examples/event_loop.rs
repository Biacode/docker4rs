extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::{Future, Stream};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let listener = TcpListener::bind(
        &"127.0.0.1:8080".parse().unwrap(),
        &core.handle()
    ).unwrap();
    let handle = core.handle();
    let server = listener.incoming().for_each(|(client, _client_addr)| {
        handle.spawn(process(client));
        Ok(()) // keep accepting connections
    });
    core.run(server).unwrap();
}

// Here we'll express the handling of `client` and return it as a future
// to be spawned onto the event loop.
fn process(client: TcpStream) -> Box<Future<Item=(), Error=()>> {
    println!("Got client {:?}", client);
    tokio_io::io::write_all(client, b"Hello, World\n").then(|_| Ok(())).boxed()
}