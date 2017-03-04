extern crate serde_json;
extern crate hyper;

use std::io::Read;
use self::hyper::client::{Client};

use super::{DockerCredentials};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "Id")]
    id: String,

    #[serde(rename = "Names")]
    names: Vec<String>,

    #[serde(rename = "Image")]
    image: String,
}

impl Container {
    pub fn get_running_containers(docker_credentials: &DockerCredentials, client: &Client) -> Vec<Container> {
        let mut response_body = String::new();
        // TODO: make Url instead of request url builder
        let foo = &build_request_url(docker_credentials, "/containers/json")[..];
        let mut response = client.get(foo).send().unwrap();
        response.read_to_string(&mut response_body).expect("can not read response in to response body");
        return serde_json::from_str(&response_body).unwrap();
    }
}

fn build_request_url(docker_credentials: &DockerCredentials, path: &str) -> String {
    let mut base_url = base_url(
        &docker_credentials.get_api_address(),
        &docker_credentials.get_api_port().to_string()
    );
    base_url.push_str(path);
    base_url
}

fn base_url(base: &str, port: &str) -> String {
    let ref base_url_ref = base.to_owned();
    let mut base_url = base_url_ref.clone();
    base_url.push_str(":");
    base_url.push_str(port);
    base_url
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_running_containers() {}
}