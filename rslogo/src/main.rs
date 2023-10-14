use clap::{parser, Parser};
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{Lines, Read},
    ops::{Deref, DerefMut},
    path::PathBuf,
    process::ExitCode,
};
use unsvg::{get_end_coordinates, Color, Image, COLORS};

mod drawsvg;
mod tool;

use tool::*;

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
    block: i32,
    xcor: f32,
    ycor: f32,
    direction: f32,
    pen_color: f32,
    variables: HashMap<String, f32>,
    line_number: usize,
    image: Option<&'a mut Image>,
    lines: Option<std::str::Lines<'a>>,
}
impl<'a> LogoParser<'a> {
    fn new(c: &'a str, w: u32, h: u32, img: Option<&'a mut Image>) -> Self {
        LogoParser {
            width: w,
            height: h,
            pen_up: true,
            block: 0,
            xcor: w as f32 / 2.0,
            ycor: h as f32 / 2.0,
            direction: 0.0,
            pen_color: 0.0,
            variables: HashMap::new(),
            line_number: 1,
            lines: Some(c.lines()),
            image: img,
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

    fn parse_action_with_vec(&mut self, commands: &VecDeque<&str>) -> Result<(), ()> {
        let mut vec = commands.clone();
        while let Some(line) = vec.pop_front() {
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

    //draw pictures and return value
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
                // if commands.len() != 2 {
                //     return Err(CommandError("Wrong number of arguments".to_string()));
                // }

                if commands.len() > 2 {
                    let mut cmd: Vec<&str> = Vec::new();
                    for i in 1..commands.len() {
                        cmd.push(commands[i]);
                    }

                    match self.prefix(&cmd) {
                        Some(result) => {
                            return Ok(result);
                        }
                        None => {
                            return Err(CommandError("Variable not found".to_string()));
                        }
                    }
                } else {
                    match self.get_value(commands[1]) {
                        Some(result) => {
                            return Ok(result);
                        }
                        None => {
                            return Err(CommandError("Variable not found".to_string()));
                        }
                    }
                }
            }
            "MAKE" => {
                // if commands.len() != 3 {
                //     return Err(CommandError("Wrong number of arguments".to_string()));
                // } else
                if !commands[1].starts_with(QUOTES) {
                    return Err(CommandError("Wrong type of arguments".to_string()));
                } else {
                    // let prefix = self.prefix(&commands[2..]);
                    let mut cmd: Vec<&str> = Vec::new();
                    for i in 2..commands.len() {
                        cmd.push(commands[i]);
                    }
                    let value = self.prefix(&cmd);

                    let name = commands[1][1..commands[1].len()].to_string();

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
                }
                // else if !commands[1].starts_with(QUOTES) {
                //     return Err(CommandError("Wrong type of arguments".to_string()));
                // }
                else {
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
                // if commands.len() < 5 {
                //     return Err(CommandError("Wrong number of arguments".to_string()));
                // }
                self.block += 1;
                let mut flag = true;
                // let arg = commands[1].to_string();
                // if arg.eq("EQ") {
                //     flag = true;
                // } else if arg.eq("NE") {
                //     flag = false;
                // } else {
                //     return Err(CommandError("Wrong type of arguments".to_string()));
                // }

                // if commands.len() == 5 {
                //     let first = self.get_value(commands[2]).expect("Null value");
                //     let second = self.get_value(commands[3]).expect("Null value");
                //     if first == second && !flag {
                //         flag = false;
                //     }
                //     if first != second && flag {
                //         flag = false;
                //     }

                //     if commands[4] != "[" {
                //         return Err(CommandError("Wrong type of arguments".to_string()));
                //     }
                // match self.prefix(commands){
                //   Some(0.0)=>{flag = false;}
                //   Some(1.0)=>{flag = true;}
                //   Some(_)=>{}
                //   None=>{return Err(CommandError("Wrong type of arguments".to_string()));}
                // }
                match self.prefix(commands) {
                    Some(r) => {
                        match r {
                            0.0 => flag = false,
                            1.0 => flag = true,
                            _ => {}
                        }

                        // if r == 0 {
                        //     flag = false;
                        // } else {
                        //     flag = true;
                        // }
                    }
                    None => {
                        return Err(CommandError("Wrong type of arguments".to_string()));
                    }
                }
                if !flag {
                    // self.block += 1;
                } else {
                    self.block -= 1;
                }
                Ok(0.0)
            }
            "WHILE" => {
                let mut flag = true;
                self.block += 1;
                match self.prefix(commands) {
                    Some(r) => match r {
                        0.0 => {
                            // self.block -= 1;
                            return Ok(0.0);
                        }
                        1.0 => {}
                        _ => {}
                    },
                    None => {
                        return Err(CommandError("Wrong type of arguments".to_string()));
                    }
                }

                let mut v: VecDeque<&str> = VecDeque::new();

                let mut semicolon = 1;
                while semicolon > 0 {
                    let line = self
                        .lines
                        .as_mut()
                        .unwrap()
                        .next()
                        .expect("Error parsing while.");
                    if line.contains("[") {
                        semicolon += 1;
                    } else if line.contains("]") {
                        semicolon -= 1;
                    }
                    v.push_back(line);
                }

                loop {
                    let _ = self.parse_action_with_vec(&v);
                    if self.block < 0 {
                        self.block = 0;
                    }
                    match self.prefix(commands) {
                        Some(r) => match r {
                            0.0 => break,

                            _ => {}
                        },
                        None => {
                            return Err(CommandError("Wrong type of arguments".to_string()));
                        }
                    }
                }
                self.block -= 1;

                Ok(0.0)
            }

            _ => {
                return Err(CommandError("Wrong type of command".to_string()));
            }
        }
    }

    fn match_action(&mut self, part: &Vec<&str>) -> Result<(), ()> {
        if part[0] == "]" {
            self.block -= 1;
            return Ok(());
        }
        if self.block > 0 {
            if part[0] == "WHILE" || part[0] == "IF" {
                self.block += 1;
            }
            return Ok(());
        }
        match part[0] {
            "//" => {}
            "PENUP" => {
                self.pen_up = true;
                match self.process_actions(&part) {
                    Ok(_) => {}
                    Err(e) => {
                        return self.log_error(&e.0);
                    }
                }
            }
            "PENDOWN" => {
                self.pen_up = false;
                match self.process_actions(&part) {
                    Ok(_) => {}
                    Err(e) => {
                        return self.log_error(&e.0);
                    }
                }
            }
            "FORWARD" => match self.process_actions(&part) {
                Ok(d) => {
                    self.draw(d, "FORWARD");
                    // self.ycor -= d * self.direction.to_radians().sin();
                    // (self.xcor, self.ycor) = img
                    //     .draw_simple_line(self.xcor , self.ycor, self.direction, 100.0, COLORS[1])
                    //     .expect("Error drawing picture");
                    println!("forward {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "BACK" => match self.process_actions(&part) {
                Ok(d) => {
                    self.draw(d, "BACK");
                    // self.ycor += d * self.direction.to_radians().sin();
                    println!("b {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "RIGHT" => match self.process_actions(&part) {
                Ok(d) => {
                    self.draw(d, "RIGHT");
                    // self.xcor += d * self.direction.to_radians().cos();
                    println!("r {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "LEFT" => match self.process_actions(&part) {
                Ok(d) => {
                    self.draw(d, "LEFT");
                    // self.xcor -= d * self.direction.to_radians().cos();
                    println!("left {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },

            "SETPENCOLOR" => match self.process_actions(&part) {
                Ok(d) => {
                    if d >= 0.0 {
                        self.pen_color = d;
                    } else {
                        return self.log_error("Wrong color");
                    }
                    println!("COLOR {}", self.pen_color);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "SETHEADING" => match self.process_actions(&part) {
                Ok(d) => {
                    self.direction = d;
                    println!("DIR {}", self.direction);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "SETX" => match self.process_actions(&part) {
                Ok(d) => self.xcor = d,
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "SETY" => match self.process_actions(&part) {
                Ok(d) => self.ycor = d,
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "TURN" => match self.process_actions(&part) {
                Ok(d) => {
                    self.direction += d;
                    println!("DIR {}", self.direction);
                }

                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "MAKE" => match self.process_actions(&part) {
                Ok(d) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "ADDASSIGN" => match self.process_actions(&part) {
                Ok(d) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "IF" => match self.process_actions(&part) {
                Ok(d) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "WHILE" => match self.process_actions(&part) {
                Ok(d) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },

            _ => {
                println!("Wrong type of command on line {}", self.line_number);
            }
        }
        Ok(())
    }

    fn log_error(&self, error: &str) -> Result<(), ()> {
        println!("Error: {}, line: {}", error, self.line_number);
        std::process::exit(1);
    }

    fn draw(&mut self, val: f32, arg: &str) {
        match arg {
            "LEFT" => {
                self.direction -= 90.0;
            }
            "RIGHT" => {
                self.direction += 90.0;
            }
            "FORWARD" => {}
            "BACK" => {
                self.direction += 180.0;
            }
            _ => {}
        }

        if self.pen_up {
            (self.xcor, self.ycor) =
                get_end_coordinates(self.xcor, self.ycor, self.direction as i32, val);
            return;
        }

        (self.xcor, self.ycor) = self
            .image
            .as_mut()
            .expect("Null image")
            .draw_simple_line(
                self.xcor,
                self.ycor,
                self.direction as i32,
                val,
                COLORS[self.pen_color as usize % 16],
            )
            .expect("Error drawing picture");
    }
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;
    let mut image = Image::new(width, height);

    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let mut logo_parser = LogoParser::new(&contents, width, height, Some(&mut image));

    //  if let result =   logo_parser.parse_action(){
    logo_parser.parse_action()?;

    //  }

    match image_path.extension().map(|s| s.to_str()).flatten() {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    }

    Ok(())
}
