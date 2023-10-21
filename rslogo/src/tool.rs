use std::collections::VecDeque;

use crate::LogoParser;

pub const QUOTES: char = '\"';

#[derive(Debug)]
/// An error that occurs when a command fails to execute.
pub struct CommandError(pub String);

pub trait Tool {
    fn get_value(&self, name: &str) -> Option<f32>;
    fn prefix(&self, commands: &[&str]) -> Option<f32>;

    fn quantize(x: f32) -> f32 {
        (x * 256.0).round() / 256.0
    }
    
    /// Calculates the end coordinates of a line segment given its starting point, direction and length.
    /// 
    /// # Arguments
    /// 
    /// * `x` - The x-coordinate of the starting point of the line segment.
    /// * `y` - The y-coordinate of the starting point of the line segment.
    /// * `direction` - The direction of the line segment in degrees. 0 degrees is straight up and angles increase clockwise.
    /// * `length` - The length of the line segment.
    /// 
    /// # Returns
    /// 
    /// A tuple containing the x and y coordinates of the end point of the line segment.
    fn get_end_coordinates(x: f32, y: f32, direction: i32, length: f32) -> (f32, f32) {
      let x = Self::quantize(x);
      let y = Self::quantize(y);

      // directions start at 0 degrees being straight up, and go clockwise
      // we need to add 90 degrees to make 0 degrees straight right.
      let direction_rad = ((direction as f32) - 90.0).to_radians();

      let end_x = Self::quantize(x + (direction_rad.cos() * length));
      let end_y = Self::quantize(y + (direction_rad.sin() * length));

      (end_x, end_y)
    }
}

impl Tool for LogoParser<'_> {
    /// Returns the value of a given variable or constant.
    /// 
    /// # Arguments
    /// 
    /// * `name` - A string slice that holds the name of the variable or constant.
    /// 
    /// # Returns
    /// 
    /// * `Some(f32)` - The value of the variable or constant if it exists and is a float.
    /// * `Some(1.0)` - If the name is a quoted string "TRUE".
    /// * `Some(0.0)` - If the name is a quoted string "FALSE".
    /// * `None` - If the name is not a valid variable or constant.
    fn get_value(&self, name: &str) -> Option<f32> {
      if name.starts_with(QUOTES) {
        let arg = name[1..name.len()].to_string();
        let result = arg.parse::<f32>();
        match result {
          Ok(result) => Some(result),
          Err(_) => {
            if arg == "TRUE" {
              return Some(1.0);
            } else if arg == "FALSE" {
              return Some(0.0);
            }
            None
          }
        }
      } else if name.starts_with(':') {
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
        return Some(self.pen_color);
      } else {
        return None;
      }
    }

    /// Evaluates a list of commands in prefix notation and returns the result as a float.
    /// 
    /// # Arguments
    ///
    /// * `commands` - A slice of string references representing the commands to be evaluated.
    ///
    /// # Returns
    ///
    /// An `Option<f32>` representing the result of the evaluation. If the evaluation is successful, 
    /// the result is wrapped in `Some`. If there is an error during evaluation, `None` is returned.
    ///
    fn prefix(&self, commands: &[&str]) -> Option<f32> {
        let mut stack: VecDeque<String> = VecDeque::new();

        let mut result: f32 = 1.0;
        for cmd in commands.iter().rev() {
            if cmd.starts_with('[') || cmd.starts_with("WHILE") || cmd.starts_with("IF") {
                continue;
            }
            let v = self.get_value(cmd);
            if v.is_some() || cmd.starts_with(':') {
                match v {
                    Some(val) => {
                        result = val;
                        stack.push_back(val.to_string());
                        continue;
                    }
                    None => {
                        return None;
                    }
                }
            } else {
                if stack.len() < 2 {
                    return None;
                }
                let f = stack.pop_back().expect("pop error");
                let s = stack.pop_back().expect("pop error");
                let first = f.parse::<f32>().expect("parse float error");
                let second = s.parse::<f32>().expect("parse float error");
                match *cmd {
                    "AND" => {
                        result = if first != 0.0 && second != 0.0 {
                            1.0
                        } else {
                            0.0
                        };
                    }
                    "OR" => {
                        result = if first != 0.0 || second != 0.0 {
                            1.0
                        } else {
                            0.0
                        };
                    }
                    "GT" => {
                        result = if first > second { 1.0 } else { 0.0 };
                    }
                    "LT" => {
                        result = if first < second { 1.0 } else { 0.0 };
                    }
                    "EQ" => {
                        result = if first == second { 1.0 } else { 0.0 };
                    }
                    "NE" => {
                        result = if first != second { 1.0 } else { 0.0 };
                    }
                    "+" => result = first + second,
                    "-" => result = first - second,
                    "*" => result = first * second,
                    "/" => result = first / second,
                    "IF" | "WHILE" => return Some(result),
                    _ => return None,
                };
                stack.push_back(result.to_string());
            }
        }
        if stack.len() != 1 {
            return None;
        }
        Some(result)
    }
}

#[test]
fn test_get_value() {
    let parser = LogoParser::new("xxx", 50, 50, None);
    assert_eq!(parser.get_value("\"3.14"), Some(3.14));
    assert_eq!(parser.get_value("\"TRUE"), Some(1.0));
    assert_eq!(parser.get_value("\"FALSE"), Some(0.0));
    assert_eq!(parser.get_value("XCOR"), Some(25.0));
    assert_eq!(parser.get_value("YCOR"), Some(25.0));
    assert_eq!(parser.get_value("HEADING"), Some(0.0));
    assert_eq!(parser.get_value("COLOR"), Some(7.0));
    assert_eq!(parser.get_value("UNKNOWN"), None);
}

#[test]
fn test_prefix() {
    let parser = LogoParser::new("xxx", 50, 50, None);
    assert_eq!(parser.prefix(&vec!["\"1"]), Some(1.0));
    assert_eq!(parser.prefix(&vec!["\"3.14"]), Some(3.14));
    assert_eq!(parser.prefix(&vec![":foo"]), None);
    assert_eq!(parser.prefix(&vec!["AND", "\"1", "\"0"]), Some(0.0));
    assert_eq!(parser.prefix(&vec!["OR", "\"0", "\"1"]), Some(1.0));
    assert_eq!(parser.prefix(&vec!["GT", "\"1", "\"2"]), Some(0.0));
    assert_eq!(parser.prefix(&vec!["UNKNOWN", "\"2", "\"1"]), None);
    assert_eq!(parser.prefix(&vec!["OR", "AND", "\"1", "\"2"]), None);
    assert_eq!(
        parser.prefix(&vec!["EQ", "\"7", "+", "\"2", "\"5"]),
        Some(1.0)
    );
}
