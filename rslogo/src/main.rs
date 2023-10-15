use clap::Parser;
use std::{collections::HashMap, fs::File, io::Read};
use unsvg::{get_end_coordinates, Image, COLORS};

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
struct Procedure {
    commands: Vec<String>,
    args: Vec<String>,
}
#[derive(Default)]
struct LogoParser<'a> {
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
    procedures: HashMap<String, Procedure>,
}
impl<'a> LogoParser<'a> {
    fn new(c: &'a str, w: u32, h: u32, img: Option<&'a mut Image>) -> Self {
        LogoParser {
            pen_up: true,
            block: 0,
            xcor: w as f32 / 2.0,
            ycor: h as f32 / 2.0,
            direction: 0.0,
            pen_color: 7.0,
            variables: HashMap::new(),
            procedures: HashMap::new(),
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

            // if self.block > 0 {
            //     if line.contains("[") {
            //         self.block += 1;
            //         continue;
            //     }
            //     if line.contains("]") {
            //         self.block -= 1;
            //     }
            //     continue;
            // }

            let parts = &line.split_whitespace().collect();

            let res = self.match_action (parts,None);

            if res.is_err() {
                return res;
            }

            self.line_number += 1;
        }
        Ok(())
    }

    fn parse_action_with_vec<T: AsRef<str>>(&mut self, commands: &Vec<T>) -> Result<(), ()> {
        for i in 0..commands.len() {
            let line = commands[i].as_ref();
            if line.len() == 0 {
                continue;
            }
            let parts = &line.split_whitespace().collect();

            let res = self.match_action(parts,None);

            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }

    //draw pictures and return value
    fn process_actions(&mut self, parts: &Vec<&str>) -> Result<f32, CommandError> {
        match parts[0] {
            "PENUP" | "PENDOWN" => {
                if parts.len() > 1 {
                    return Err(CommandError("Wrong number of arguments".to_string()));
                } else {
                    return Ok(0.0);
                }
            }
            "FORWARD" | "BACK" | "RIGHT" | "LEFT" | "SETPENCOLOR" | "TURN" | "SETHEADING"
            | "SETX" | "SETY" => {
                if parts.len() > 2 {
                    let mut cmd: Vec<&str> = Vec::new();
                    for i in 1..parts.len() {
                        cmd.push(parts[i]);
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
                    match self.get_value(parts[1]) {
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
                if !parts[1].starts_with(QUOTES) {
                    return Err(CommandError("Wrong type of arguments".to_string()));
                } else {
                    let mut cmd: Vec<&str> = Vec::new();
                    for i in 2..parts.len() {
                        cmd.push(parts[i]);
                    }
                    let value = self.prefix(&cmd);

                    let name = parts[1][1..parts[1].len()].to_string();

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
                if parts.len() != 3 {
                    return Err(CommandError("Wrong number of arguments".to_string()));
                } else {
                    let name = &parts[1][1..parts[1].len()].to_string();
                    let odd_value = self.variables.get(name);
                    if odd_value.is_none() {
                        return Err(CommandError("Variable not found".to_string()));
                    }
                    let name_2 = &parts[2];
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
                self.block += 1;

                let res = self.prefix(parts);
                if res.is_none() {
                    return Err(CommandError("Wrong type of arguments".to_string()));
                }
                let r = res.unwrap();
                if r == 1.0 {
                    self.block -= 1;
                }
                Ok(0.0)
            }
            "WHILE" => {
                self.block += 1;

                let res = self.prefix(parts);
                if res.is_none() {
                    return Err(CommandError("Wrong type of arguments".to_string()));
                }
                let r = res.unwrap();
                if r == 0.0 {
                    return Ok(0.0);
                }

                let mut v: Vec<&str> = Vec::new();

                let mut semicolon = 1;
                while semicolon > 0 {
                    let line = self
                        .lines
                        .as_mut()
                        .unwrap()
                        .next();
                    if line.is_none() {
                        return Err(CommandError("Error parsing error".to_string()));
                    }
                    let line = line.unwrap();
                    if line.contains("[") {
                        semicolon += 1;
                    } else if line.contains("]") {
                        semicolon -= 1;
                    }
                    v.push(line);
                }

                loop {
                    let _ = self.parse_action_with_vec(&v);
                    if self.block < 0 {
                        self.block = 0;
                    }
                    let res = self.prefix(parts);
                    if res.is_none() {
                        return Err(CommandError("Wrong type of arguments".to_string()));
                    }
                    let r = res.unwrap();
                    if r == 0.0 {
                        break;
                    }
                }
                self.block -= 1;

                Ok(0.0)
            }
            "TO" => {
                let mut args: Vec<String> = Vec::new();
                for i in 2..parts.len() {
                    args.push(parts[i][1..].to_string());
                }
                let mut cmd: Vec<String> = Vec::new();

                loop {
                    let line = self
                        .lines
                        .as_mut()
                        .unwrap()
                        .next()
                        .expect("Error parsing To.");
                    if line.contains("END") {
                        break;
                    }
                    cmd.push(line.to_string());
                }

                let pros = Procedure {
                    commands: cmd,
                    args: args,
                };

                self.procedures.insert(parts[1].to_string(), pros);

                Ok(0.0)
            }

            _ => {
                return Err(CommandError("Wrong type of command".to_string()));
            }
        }
    }

    fn match_action(&mut self, parts: &Vec<&str>,sequence:Option<&Vec<&str>>) -> Result<(), ()> {
        if parts[0] == "]" {
            if self.block > 0 {
                self.block -= 1;
            }
            return Ok(());
        }
        if self.block > 0 {
            if parts[0] == "WHILE" || parts[0] == "IF" {
                self.block += 1;
            }
            return Ok(());
        }
        match parts[0] {
            "//" => {}
            "PENUP" => {
                self.pen_up = true;
                match self.process_actions(&parts) {
                    Ok(_) => {}
                    Err(e) => {
                        return self.log_error(&e.0);
                    }
                }
            }
            "PENDOWN" => {
                self.pen_up = false;
                match self.process_actions(&parts) {
                    Ok(_) => {}
                    Err(e) => {
                        return self.log_error(&e.0);
                    }
                }
            }
            "FORWARD" => match self.process_actions(&parts) {
                Ok(d) => {
                    self.draw(d, "FORWARD");
                    println!("forward {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "BACK" => match self.process_actions(&parts) {
                Ok(d) => {
                    self.draw(d, "BACK");
                    println!("b {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "RIGHT" => match self.process_actions(&parts) {
                Ok(d) => {
                    self.draw(d, "RIGHT");
                    println!("r {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "LEFT" => match self.process_actions(&parts) {
                Ok(d) => {
                    self.draw(d, "LEFT");
                    println!("left {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },

            "SETPENCOLOR" => match self.process_actions(&parts) {
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
            "SETHEADING" => match self.process_actions(&parts) {
                Ok(d) => {
                    self.direction = d;
                    println!("DIR {}", self.direction);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "SETX" => match self.process_actions(&parts) {
                Ok(d) => self.xcor = d,
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "SETY" => match self.process_actions(&parts) {
                Ok(d) => self.ycor = d,
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "TURN" => match self.process_actions(&parts) {
                Ok(d) => {
                    self.direction += d;
                    println!("DIR {}", self.direction);
                }

                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "MAKE" => match self.process_actions(&parts) {
                Ok(_) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "ADDASSIGN" => match self.process_actions(&parts) {
                Ok(_) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "IF" => match self.process_actions(&parts) {
                Ok(_) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "WHILE" => match self.process_actions(&parts) {
                Ok(_) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "TO" => match self.process_actions(&parts) {
                Ok(_) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },

            _ => {
                let pro = self.procedures.get(parts[0]);
                if pro.is_none() {
                    let _ = self.log_error("No procedures found");
                } else {
                    let proced = pro.unwrap();
                    let arg = proced.args.clone();

                    let len = arg.len();
                    // if len != parts.len() - 1 {
                    //     let _ = self.log_error("Wrong number of arguments");
                    // }
                    let mut part: Vec<&str> = Vec::new();
                    for i in 1..parts.len() {
                        part.push(parts[i]);
                    }
                    for i in 0..len {
                        match self.prefix(&part) {
                            Some(result) => {
                                self.variables.insert(arg[i].to_string(), result);
                            }
                            None => {
                                let _ = self.log_error("Error parsing procedure");
                            }
                        }
                    }
                    let cmd = proced.commands.clone();
                    self.parse_action_with_vec(&cmd)?;

                    for i in 0..len {
                        // match self.get_value(parts[i + 1]) {
                        //     Some(_) => {
                        self.variables.remove(arg[i].as_str());
                        // }
                        // None => {
                        //     let _ = self.log_error("Error parsing procedure");
                        // }
                        // }
                    }
                }
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

    logo_parser.parse_action()?;

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
