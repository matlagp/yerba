use std::net::IpAddr;
use std::path::PathBuf;
use std::error::Error;
use crate::yaml_parser::YAMLParser;

#[derive(Debug)]
pub struct ClientConfig {
    server_address: IpAddr,
    server_port: u16,
    directory_to_backup: PathBuf,
    cookie: String
}

impl ClientConfig {
    pub fn build(yaml_parser: &YAMLParser) -> Result<ClientConfig, Box<Error>> {
        Ok(ClientConfig {
            server_address: IpAddr::V4(
                match yaml_parser.properties["server_address"].as_str() {
                    Some(addr) => addr.parse()?,
                    None => bail!("No address")
                }
            ),
            server_port: match yaml_parser.properties["server_port"].as_i64() {
                Some(port) => port as u16,
                None => bail!("No port")
            },
            directory_to_backup: PathBuf::from(
                match yaml_parser.properties["directory_to_backup"].as_str() {
                    Some(dir) => dir,
                    None => bail!("No directory to backup")
                }
            ),
            cookie: match yaml_parser.properties["cookie"].as_str() {
                Some(cookie) => cookie.to_owned(),
                None => bail!("No cookie")
            }
        })
    }
}

