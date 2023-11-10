use pars_libs::{parse_line, Remote, RemoteCommand};
use std::collections::VecDeque;
use std::io::{self, stdout, BufRead, Write};
use std::process::{Command, ExitStatus};
use std::sync::{Arc, Mutex};
use std::vec;

fn parse_input() {
    let line = "ls -l | grep foo | wc -c";
    let commands = parse_line(line);
    println!("{:?}", commands);
}

#[derive(Clone, Debug)]
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
            thread_limit: 2,
            counter: Arc::new(Mutex::new(0)),
            mode: String::from("Never"),
        }
    }

    fn add_count(&mut self) {
        *self.counter.lock().unwrap() += 1;
    }
    fn sub_count(&mut self) {
        *self.counter.lock().unwrap() -= 1;
    }

    fn execute_commands(&mut self, commands: VecDeque<ParallelCommand>) {
        if *self.counter.lock().unwrap() >= self.thread_limit {
            println!("Thread limit reached");
            return;
        }
        self.add_count();
        let commands = commands.clone();
        // let (sender, receiver) = std::sync::mpsc::channel();
        std::thread::scope(|s| {
            let _ = s.spawn(move || {
                let mut outputs = Vec::<_>::new();
                commands.into_iter().for_each(|cmd| {
                    let out = Command::new(cmd.command.as_str())
                        .args(cmd.args.clone())
                        .output();
                    match out {
                        Ok(output) => {
                            outputs.push(output);
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                });
                self.sub_count();
                print_result(outputs);
            });
        });
    }
}

fn print_result(output: Vec<std::process::Output>) {
    for i in 0..output.len() {
        stdout().lock().write_all(&output[i].stdout).ok();
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

    let mut exec = ParallelExecutor::new();

    let _ = std::thread::scope(|s| {
        let _ = s.spawn(|| {
            let stdin = std::io::stdin();
            let lines = stdin.lock().lines();
            for line in lines {
                let commands: Vec<Vec<String>> = parse_line(&line.unwrap()).unwrap();
                let mut cmds: VecDeque<ParallelCommand> = VecDeque::new();
                for command_args in commands {
                    let para = ParallelCommand {
                        command: command_args[0].clone(),
                        args: command_args[1..].to_vec(),
                        executed: false,
                        exit_status: None,
                    };
                    cmds.push_back(para);
                }
                println!("exec cmds");
                exec.execute_commands(cmds);
                // exec.commands.push(cmds);
            }
        });
    });

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

// use std::{process::Command, thread::sleep, time::Duration};

// struct MyCommand {
//     command: String,
//     args: Vec<String>,
// }

// fn main() {
//     // Assuming you have a cmd variable of type MyCommand
//     let cmd = MyCommand {
//         command: "ls".to_string(),
//         args: vec!["-l".to_string(), "-a".to_string()],
//     };

//     // Create a new process builder without executing it
//     let process = Command::new(&cmd.command).args(&cmd.args).output();
//     println!("{}", String::from_utf8_lossy(&process.unwrap().stdout));

//     // Check if the process was created successfully
//     // match process {
//     //     Ok(mut child) => {
//     //         // Do something with the child process if needed
//     //         sleep(Duration::from_secs(50)); // For example, you can wait for the process to finish
//     //         let status = child.wait().expect("Failed to wait for child process");
//     //         println!("Child process exited with: {:?}", status);
//     //     }
//     //     Err(e) => {
//     //         eprintln!("Error spawning process: {:?}", e);
//     //     }
//     // }
// }
