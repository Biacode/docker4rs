#[derive(PartialEq, Debug, Clone)]
pub struct DockerCredentials {
    api_address: String,
    api_port: i32
}

impl DockerCredentials {
    pub fn new(docker_api_address: String, docker_api_port: i32) -> Self {
        DockerCredentials {
            api_address: docker_api_address,
            api_port: docker_api_port
        }
    }

    pub fn get_api_address(&self) -> &String {
        &self.api_address
    }

    pub fn get_api_port(&self) -> &i32 {
        &self.api_port
    }
}

#[cfg(test)]
mod trests {
    use super::{DockerCredentials};

    #[test]
    fn new_docker_credentials() {
        let api_address: String = "10.18.3.35".to_owned();
        let api_port: i32 = 2375;
        let credentials: DockerCredentials = DockerCredentials::new(api_address.clone(), api_port);
        assert_eq!(api_address, credentials.api_address);
        assert_eq!(api_port, credentials.api_port);
    }

    #[test]
    fn get_api_address_of_docker_credentials() {
        let api_address: String = "10.18.3.35".to_owned();
        let credentials: DockerCredentials = DockerCredentials::new(api_address.clone(), 7787);
        assert_eq!(&api_address, credentials.get_api_address());
    }

    #[test]
    fn get_api_port_of_docker_credentials() {
        let api_port: i32 = 7787;
        let credentials: DockerCredentials = DockerCredentials::new("10.18.3.35".to_owned(), api_port);
        assert_eq!(&api_port, credentials.get_api_port());
    }
}
