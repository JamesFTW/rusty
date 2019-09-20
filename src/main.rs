mod configuration;
use crate::configuration::config::Config;

fn main() {
    let d = configuration::httpdconf::HttpdConf {filepath: "/desktop/files"};
    println!("{:?}", d.lookup());
}
