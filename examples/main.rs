extern crate hyper;
extern crate docker4rs;

use docker4rs::docker::Container;
use docker4rs::docker::DockerCredentials;

use std::sync::Arc;
use hyper::Client;

fn main() {
    let client = Arc::new(Client::new());
    let containers = Container::get_running_containers(
        &DockerCredentials::new("http://localhost".to_string(), 2375),
        &client.clone()
    );
    println!("The running containers {:?}", containers);
}