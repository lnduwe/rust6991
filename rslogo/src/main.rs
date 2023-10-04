use std::{fs::File, io::Read, num::ParseFloatError, result};

use clap::{parser, Parser};
use unsvg::Image;

const QUOTES: char = '\"';
// #[derive(Debug, PartialEq)]
// pub struct Color {
//   pub red: u8,
//   pub green: u8,
//   pub blue: u8,
// }

// fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
//   u8::from_str_radix(input, 16)
// }

// fn is_hex_digit(c: char) -> bool {
//   c.is_digit(16)
// }

// fn hex_primary(input: &str) -> IResult<&str, u8> {
//   map_res(
//     take_while_m_n(2, 2, is_hex_digit),
//     from_hex
//   ).parse(input)
// }

// fn hex_color(input: &str) -> IResult<&str, Color> {
//   let (input, _) = tag("#")(input)?;
//   let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
//   Ok((input, Color { red, green, blue }))
// }

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
struct Command {
    // name: String,
    // arg: Vec<String>,
    // xcor: f32,
    // ycor: f32,
}
impl Command {
    fn process(&self, commands: &Vec<&str>) -> Result<f32, CommandError> {
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
                    return Err(CommandError("Wrong type of arguments".to_string()));
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
            _ => {
                return Err(CommandError("Wrong type of command".to_string()));
            }
        }
    }
}
#[derive(Default)]
struct LogoParser {
    file_path: String,
    width: u32,
    height: u32,
    pen_up: bool,
}
impl LogoParser {
    fn ParseLogo(&mut self) {
        let mut xcor: f32 = 0.0;
        let mut ycor: f32 = 0.0;
        let mut direction: f32 = 0.0;
        let mut pen_color: f32 = 0.0;

        let mut commands: Vec<Command> = Vec::new();
        let mut file = File::open(&self.file_path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");
        let lines = contents.lines();
        let mut line_number = 1;
        let cmd = Command {};
        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // println!("{:?}", &parts);

            match parts[0] {
                "//" => {}
                "PENUP" => {
                    self.pen_up = true;
                    match cmd.process(&parts) {
                        Ok(_) => {}
                        Err(e) => println!("Error: {}", e.0),
                    }
                }
                "PENDOWN" => {
                    self.pen_up = false;
                    match cmd.process(&parts) {
                        Ok(_) => {}
                        Err(e) => println!("Error: {}", e.0),
                    }
                }
                "FORWARD" => {
                    direction = 0.0;
                    match cmd.process(&parts) {
                        Ok(d) => {
                            println!("{}", d);
                        }
                        Err(e) => println!("Error: {}", e.0),
                    }
                }
                "BACK" => {
                    direction = 180.0;
                    match cmd.process(&parts) {
                        Ok(d) => {
                            println!("{}", d);
                        }
                        Err(e) => println!("Error: {}", e.0),
                    }
                }
                "RIGHT" => {
                    direction = 90.0;

                    match cmd.process(&parts) {
                        Ok(d) => {
                            println!("{}", d);
                        }
                        Err(e) => println!("Error: {}", e.0),
                    }
                }

                "LEFT" => {
                    direction = 270.0;

                    match cmd.process(&parts) {
                        Ok(d) => {
                            println!("{}", d);
                        }
                        Err(e) => println!("Error: {}", e.0),
                    }
                }

                "SETPENCOLOR" => match cmd.process(&parts) {
                    Ok(d) => pen_color = d,
                    Err(e) => println!("Error: {}", e.0),
                },
                "SETHEADING" => match cmd.process(&parts) {
                    Ok(d) => direction = d,
                    Err(e) => println!("Error: {}", e.0),
                },
                "SETX" => match cmd.process(&parts) {
                    Ok(d) => xcor = d,
                    Err(e) => println!("Error: {}", e.0),
                },
                "SETY" => match cmd.process(&parts) {
                    Ok(d) => ycor = d,
                    Err(e) => println!("Error: {}", e.0),
                },
                "TURN" => match cmd.process(&parts) {
                    Ok(d) => direction += d,
                    Err(e) => println!("Error: {}", e.0),
                },
                _ => {
                    println!("Wrong type of command on line {}", line_number);
                }
            }
            line_number += 1;
        }
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

    let mut logo_parser = LogoParser {
        file_path: file_path.to_str().unwrap().to_string(),
        width: width,
        height: height,
        pen_up: true,
    };

    logo_parser.ParseLogo();

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
