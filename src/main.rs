mod configuration;
use crate::configuration::config::Config;

use configuration::httpdconf as CONFIG;

fn main() {
  let config = CONFIG::HttpdConf {filepath: "/desktop/files"};
  println!("{:?}", config.lookup());
}
