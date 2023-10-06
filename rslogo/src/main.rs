use std::{collections::HashMap, fs::File, io::Read, num::ParseFloatError, result};

use clap::{parser, Parser};
use unsvg::Image;

const QUOTES: char = '\"';

enum Commands {
    PENUP,
    PENDOWN,
    FORWARD(f32),
    BACK(f32),
    RIGHT(f32),
    LEFT(f32),
    SETPENCOLOR(f32),
    TURN(f32),
    SETHEADING(f32),
    SETX(f32),
    SETY(f32),
}

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
struct LogoParser {
    file_path: String,
    width: u32,
    height: u32,
    pen_up: bool,
    xcor: f32,
    ycor: f32,
    direction: f32,
    pen_color: f32,
    variables: HashMap<String, f32>,
}
impl LogoParser {
    fn new(file_path: String) -> Self {
        LogoParser {
            file_path,
            width: 100,
            height: 100,
            pen_up: false,
            xcor: 0.0,
            ycor: 0.0,
            direction: 0.0,
            pen_color: 0.0,
            variables: HashMap::new(),
        }
    }

    fn ParseAction(&mut self) {
        let mut file = File::open(&self.file_path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");
        let lines = contents.lines();
        let mut line_number = 1;
        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // println!("{:?}", &parts);

            match parts[0] {
                "//" => {}
                "PENUP" => {
                    self.pen_up = true;
                    match self.process_actions(&parts) {
                        Ok(_) => {}
                        Err(e) => self.print_log(line_number, &e.0),
                    }
                }
                "PENDOWN" => {
                    self.pen_up = false;
                    match self.process_actions(&parts) {
                        Ok(_) => {}
                        Err(e) => self.print_log(line_number, &e.0),
                    }
                }
                "FORWARD" => {
                    self.direction = 0.0;
                    match self.process_actions(&parts) {
                        Ok(d) => {
                            println!("{}", d);
                        }
                        Err(e) => self.print_log(line_number, &e.0),
                    }
                }
                "BACK" => {
                    self.direction = 180.0;
                    match self.process_actions(&parts) {
                        Ok(d) => {
                            println!("{}", d);
                        }
                        Err(e) => self.print_log(line_number, &e.0),
                    }
                }
                "RIGHT" => {
                    self.direction = 90.0;

                    match self.process_actions(&parts) {
                        Ok(d) => {
                            println!("{}", d);
                        }
                        Err(e) => self.print_log(line_number, &e.0),
                    }
                }
                "LEFT" => {
                    self.direction = 270.0;

                    match self.process_actions(&parts) {
                        Ok(d) => {
                            println!("{}", d);
                        }
                        Err(e) => self.print_log(line_number, &e.0),
                    }
                }

                "SETPENCOLOR" => match self.process_actions(&parts) {
                    Ok(d) => self.pen_color = d,
                    Err(e) => self.print_log(line_number, &e.0),
                },
                "SETHEADING" => match self.process_actions(&parts) {
                    Ok(d) => self.direction = d,
                    Err(e) => self.print_log(line_number, &e.0),
                },
                "SETX" => match self.process_actions(&parts) {
                    Ok(d) => self.xcor = d,
                    Err(e) => self.print_log(line_number, &e.0),
                },
                "SETY" => match self.process_actions(&parts) {
                    Ok(d) => self.ycor = d,
                    Err(e) => self.print_log(line_number, &e.0),
                },
                "TURN" => match self.process_actions(&parts) {
                    Ok(d) => self.direction += d,
                    Err(e) => self.print_log(line_number, &e.0),
                },
                "MAKE" => match self.process_actions(&parts) {
                    Ok(d) => self.direction += d,
                    Err(e) => self.print_log(line_number, &e.0),
                },
                _ => {
                    println!("Wrong type of command on line {}", line_number);
                }
            }
            line_number += 1;
        }
    }

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
                } else if !commands[1].starts_with(QUOTES) {
                    if !commands[1].starts_with(":") {
                        return Err(CommandError("Wrong type of arguments".to_string()));
                    } else {
                        let mut arg = commands[1].to_string();
                        arg.remove(0);
                        // print!("{:?}", arg  );
                        match self.variables.get(&arg) {
                            Some(result) => {
                                return Ok(*result);
                            }
                            None => {
                                return Err(CommandError("Variable not found".to_string()));
                            }
                        }
                    }
                } else {
                    let mut arg = commands[1].to_string();
                    arg.remove(0);
                    let result = arg.parse::<f32>();
                    match result {
                        Ok(result) => {
                            return Ok(result);
                        }
                        Err(e) => {
                            return Err(CommandError(e.to_string()));
                        }
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
                    // print!("{:?}", name);
                    let mut v = commands[2].to_string();
                    v.remove(0);

                    let value = v.parse::<f32>();
                    // print!("{:?}", value);
                    match value {
                        Ok(result) => {
                            self.variables.insert(name, result);
                            Ok(0.0)
                        }
                        Err(e) => {
                            return Err(CommandError(e.to_string()));
                        }
                    }
                }
            }
            _ => {
                return Err(CommandError("Wrong type of command".to_string()));
            }
        }
    }

    fn print_log(&self, line_number: u32, error: &str) {
        println!("Error: {}, line: {}", error, line_number)
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

    let mut logo_parser = LogoParser::new(file_path.to_str().unwrap().to_string());

    logo_parser.ParseAction();

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
