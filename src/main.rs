mod configuration;
use crate::configuration::config::Config;
use configuration::configurationreader::ConfigurationReader as ConfigReader;

fn main() {
  let mut test = ConfigReader::get_config("HTTPD_CONF");
  test.load();

  println!("{:#?}", test.get_config());
}
