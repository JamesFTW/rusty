use std::io::Error;
use std::collections::HashMap;

pub trait Config {
  fn load(&mut self) -> Result<(), Error>;
  fn get_config(&self) -> &HashMap<String, String>;
}