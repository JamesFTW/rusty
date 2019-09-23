use std::io::Error;
use std::collections::HashMap;

use super::httpdconf::HttpdConf as HttpdConf;
use super::mimetypes::MimeTypeConf as MimeTypeConf;

pub trait Config {
  fn load(&mut self) -> Result<(), Error>;
  fn get_config(&self) -> &HashMap<String, String>;
}

#[derive(Debug)]
pub enum ConfigType {
  HttpdConf(HttpdConf),
  MimeTypeConf(MimeTypeConf),
}

impl Config for ConfigType {
  fn load(&mut self) -> Result<(), Error> {
    match *self {
      ConfigType::HttpdConf(ref mut http) => http.load(),
      ConfigType::MimeTypeConf(ref mut mime) => mime.load(),
    }
  }
  fn get_config(&self)-> &HashMap<String, String> {
    match *self {
      ConfigType::HttpdConf(ref http) => http.get_config(),
      ConfigType::MimeTypeConf(ref mime) => mime.get_config(),
    }
  }
}