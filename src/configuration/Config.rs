use std::io::Error;

pub trait Config {
  fn load(&self, path: &'static str) -> Result<(), Error>;
  fn lookup(&self) -> String;
}