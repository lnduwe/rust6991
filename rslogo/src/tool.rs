pub const QUOTES: char = '\"';
pub struct CommandError(pub String);

pub trait Tool {
    fn get_value(&self, name: &str) -> Option<f32>;



  //   fn process_if(commands:&Vec<&str> ,lines: &Option<std::str::Lines>, line_number:&mut usize) -> Result<f32, CommandError> {
  //     if commands.len() < 5 {
  //         return Err(CommandError("Wrong number of arguments".to_string()));
  //     }
  //     let mut flag = true;
  //     let arg = commands[1].to_string();
  //     if arg.eq("EQ") {
  //         flag = true;
  //     } else if arg.eq("NE") {
  //         flag = false;
  //     } else {
  //         return Err(CommandError("Wrong type of arguments".to_string()));
  //     }

  //     if commands.len() == 5 {
  //         let first = self.get_value(commands[2]).expect("Null value");
  //         let second = self.get_value(commands[3]).expect("Null value");
  //         if first == second && !flag {
  //             flag = false;
  //         }
  //         if first != second && flag {
  //             flag = false;
  //         }

  //         if commands[4] != "[" {
  //             return Err(CommandError("Wrong type of arguments".to_string()));
  //         }

  //         let mut count = 0;
  //         // if flag {

  //         if !flag {
  //             loop {
  //                 let line = lines.as_mut().unwrap().next();
  //                 if line.is_none() {
  //                     break;
  //                 }
  //                 let line = line.unwrap();
  //                 if line.len() == 0 {
  //                     // count += 1;
  //                     continue;
  //                 }
  //                 if !line.contains("]") {
                       
  //                 } else {
  //                     *line_number += 1;
  //                     break;
  //                 }
  //                 *line_number += 1;
  //             }
  //         }
  //         Ok(0.0)
  //     } else {
  //         Ok(0.0)
  //     }
  // }


}
