use pars_libs::{parse_line, Remote, RemoteCommand};
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::process::{Command, ExitStatus};
use std::sync::{Arc, Mutex};
use std::vec;

fn parse_input() {
    let line = "ls -l | grep foo | wc -c";
    let commands = parse_line(line);
    println!("{:?}", commands);
}

struct ParallelCommand {
    command: String,
    args: Vec<String>,
    executed: bool,
    exit_status: Option<i32>,
}

struct ParallelExecutor {
    commands: Vec<VecDeque<ParallelCommand>>,
    thread_limit: usize,
    counter: Arc<Mutex<usize>>,
    mode: String,
}

impl ParallelExecutor {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
            thread_limit: 1,
            counter: Arc::new(Mutex::new(0)),
            mode: String::from("Never"),
        }
    }

    // fn add_command(&mut self, command: String, args: Vec<String>) {
    //     self.commands.push_back(ParallelCommand {
    //         command,
    //         args,
    //         executed: false,
    //         exit_status: None,
    //     });
    // }


    fn execute_commands(&mut self) {
        if *self.counter.lock().unwrap() >= self.thread_limit {
            return;
        }
        // if let Some(command) = self.commands.first() {
            *self.counter.lock().unwrap() += 1;
          if  let Some(command) = self.commands.pop(){
            let thread = std::thread::spawn(move || {
              command.iter().for_each(|cmd|{
                let mut com = Command::new(cmd.command.as_str());
                    com.args(cmd.args.clone());
                    let code = com.status();
              });
            });
            // vec_threads.push(thread);
            thread.join();
            *self.counter.lock().unwrap() -= 1;
            self.execute_commands();
        }
    }

    // for thread in vec_threads {
    //     thread.join().unwrap();
    // }
}

fn main() {
    // let args: Vec<String> = std::env::args().collect();

    // let mut num_j = 0;
    // let mut r_value = String::new();

    // for (index, arg) in args.iter().enumerate() {
    //     if arg == "-j" {
    //         if let Some(j_value) = args.get(index + 1) {
    //             if let Ok(j) = j_value.parse::<u32>() {
    //                 num_j = j;
    //             }
    //         }
    //     } else if arg == "-r" {
    //         if let Some(r_arg) = args.get(index + 1) {
    //             r_value = r_arg.clone();
    //         }
    //     } else if arg == "-e" || arg == "--halt" {
    //         if let Some(e_arg) = args.get(index + 1) {
    //             r_value = e_arg.clone();
    //         }
    //     }
    // }

    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let mut exec = ParallelExecutor::new();

    for line in lines {
        let commands: Vec<Vec<String>> = parse_line(&line.unwrap()).unwrap();
        let mut  cmds: VecDeque<ParallelCommand> = VecDeque::new();
        for command_args in commands {
            let para = ParallelCommand {
                command: command_args[0].clone(),
                args: command_args[1..].to_vec(),
                executed: false,
                exit_status: None,
            };
            cmds.push_back(para);
            // println!("{:?}", para.command);
            // println!("{:?}", para.args);
            // exec.commands.push(para);
        }
        exec.commands.push(cmds);
        // for command_args in commands {
        //     let command = &command_args[0];
        //     let args = &command_args[1..];
        //     let status = Command::new(command).args(args).status();

        //     if let Err(exit_status) = status {
        //         println!("Command failed with exit status: {:?}", exit_status);
        //         break; // Stop executing commands on this line
        //     }
        // }
        exec.execute_commands();
    }

    // println!("Number of -j: {}", num_j);
    // println!("Value of -r: {}", r_value);

    // parse_input();
}

//  use pars_lib::parse_line;
// use std::process::Command;
// use std::sync::{Arc, Mutex};
// use std::thread;

// struct CommandState {
//     command: String,
//     args: Vec<String>,
//     executed: bool,
//     exit_status: Option<i32>,
// }

// struct CommandExecutor {
//     commands: Arc<Mutex<Vec<CommandState>>>,
// }

// impl CommandExecutor {
//     fn new() -> Self {
//         Self {
//             commands: Arc::new(Mutex::new(Vec::new())),
//         }
//     }

//     fn add_command(&mut self, command: String, args: Vec<String>) {
//         self.commands.lock().unwrap().push(CommandState {
//             command,
//             args,
//             executed: false,
//             exit_status: None,
//         });
//     }

//     fn execute_commands(&self) {
//         let mut threads = Vec::new();

//         for command in self.commands.lock().unwrap().iter() {
//             if command.executed {
//                 continue;
//             }

//             let commands = self.commands.clone();
//             let thread = thread::spawn(move || {
//                 let mut command = Command::new(&command.command);
//                 command.args(&command.args);

//                 let output = command.output().unwrap();

//                 commands.lock().unwrap()[command.command].executed = true;
//                 commands.lock().unwrap()[command.command].exit_status = Some(output.status.code());

//                 if output.status.code() != 0 {
//                     panic!("Command failed: {} {}", command.command, command.args.join(" "));
//                 }
//             });

//             threads.push(thread);
//         }

//         for thread in threads {
//             thread.join().unwrap();
//         }
//     }
// }

// fn main() {
//     let mut executor = CommandExecutor::new();

//     for line in std::io::stdin().lines() {
//         let commands = parse_line(line.unwrap());

//         for command in commands {
//             executor.add_command(command[0].clone(), command[1:].to_vec());
//         }

//         executor.execute_commands();
//     }
// }
