use clap::{App, Arg};


#[derive(Debug)]
pub struct Config {
    pub file: Vec<String>,
    pub lines: usize,
    pub bytes: Option<usize>,
}

pub fn get_args() -> Result<Config, Box<dyn std::error::Error>> {
  let matches = App::new("headr")
      .version("0.1.0")
      .author("Your Name")
      .about("A simple implementation of the head command")
      .arg(Arg::new("file")
          .about("The file to read")
          .required(true)
          .index(1))
      .arg(Arg::new("lines")
          .about("Number of lines to read")
          .short('n')
          .long("lines")
          .takes_value(true)
          .default_value("10"))
      .arg(Arg::new("bytes")
          .about("Number of bytes to read")
          .short('c')
          .long("bytes")
          .takes_value(true))
      .get_matches();

    Ok(Config {
      file: matches.values_of("file").unwrap().map(|s| s.to_string()).collect(),
      lines: matches.value_of("lines").unwrap().parse()?,
      bytes: matches.value_of("bytes").map(|v| v.parse().unwrap()),
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

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);
    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
