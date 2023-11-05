use pars_libs::{parse_line, Remote, RemoteCommand};
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
    commands: Vec<ParallelCommand>,
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

    fn add_command(&mut self, command: String, args: Vec<String>) {
        self.commands.push(ParallelCommand {
            command,
            args,
            executed: false,
            exit_status: None,
        });
    }

    fn append_command(&mut self) {
        while *self.counter.lock().unwrap() < self.thread_limit {
            if let Some(command) = self.commands.pop() {
                *self.counter.lock().unwrap() += 1;
                std::thread::scope(|s| {
                    let thread = s.spawn(move || {
                        let mut cmd = Command::new(&command.command);
                        cmd.args(&command.args);
                        let code = cmd.status().unwrap().code();
                        return code;
                    });
                    let res = thread.join().unwrap();
                    *self.counter.lock().unwrap() -= 1;
                    if res == Some(0) {
                        todo!()
                    }
                });
            }
        }
    }

    fn execute_commands(&mut self) {
        for _ in 0..self.thread_limit {
            if let Some(command) = self.commands.pop() {
                *self.counter.lock().unwrap() += 1;
                std::thread::scope(|s| {
                    let thread = s.spawn(move || {
                        let mut cmd = Command::new(&command.command);
                        cmd.args(&command.args);
                        let code = cmd.status().unwrap().code();
                        return code;
                    });
                    let res = thread.join().unwrap();
                    *self.counter.lock().unwrap() -= 1;
                    if res == Some(0) {
                        todo!()
                    }
                });
            }
        }
    }
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

    for line in lines {
        let commands: Vec<Vec<String>> = parse_line(&line.unwrap()).unwrap();

        for command_args in commands {
            let command = &command_args[0];
            let args = &command_args[1..];
            let status = Command::new(command).args(args).status();

            if let Err(exit_status) = status {
                println!("Command failed with exit status: {:?}", exit_status);
                break; // Stop executing commands on this line
            }
        }
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
