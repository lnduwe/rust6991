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
/// A struct representing a Logo parser, containing various properties such as pen state, position, direction, and variables.
#[derive(Default)]
struct LogoParser<'a> {
    /// Struct representing the state of the Logo interpreter.
    ///
    /// # Fields
    ///
    /// * `pen_up`: A boolean indicating whether the pen is up or down.
    /// * `block`: An integer representing the number of brackets, for use with IF and WHILE statements.
    /// * `xcor`: A float representing the x-coordinate of the turtle.
    /// * `ycor`: A float representing the y-coordinate of the turtle.
    /// * `direction`: A float representing the direction the turtle is facing.
    /// * `pen_color`: A float representing the color of the pen.
    /// * `variables`: A HashMap that stores variables.
    /// * `line_number`: An usize representing the current line number.
    /// * `image`: An optional mutable reference to an Image.
    /// * `lines`: An optional iterator over the lines of code.
    /// * `procedures`: A HashMap that stores procedures.
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

    //parse commands from lines
    fn parse_action(&mut self) -> Result<(), ()> {
        while let Some(line) = self.lines.as_mut().unwrap().next() {
            if line.is_empty() {
                self.line_number += 1;
                continue;
            }
            let parts = &line.split_whitespace().collect();
            self.match_action(parts)?;
            self.line_number += 1;
        }
        Ok(())
    }

    //parse commands from the given vector
    fn parse_action_with_vec<T: AsRef<str>>(
        &mut self,
        commands: &Vec<T>,
        procedure: bool,
    ) -> Result<(), ()> {
        if !procedure {
            for l in commands {
                // let line = commands[i].as_ref();
                let line = l.as_ref();
                if line.is_empty() {
                    continue;
                }
                let parts = &line.split_whitespace().collect();

                self.match_action(parts)?;
            }
        } else {
            for i in 0..commands.len() {
                let line = commands[i].as_ref();
                if line.is_empty() {
                    continue;
                }
                let parts: &Vec<&str> = &line.split_whitespace().collect();
                if parts[0].eq("WHILE") {
                    let result = self.prefix(parts);
                    if result.is_none() {
                        return Err(());
                    }
                    let r = result.unwrap();
                    if r == 0.0 {
                        break;
                    }
                    let start = i + 1;
                    let mut end = 0;

                    for (j, l) in commands.iter().enumerate().skip(start) {
                        let line = l.as_ref();
                        if line.is_empty() {
                            continue;
                        }
                        if line.contains(']') {
                            end = j;
                            break;
                        }
                    }

                    loop {
                        for l in commands.iter().take(end).skip(start) {
                            let line = l.as_ref();
                            if line.is_empty() {
                                continue;
                            }
                            let parts: &Vec<&str> = &line.split_whitespace().collect();
                            self.match_action(parts)?;
                        }

                        let result = self.prefix(parts);
                        let r = result.unwrap();
                        if r == 0.0 {
                            break;
                        }
                    }
                }

                self.match_action(parts)?;
            }
        }
        Ok(())
    }

    //process action according to the command
    fn process_actions(&mut self, parts: &Vec<&str>) -> Result<f32, CommandError> {
        match parts[0] {
            "PENUP" | "PENDOWN" => {
                if parts.len() > 1 {
                    Err(CommandError("Wrong number of arguments".to_string()))
                } else {
                    Ok(0.0)
                }
            }
            "FORWARD" | "BACK" | "RIGHT" | "LEFT" | "SETPENCOLOR" | "TURN" | "SETHEADING"
            | "SETX" | "SETY" => {
                if parts.len() < 2 {
                    return Err(CommandError("Wrong number of arguments".to_string()));
                }
                if parts.len() > 2 {
                    let mut cmd: Vec<&str> = Vec::new();

                    for l in parts.iter().skip(1) {
                        cmd.push(l);
                    }

                    match self.prefix(&cmd) {
                        Some(result) => Ok(result),
                        None => Err(CommandError("Variable not found".to_string())),
                    }
                } else {
                    match self.get_value(parts[1]) {
                        Some(result) => Ok(result),
                        None => Err(CommandError("Variable not found".to_string())),
                    }
                }
            }
            "MAKE" => {
                if !parts[1].starts_with(QUOTES) {
                    Err(CommandError("Wrong type of arguments".to_string()))
                } else {
                    let mut cmd: Vec<&str> = Vec::new();
                    for pt in parts.iter().skip(2) {
                        cmd.push(pt);
                    }
                    let value = self.prefix(&cmd);
                    let name = parts[1][1..parts[1].len()].to_string();

                    match value {
                        Some(result) => {
                            self.variables.insert(name, result);
                            Ok(0.0)
                        }
                        None => Err(CommandError("Wrong type of arguments".to_string())),
                    }
                }
            }
            "ADDASSIGN" => {
                if parts.len() != 3 {
                    Err(CommandError("Wrong number of arguments".to_string()))
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
                    let line = self.lines.as_mut().unwrap().next();
                    if line.is_none() {
                        return Err(CommandError("Error parsing error".to_string()));
                    }
                    let line = line.unwrap();
                    if line.contains('[') {
                        semicolon += 1;
                    } else if line.contains(']') {
                        semicolon -= 1;
                    }
                    v.push(line);
                }

                loop {
                    let _ = self.parse_action_with_vec(&v, false);
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
                let mut arg: Vec<String> = Vec::new();
                for pt in parts.iter().skip(2) {
                    arg.push(pt[1..].to_string());
                }

                let mut cmd: Vec<String> = Vec::new();

                loop {
                    let line = self.lines.as_mut().unwrap().next();
                    if line.is_none() {
                        return Err(CommandError("Error parsing TO".to_string()));
                    }
                    let line = line.unwrap();
                    if line.trim().eq("END") {
                        break;
                    }
                    cmd.push(line.to_string());
                }

                let pros = Procedure {
                    commands: cmd,
                    args: arg,
                };

                self.procedures.insert(parts[1].to_string(), pros);

                Ok(0.0)
            }

            _ => Err(CommandError("Wrong type of command".to_string())),
        }
    }

    /// This function matches the action to be taken based on the input parts. It processes the actions and logs any errors that occur. It also updates the pen color, direction, and position based on the input parts.
    ///
    /// # Arguments
    ///
    /// * `self` - mutable reference to the current instance of the Turtle struct
    /// * `parts` - a vector of string slices representing the input parts to be processed
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the action is successfully matched and processed
    /// * `Err(())` if an error occurs during processing
    fn match_action(&mut self, parts: &Vec<&str>) -> Result<(), ()> {
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
                match self.process_actions(parts) {
                    Ok(_) => {}
                    Err(e) => {
                        return self.log_error(&e.0);
                    }
                }
            }
            "PENDOWN" => {
                self.pen_up = false;
                match self.process_actions(parts) {
                    Ok(_) => {}
                    Err(e) => {
                        return self.log_error(&e.0);
                    }
                }
            }
            "FORWARD" => match self.process_actions(parts) {
                Ok(d) => {
                    self.draw(d, "FORWARD");
                    println!("forward {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "BACK" => match self.process_actions(parts) {
                Ok(d) => {
                    self.draw(d, "BACK");
                    println!("b {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "RIGHT" => match self.process_actions(parts) {
                Ok(d) => {
                    self.draw(d, "RIGHT");
                    println!("r {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "LEFT" => match self.process_actions(parts) {
                Ok(d) => {
                    self.draw(d, "LEFT");
                    println!("left {}", d);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },

            "SETPENCOLOR" => match self.process_actions(parts) {
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
            "SETHEADING" => match self.process_actions(parts) {
                Ok(d) => {
                    self.direction = d;
                    println!("DIR {}", self.direction);
                }
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "SETX" => match self.process_actions(parts) {
                Ok(d) => self.xcor = d,
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "SETY" => match self.process_actions(parts) {
                Ok(d) => self.ycor = d,
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "TURN" => match self.process_actions(parts) {
                Ok(d) => {
                    self.direction += d;
                    println!("DIR {}", self.direction);
                }

                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "MAKE" => match self.process_actions(parts) {
                Ok(_) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "ADDASSIGN" => match self.process_actions(parts) {
                Ok(_) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "IF" => match self.process_actions(parts) {
                Ok(_) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "WHILE" => match self.process_actions(parts) {
                Ok(_) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },
            "TO" => match self.process_actions(parts) {
                Ok(_) => {}
                Err(e) => {
                    return self.log_error(&e.0);
                }
            },

            _ => {
                let pro = self.procedures.get(parts[0]);
                if let Some(proced) = pro {
                    let arg = proced.args.clone();

                    let len = arg.len();
                    let mut part: Vec<&str> = Vec::new();
                    for p in parts.iter().skip(1) {
                        part.push(p);
                    }

                    for ag in arg.iter().take(len) {
                        match self.prefix(&part) {
                            Some(result) => {
                                self.variables.insert(ag.to_string(), result);
                            }
                            None => {
                                return self.log_error("Error parsing procedure");
                            }
                        }
                    }

                    let cmd = proced.commands.clone();
                    self.parse_action_with_vec(&cmd, true)?;
                } else {
                    return self.log_error("No procedures found");
                }
            }
        }
        Ok(())
    }

    /// Logs an error message along with the line number where the error occurred.
    ///
    /// # Arguments
    ///
    /// * `error` - A string slice that holds the error message.
    ///
    /// # Returns
    ///
    /// Returns a `Result` enum with an empty `Err` variant.
    fn log_error(&self, error: &str) -> Result<(), ()> {
        println!("Error: {}, line: {}", error, self.line_number);
        Err(())
    }

    /// Draws a line of length `val` in the direction specified by `arg`.
    ///
    /// # Arguments
    ///
    /// * `val` - A `f32` representing the length of the line to be drawn.
    /// * `arg` - A `&str` representing the direction in which the line should be drawn. Valid values are "LEFT", "RIGHT", "FORWARD", and "BACK".
    fn draw(&mut self, val: f32, arg: &str) {
        let mut dir = self.direction;
        match arg {
            "LEFT" => {
                dir += 270.0;
            }
            "RIGHT" => {
                dir += 90.0;
            }
            "FORWARD" => {}
            "BACK" => {
                dir += 180.0;
            }
            _ => {}
        }

        if self.pen_up {
            (self.xcor, self.ycor) = get_end_coordinates(self.xcor, self.ycor, dir as i32, val);
            return;
        }

        (self.xcor, self.ycor) = self
            .image
            .as_mut()
            .expect("Null image")
            .draw_simple_line(
                self.xcor,
                self.ycor,
                dir as i32,
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

    match image_path.extension().and_then(|s| s.to_str()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_forward() {
        let mut turtle = LogoParser::new("test", 100, 100, None);
        turtle.draw(50.0, "FORWARD");
        assert_eq!(turtle.xcor, 50.0);
        assert_eq!(turtle.ycor, 0.0);
    }

    #[test]
    fn test_draw_back() {
        let mut turtle = LogoParser::new("xxx", 100, 100, None);
        turtle.draw(50.0, "BACK");
        assert_eq!(turtle.xcor, 50.0);
        assert_eq!(turtle.ycor, 100.0);
    }

    #[test]
    fn test_draw_left() {
        let mut turtle = LogoParser::new("xxx", 100, 100, None);
        turtle.draw(50.0, "LEFT");
        assert_eq!(turtle.xcor, 0.0);
        assert_eq!(turtle.ycor, 50.0);
    }

    #[test]
    fn test_draw_right() {
        let mut turtle = LogoParser::new("xxx", 100, 100, None);
        turtle.draw(50.0, "RIGHT");
        assert_eq!(turtle.xcor, 100.0);
        assert_eq!(turtle.ycor, 50.0);
    }
}
