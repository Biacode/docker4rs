extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

fn main() {
    let mut core = Core::new().unwrap();
    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address, &core.handle()).unwrap();
    let connections = listener.incoming();
    let handle = core.handle().clone();
    let server = connections.for_each(|(socket, _peer_addr)| {
        use futures::Future;
        let serve_one = tokio_io::io::write_all(socket, b"Hello, world!\n").then(|_| Ok(()));
        handle.spawn(serve_one);
        Ok(())
    });

    core.run(server).unwrap();
}