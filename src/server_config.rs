use std::net::IpAddr;
use std::path::PathBuf;
use std::error::Error;
use crate::yaml_parser::YAMLParser;

#[derive(Debug)]
struct ServerConfig {
    listen_address: IpAddr,
    listen_port: u16,
    backups_per_client: u8,
    backup_storage_directory: PathBuf
}

impl ServerConfig {
    fn build(yaml_parser: &YAMLParser) -> Result<ServerConfig, Box<Error>> {
        Ok(ServerConfig {
            listen_address: IpAddr::V4(
                match yaml_parser.properties["listen_address"].as_str() {
                    Some(addr) => addr.parse()?,
                    None => bail!("No address")
                }
            ),
            listen_port: match yaml_parser.properties["listen_port"].as_i64() {
                Some(port) => port as u16,
                None => bail!("No port")
            },
            backups_per_client: match yaml_parser.properties["backups_per_client"].as_i64() {
                Some(bpc) => bpc as u8,
                None => bail!("No number of backups per client")
            },
            backup_storage_directory: PathBuf::from(
                match yaml_parser.properties["backup_storage_directory"].as_str() {
                    Some(path) => path,
                    None => bail!(" No backup storage directory")
                }
            )
        })
    }
}

