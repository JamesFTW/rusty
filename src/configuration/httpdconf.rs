
use super::config::Config as Config;
use crate::configuration::util::trim as trim;

use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufRead, Error};

#[derive(Debug)]
pub struct HttpdConf {
  pub filepath: &'static str,
  pub config_map: HashMap<String, String>,
}

impl HttpdConf {
  pub fn new(path: &'static str) -> HttpdConf {
    HttpdConf {
      filepath: path,
      config_map: HashMap::new()
    }
  }
}

impl Config for HttpdConf {
  fn load(&mut self) -> Result<(), Error> {
    let input = fs::File::open(self.filepath)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
      let ll = line?;
      let tokens: Vec<&str> = ll.split(" ").collect();

      if tokens[0].to_string() == "ScriptAlias" {
        let string = trim(tokens[2].to_string());
        self.config_map.insert(tokens[1].to_string(), string);

      } else if tokens[0].to_string() == "Alias" {
        let string = trim(tokens[2].to_string());
        self.config_map.insert(tokens[1].to_string(), string);

      } else {
        let char_vec: Vec<char> = tokens[1].chars().collect();
        let first_el = char_vec[0];

        if first_el == '\"' {
          let string = trim(tokens[1].to_string());
          self.config_map.insert(tokens[0].to_string(), string);

        } else {
          self.config_map.insert(tokens[0].to_string(), tokens[1].to_string());
        }
      }
    }
    Ok(())
  }

  fn get_config(&self) -> &HashMap<String, String> {
    return &self.config_map;
  }
}