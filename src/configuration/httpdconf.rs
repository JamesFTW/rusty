
use super::config;

#[derive(Debug)]
pub struct HttpdConf {
  pub filepath: &'static str
}

impl config::Config for HttpdConf {
  fn load(&self) {
    println!("hdhd");
  }
  fn lookup(&self) -> String {
    return "Test".to_string();
  }
}
