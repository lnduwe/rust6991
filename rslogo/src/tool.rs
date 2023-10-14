use std::{
    collections::{HashMap, VecDeque},
    ops::Deref,
    string,
};

use unsvg::{Image, COLORS};

use crate::LogoParser;

pub const QUOTES: char = '\"';
// pub const  tokens<&str> = vec!["AND", "OR", "GT", "LT", "EQ", "NE", "+", "-", "*", "/"];

#[derive(Debug)]
pub struct CommandError(pub String);

pub trait Tool {
    fn get_value(&self, name: &str) -> Option<f32>;
    fn prefix(&self, commands: &Vec<&str>) -> Option<f32>;
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

    fn prefix(&self, commands: &Vec<&str>) -> Option<f32> {
        let mut stack: VecDeque<String> = VecDeque::new();

        let mut result: f32 = 1.0;
        for cmd in commands.iter().rev() {
         if cmd.starts_with("[")||cmd.starts_with("WHILE")||cmd.starts_with("IF") {
                continue;
            }
            let v = self.get_value(&cmd);
            if v.is_some() || cmd.starts_with(":") {
                // let val = v.expect("Variable not found");
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
                // if let Ok(val) = cmd.parse::<f32>() {
                //     stack.push_back(val.to_string());
                //     continue;
                // } else if let Some(v) = self.variables.get(*cmd) {
                //     stack.push_back(v.to_string());
                //     continue;
                // } else {
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
    assert_eq!(parser.get_value("COLOR"), Some(8.0));
    assert_eq!(parser.get_value("UNKNOWN"), None);
}

#[test]
fn test_prefix() {
    let parser = LogoParser::new("xxx", 50, 50, None);
    // let a = parser.prefix(&vec!["1"]);

    // println!("a = {}", a.unwrap());
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
