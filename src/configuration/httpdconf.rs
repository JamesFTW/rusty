
use super::config::Config as Config;

use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufRead, Error};

pub struct HttpdConf {
  pub filepath: &'static str,
}

impl Config for HttpdConf {
  fn load(&self, path: &'static str) -> Result<(), Error> {
    let mut config_map: HashMap<String, String> = HashMap::new();

    /**
     * Going to use a single map for all config.
     * {
     *  httpdConf: 'baz',
     *  script_Alias: {
     *    'foo': 'bar'
     *  }
     * }
     */
    let input = fs::File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
      let ll = line?;
      let tokens: Vec<&str> = ll.split(" ").collect();

      config_map.insert(tokens[0].to_string(), tokens[1].to_string());
    }

    print!("{:#?}",config_map);
    
    Ok(())
  }

  fn lookup(&self) -> String {
    return "Test".to_string();
  }
}
