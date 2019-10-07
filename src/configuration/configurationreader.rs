
use super::httpdconf::HttpdConf as HttpdConf;
use super::mimetypes::MimeTypeConf as MimeTypeConf;
use super::config::ConfigType as ConfigType;

#[derive(Debug)]
pub struct ConfigurationReader;

impl ConfigurationReader {
  pub fn config(str: &str) -> ConfigType {
    match str.as_ref() {
      "HTTPD_CONF" => ConfigType::HttpdConf(HttpdConf::new("./conf/httpd.conf")),
      "MIME_TYPE" => ConfigType::MimeTypeConf(MimeTypeConf::new("./conf/mime.types")),
      &_ => unimplemented!()
    }
  }
}

