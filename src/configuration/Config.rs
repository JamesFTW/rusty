pub trait Config {
  fn load(&self);
  fn lookup(&self) -> String;
}