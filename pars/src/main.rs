use pars_libs::{parse_line, Remote, RemoteCommand};
use std::collections::VecDeque;
use std::io::{self, stdout, BufRead, Write};
use std::process::{Command, ExitStatus};
use std::sync::{Arc, Mutex};
use std::thread::{current, sleep};
use std::time::Duration;

#[derive(Clone, Debug)]
struct ParallelCommand {
    command: String,
    args: Vec<String>,
    executed: bool,
    exit_status: Option<i32>,
}

#[derive(Clone, Debug)]
struct ParallelExecutor {
    commands: VecDeque<ParallelCommand>,
    mode: String,
}

impl ParallelExecutor {
    fn new() -> Self {
        Self {
            commands: VecDeque::new(),
            mode: String::from("Never"),
        }
    }

    fn execute_commands(&mut self) {
        let mut outputs = Vec::<_>::new();
        self.commands.iter().for_each(|cmd| {
            let out = Command::new(cmd.command.as_str())
                .args(cmd.args.clone())
                .output();
            match out {
                Ok(output) => {
                    outputs.push(output);
                }
                Err(_) => {
                    //     println!("Error: {}", e);
                }
            }
        });

        print_result(outputs);
    }
}

fn print_result(output: Vec<std::process::Output>) {
    for i in 0..output.len() {
        stdout().lock().write_all(&output[i].stdout).ok();
    }
}

fn test() {
    sleep(Duration::from_secs(10));
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
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

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

        thread_pool.install(|| {
            thread_pool.spawn(move || {
                let mut exec = ParallelExecutor::new();
                exec.commands = cmds;
                exec.execute_commands();
            });
        });
    }
}
