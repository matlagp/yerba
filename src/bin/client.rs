extern crate yerba;

use yerba::yaml_parser::YAMLParser;
use yerba::client_config::ClientConfig;

fn main() {
    let cc = ClientConfig::build(
        YAMLParser::new().seed_properties("example_client_config.yaml").unwrap()
    ).unwrap();
    println!("{:?}", cc);
}
