
use super::config::Config as Config;
use crate::configuration::util::trim as trim;

use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufRead, Error};

pub struct MimeTypeConf {
  pub filepath: &'static str,
  pub config_map: HashMap<String, String>,
}

impl MimeTypeConf {
  pub fn new(path: &'static str) -> MimeTypeConf {
    MimeTypeConf {
      filepath: path,
      config_map: HashMap::new()
    }
  }
}

impl Config for MimeTypeConf {
  fn load(&mut self) -> Result<(), Error> {
    let input = fs::File::open(self.filepath)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
      let ll = line?;
      let mut tokens: Vec<&str> = ll.split_ascii_whitespace().collect();

      if !tokens.is_empty() {
        if tokens[0] != "#" && tokens.len() > 1 {
          let val = tokens[0];

          while !tokens.is_empty() {
            let bruh = tokens[tokens.len() - 1];

            self.config_map.insert(bruh.to_string(), val.to_string());
            tokens.pop();
          }
        }
      }
    }
    Ok(())
  }

  fn get_config(&self) -> &HashMap<String, String> {
    return &self.config_map;
  }
}