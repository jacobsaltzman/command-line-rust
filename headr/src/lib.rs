use clap::{App, Arg};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
  pub file: Vec<String>,
  pub lines: usize,
  pub bytes: Option<usize>,
}

pub fn get_args() -> MyResults<Config>{
  let matches = App::new("headr")
    .version("0.1.0")
    .author("Your Name")
    .about("A simple implementation of the head command");
  Ok(Config{
    file: matches.value_of("file").unwrap().to_string(),
    lines: matches.value_of("lines").unwrap().parse().unwrap(),
    bytes: matches.value_of("bytes").unwrap().parse().unwrap(),
  })
    
}

fn parse_positive_int(val: &str) -> Result<(), String> {
  match val.parse::<usize>() {
    Ok(n) => {
      if n > 0 {
        Ok(())
      } else {
        Err("Value must be positive".to_string())
      }
    }
    Err(_) => Err("Value must be a number".to_string()),
  }
}