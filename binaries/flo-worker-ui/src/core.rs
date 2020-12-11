use structopt::StructOpt;
use std::path::PathBuf;

use serde::{ Deserialize, Serialize };

pub const JSON_CONF_FNAME: &str = "flo-config.json";

#[derive(Debug, StructOpt, Clone, Serialize, Deserialize)]
#[structopt(name = "flo-worker", about = "Flo worker process.")]
pub struct Opt {
  #[structopt(long)]
  pub debug: bool,

  #[structopt(long)]
  pub token: Option<String>,

  #[structopt(long, parse(from_os_str))]
  pub installation_path: Option<PathBuf>,

  #[structopt(long)]
  pub controller_host: Option<String>,
}

impl Default for Opt {
  fn default() -> Self {
    Self {
      debug: false,
      token: None,
      installation_path: None,
      controller_host: None
    }
  }
}
