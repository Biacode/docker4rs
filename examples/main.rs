extern crate hyper;
extern crate docker4rs;

use docker4rs::docker::Container;
use docker4rs::docker::DockerCredentials;

use std::sync::Arc;
use hyper::Client;

const DOCKER_API_ADDRESS: &'static str = "http://localhost";
const DOCKER_API_PORT: &'static i32 = 2375;

fn main() {
    let client = Arc::new(Client::new());
    let containers = Container::get_running_containers(
        &DockerCredentials::new(DOCKER_API_ADDRESS.to_string(), DOCKER_API_PORT),
        &client.clone()
    );
    println!("The running containers {:?}", containers);
}