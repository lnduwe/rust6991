use clap::{parser, Parser};
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{Lines, Read},
    path::PathBuf,
};
use unsvg::Image;

mod drawsvg;

const QUOTES: char = '\"';

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}
struct CommandError(String);

#[derive(Default)]
struct LogoParser<'a> {
    width: u32,
    height: u32,
    pen_up: bool,
    xcor: f32,
    ycor: f32,
    direction: f32,
    pen_color: f32,
    variables: HashMap<String, f32>,
    line_number: usize,
    // contents: &'a str,
    lines: Option<std::str::Lines<'a>>,
    // queue: Vec<String>,
}
impl<'a> LogoParser<'a> {
    fn new(c: &'a str) -> Self {
        LogoParser {
            width: 100,
            height: 100,
            pen_up: false,
            xcor: 0.0,
            ycor: 0.0,
            direction: 0.0,
            pen_color: 0.0,
            variables: HashMap::new(),
            line_number: 1,
            lines: Some(c.lines()),
            // contents: c,
        }
    }

    //get variables by searching in hashmap or get value from string
    fn get_value(&self, name: &str) -> Option<f32> {
        if name.starts_with(QUOTES) {
            let arg = name[1..name.len()].to_string();
            let result = arg.parse::<f32>();
            match result {
                Ok(result) => {
                    return Some(result);
                }
                Err(_) => {
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
            return Some(self.pen_color);
        } else {
            return None;
        }
    }

    //command entry
    fn parse_action(&mut self) -> Result<(), ()> {
        while let Some(line) = self.lines.as_mut().unwrap().next() {
            if line.len() == 0 {
                self.line_number += 1;
                continue;
            }

            let parts = &line.split_whitespace().collect();

            let res = self.match_action(parts);

            if res.is_err() {
                return res;
            }

            self.line_number += 1;
        }
        Ok(())
    }

    fn parse_action_with_vec(&mut self, commands: &mut VecDeque<&str>) -> Result<(), ()> {
        while let Some(line) = commands.pop_front() {
            if line.len() == 0 {
                continue;
            }

            let parts = &line.split_whitespace().collect();

            let res = self.match_action(parts);

            if res.is_err() {
                return res;
            }
        }

        Ok(())
    }

    //parse arguments and return value
    fn process_actions(&mut self, commands: &Vec<&str>) -> Result<f32, CommandError> {
        match commands[0] {
            "PENUP" | "PENDOWN" => {
                if commands.len() > 1 {
                    return Err(CommandError("Wrong number of arguments".to_string()));
                } else {
                    return Ok(0.0);
                }
            }
            "FORWARD" | "BACK" | "RIGHT" | "LEFT" | "SETPENCOLOR" | "TURN" | "SETHEADING"
            | "SETX" | "SETY" => {
                if commands.len() != 2 {
                    return Err(CommandError("Wrong number of arguments".to_string()));
                }
                match self.get_value(commands[1]) {
                    Some(result) => {
                        return Ok(result);
                    }
                    None => {
                        return Err(CommandError("Variable not found".to_string()));
                    }
                }
            }
            "MAKE" => {
                if commands.len() != 3 {
                    return Err(CommandError("Wrong number of arguments".to_string()));
                } else if !commands[1].starts_with(QUOTES)
                    || !commands[2].starts_with(QUOTES)
                    || commands[1].len() < 2
                {
                    return Err(CommandError("Wrong type of arguments".to_string()));
                } else {
                    let name = commands[1][1..commands[1].len()].to_string();

                    let v = commands[2];

                    let value = self.get_value(v);

                    match value {
                        Some(result) => {
                            self.variables.insert(name, result);
                            Ok(0.0)
                        }
                        None => {
                            return Err(CommandError("Wrong type of arguments".to_string()));
                        }
                    }
                }
            }
            "ADDASSIGN" => {
                if commands.len() != 3 {
                    return Err(CommandError("Wrong number of arguments".to_string()));
                } else if !commands[1].starts_with(QUOTES) {
                    return Err(CommandError("Wrong type of arguments".to_string()));
                } else {
                    let name = &commands[1][1..commands[1].len()].to_string();
                    let odd_value = self.variables.get(name);
                    if odd_value.is_none() {
                        return Err(CommandError("Variable not found".to_string()));
                    }
                    let name_2 = &commands[2];
                    let v = self.get_value(name_2);
                    match v {
                        Some(result) => {
                            self.variables
                                .insert(name.to_string(), odd_value.unwrap() + result);
                            Ok(0.0)
                        }
                        None => Err(CommandError("Variable not found".to_string())),
                    }
                }
            }
            "IF" => {
                if commands.len() < 5 {
                    return Err(CommandError("Wrong number of arguments".to_string()));
                }
                let mut flag = true;
                let arg = commands[1].to_string();
                if arg.eq("EQ") {
                    flag = true;
                } else if arg.eq("NE") {
                    flag = false;
                } else {
                    return Err(CommandError("Wrong type of arguments".to_string()));
                }

                if commands.len() == 5 {
                    let first = self.get_value(commands[2]);
                    let second = self.get_value(commands[3]);
                    if first.is_none()
                        || second.is_none()
                        || (first.unwrap() == second.unwrap() && !flag)
                        || (first.unwrap() != second.unwrap() && flag)
                    {
                        flag = false;
                    } else {
                        flag = true;
                    }

                    if commands[4] != "[" {
                        return Err(CommandError("Wrong type of arguments".to_string()));
                    }

                    let mut count = 0;
                    // if flag {
                    let mut cmd: VecDeque<&str> = VecDeque::new();
                    // loop {

                    // while let Some(line) = self.lines.as_mut().unwrap().next() {
                    //   if line.len() == 0 {
                    //       self.line_number += 1;
                    //       continue;
                    //   }}
                    // let a =self.lines.as_mut().unwrap().nth(1);
                    // let b = self.lines.as_mut().unwrap().nth(2);
                    // let c = self.lines.as_mut().unwrap().nth(3);
                    // let d = self.lines.as_mut().unwrap().nth(4);
                    // let f = self.lines.as_mut().unwrap().nth(5);

                    if !flag {
                        loop {
                            let line = self.lines.as_mut().unwrap().next();

                            if line.is_none() {
                                break;
                            }
                            let line = line.unwrap();
                            if line.len() == 0 {
                                // count += 1;
                                continue;
                            }

                            if !line.eq("]")  {
                                // count += 1;
                                // cmd.push_back(line);
                            } else {
                                self.line_number += 1;
                                break;
                            }
                            self.line_number += 1;
                        }
                    }
                    // match self.parse_action_with_vec(&mut cmd) {
                    //     Ok(_) => {}
                    //     Err(_) => {
                    //         return Err(CommandError("Wrong type of arguments".to_string()));
                    //     }
                    // }
                    // }

                    if !flag {
                        // self.line_number += count + 1;
                        return Ok(0.0);
                    } else {
                        return Ok(0.0);
                    }
                } else {
                    Ok(0.0)
                }
            }

            _ => {
                return Err(CommandError("Wrong type of command".to_string()));
            }
        }
    }

    fn match_action(&mut self, part: &Vec<&str>) -> Result<(), ()> {
        match part[0] {
            "//" | "]" => {}
            "PENUP" => {
                self.pen_up = true;
                match self.process_actions(&part) {
                    Ok(_) => {}
                    Err(e) => {
                        return self.print_log(&e.0);
                    }
                }
            }
            "PENDOWN" => {
                self.pen_up = false;
                match self.process_actions(&part) {
                    Ok(_) => {}
                    Err(e) => {
                        return self.print_log(&e.0);
                    }
                }
            }
            "FORWARD" => {
                self.direction = 0.0;
                match self.process_actions(&part) {
                    Ok(d) => {
                        println!("forward {}", d);
                    }
                    Err(e) => {
                        return self.print_log(&e.0);
                    }
                }
            }
            "BACK" => {
                self.direction = 180.0;
                match self.process_actions(&part) {
                    Ok(d) => {
                        println!("b {}", d);
                    }
                    Err(e) => {
                        return self.print_log(&e.0);
                    }
                }
            }
            "RIGHT" => {
                self.direction = 90.0;

                match self.process_actions(&part) {
                    Ok(d) => {
                        println!("r {}", d);
                    }
                    Err(e) => {
                        return self.print_log(&e.0);
                    }
                }
            }
            "LEFT" => {
                self.direction = 270.0;

                match self.process_actions(&part) {
                    Ok(d) => {
                        println!("left {}", d);
                    }
                    Err(e) => {
                        return self.print_log(&e.0);
                    }
                }
            }

            "SETPENCOLOR" => match self.process_actions(&part) {
                Ok(d) => {
                    self.pen_color = d;
                    println!("COLOR {}", self.pen_color);
                }
                Err(e) => {
                    return self.print_log(&e.0);
                }
            },
            "SETHEADING" => match self.process_actions(&part) {
                Ok(d) => {
                    self.direction = d;
                    println!("DIR {}", self.direction);
                }
                Err(e) => {
                    return self.print_log(&e.0);
                }
            },
            "SETX" => match self.process_actions(&part) {
                Ok(d) => self.xcor = d,
                Err(e) => {
                    return self.print_log(&e.0);
                }
            },
            "SETY" => match self.process_actions(&part) {
                Ok(d) => self.ycor = d,
                Err(e) => {
                    return self.print_log(&e.0);
                }
            },
            "TURN" => match self.process_actions(&part) {
                Ok(d) => {
                    self.direction += d;
                    self.direction = self.direction.abs() % 360.0;
                    println!("DIR {}", self.direction);
                }

                Err(e) => {
                    return self.print_log(&e.0);
                }
            },
            "MAKE" => match self.process_actions(&part) {
                Ok(d) => {}
                Err(e) => {
                    return self.print_log(&e.0);
                }
            },
            "ADDASSIGN" => match self.process_actions(&part) {
                Ok(d) => {}
                Err(e) => {
                    return self.print_log(&e.0);
                }
            },
            "IF" => match self.process_actions(&part) {
                Ok(d) => {
                    //  self.process_actions(array)
                }
                Err(e) => {
                    return self.print_log(&e.0);
                }
            },

            _ => {
                println!("Wrong type of command on line {}", self.line_number);
            }
        }
        Ok(())
    }

    fn print_log(&self, error: &str) -> Result<(), ()> {
        println!("Error: {}, line: {}", error, self.line_number);
        Err(())
    }
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;
    let image = Image::new(width, height);

    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let mut logo_parser = LogoParser::new(&contents);

    //  if let result =   logo_parser.parse_action(){
    logo_parser.parse_action();

    //  }

    // match image_path.extension().map(|s| s.to_str()).flatten() {
    //     Some("svg") => {
    //         let res = image.save_svg(&image_path);
    //         if let Err(e) = res {
    //             eprintln!("Error saving svg: {e}");
    //             return Err(());
    //         }
    //     }
    //     Some("png") => {
    //         let res = image.save_png(&image_path);
    //         if let Err(e) = res {
    //             eprintln!("Error saving png: {e}");
    //             return Err(());
    //         }
    //     }
    //     _ => {
    //         eprintln!("File extension not supported");
    //         return Err(());
    //     }
    // }

    Ok(())
}
