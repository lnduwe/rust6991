use std::{collections::{VecDeque, HashMap}, ops::Deref, string};

use unsvg::{Image, COLORS};

use crate::LogoParser;

pub const QUOTES: char = '\"';
pub struct CommandError(pub String);

pub trait Tool {
    fn get_value(&self, name: &str) -> Option<f32>;
    fn prefix(&self, commands: &Vec<&str>) -> Result<f32, CommandError> ;
}


impl Tool for LogoParser<'_> {
  fn get_value(&self, name: &str) -> Option<f32> {
      if name.starts_with(QUOTES) {
          let arg = name[1..name.len()].to_string();
          let result = arg.parse::<f32>();
          match result {
              Ok(result) => {
                  return Some(result);
              }
              Err(_) => {
                  if arg == "TRUE" {
                      return Some(1.0);
                  } else if arg == "FALSE" {
                      return Some(0.0);
                  }
                  return None;
              }
          }
      } else if name.starts_with(":") {
          let arg = name[1..name.len()].to_string();

          match self.variables.get(&arg) {
              Some(result) => {
                  return Some(*result);
              }
              None => {
                  return None;
              }
          }
      } else if name.eq("XCOR") {
          return Some(self.xcor);
      } else if name.eq("YCOR") {
          return Some(self.ycor);
      } else if name.eq("HEADING") {
          return Some(self.direction);
      } else if name.eq("COLOR") {
          return Some(self.pen_color as f32);
      } else {
          return None;
      }
  }

  fn prefix(&self,commands: &Vec<&str>) -> Result<f32, CommandError> {
    let mut stack: VecDeque<String> = VecDeque::new();

    for cmd in commands.iter().rev() {
        if let Ok(val) = cmd.parse::<f32>() {
            stack.push_back(val.to_string());
            continue;
        } else if let Some(v) = self.variables.get(*cmd) {
            stack.push_back(v.to_string());
            continue;
        }else {
            let f = stack.pop_back().expect("pop error");
            let s = stack.pop_back().expect("pop error");
            let first = f.parse::<f32>().expect("parse float error");
            let second = s.parse::<f32>().expect("parse float error");
            let res = match *cmd {

                "AND" => {
                    if first != 0.0 && second != 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                }
                "OR" => {
                    if first != 0.0 || second != 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                }
                "GT" => {
                    if first > second {
                        1.0
                    } else {
                        0.0
                    }
                }
                "LT" => {
                    if first < second {
                        1.0
                    } else {
                        0.0
                    }
                }
                "+" => first + second,
                "-" => first - second,
                "*" => first * second,
                "/" => first / second,
                _ => 0.0,
            };
            stack.push_back(res.to_string());
        }
    }

    Ok(0.0)
}
}