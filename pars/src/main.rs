use pars_libs::parse_line;
use std::collections::VecDeque;
use std::io::{stdout, BufRead, Write};
use std::process::{Command};
use std::sync::{Arc, Mutex};
use std::thread::{current, sleep};
use std::time::Duration;
mod ssh;
use ssh::{Remote, RemoteCommand};

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

    fn execute_commands(&mut self, termination: i32, command_loop: Arc<Mutex<bool>>) -> bool {
        let mut outputs = Vec::<_>::new();

        let mut stop = false;
        for cmd in self.commands.iter() {
            if command_loop.lock().unwrap().clone() == false {
                break;
            }
            // self.commands.iter().for_each(|cmd| {
            let out = Command::new(cmd.command.as_str())
                .args(cmd.args.clone())
                .output();
            match out {
                Ok(output) => {
                    if output.status.code().unwrap() == 0 {
                        outputs.push(output);
                    } else {
                        // if termination == 0  {
                        // println!("kdmkk");
                        stop = true;
                        if termination == 2 {
                            command_loop.lock().unwrap().clone_from(&false);
                        }
                        break;
                        // }
                    }
                }
                Err(_) => {
                    // if termination == 0 || termination == 1 {
                    // println!("3323323");
                    stop = true;
                    if termination == 2 {
                        command_loop.lock().unwrap().clone_from(&false);
                    }
                    break;
                    // }
                }
            }
            // });
        }
        print_result(outputs);
        stop
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
    let args: Vec<String> = std::env::args().collect();

    let mut threads_limit = 2;
    let mut r_value = String::new();
    let mut mode = String::from("Server");
    let mut remotes_str: Vec<String> = Vec::new();
    let mut termination_control = 1;
    let mut remotes = Vec::<Remote>::new();

    for (index, arg) in args.iter().enumerate() {
        //     if arg == "-J" || arg == "--parallel" {
        //         if let Some(j_value) = args.get(index + 1) {
        //             if let Ok(j) = j_value.parse::<u32>() {
        //                 threads_limit = j;
        //             }
        //         }
        //     } else
        if arg == "-r" || arg == "--remote" {
            match args.get(index + 1) {
                Some(r_arg) => {
                    remotes_str.push(r_arg.clone());
                }
                None => {
                    println!("Error: Remote address is not provided");
                    //exit
                    std::process::exit(1);
                }
            }
        }
        // else if arg == "-e" || arg == "--halt" {
        //         if let Some(ags) = args.get(index + 1) {
        //             match ags.as_str() {
        //                 "never" => termination_control = 0,
        //                 "lazy" => termination_control = 1,
        //                 "eager" => termination_control = 2,
        //                 _ => {
        //                     println!("Error: Invalid argument for --halt")
        //                 }
        //             }
        //         }
        //     } else if arg == "-s" || arg == "--secondary" {
        //         match args.get(index + 1) {
        //             Some(s_arg) => {
        //                 mode = s_arg.clone();
        //             }
        //             None => {
        //                 println!("Error: Remote address is not provided");
        //             }
        //         }
        //     }
    }

    remotes_str.iter().for_each(|str| {
        let colon_idx = str.find(":");
        let slash_idx = str.find("/");
        if colon_idx.is_none() || slash_idx.is_none() {
            println!("Error: Invalid remote address");
            std::process::exit(1);
        }
        let rmt = Remote {
            addr: str[..colon_idx.unwrap()].to_string(),
            port: str[colon_idx.unwrap() + 1..slash_idx.unwrap()]
                .parse::<u16>()
                .unwrap(),
        };
        remotes.push(rmt);
    });

    let rmt = Command::new("uname").arg("-a").remote_output(&remotes[0]);
    print_result(vec![rmt.unwrap()]);
   

    return;

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads_limit as usize)
        .build()
        .unwrap();

    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    // let mut loop_flag = true;
    let stdin_loop = Arc::<Mutex<bool>>::new(Mutex::new(true));
    let command_loop = Arc::<Mutex<bool>>::new(Mutex::new(true));
    // let (sender, receiver) = std::sync::mpsc::channel();
    for line in lines {
        let commands: Vec<Vec<String>> = parse_line(&line.unwrap()).unwrap();
        let mut cmds: VecDeque<ParallelCommand> = VecDeque::new();
        if *stdin_loop.lock().unwrap() {
            for command_args in commands {
                let para = ParallelCommand {
                    command: command_args[0].clone(),
                    args: command_args[1..].to_vec(),
                    executed: false,
                    exit_status: None,
                };
                cmds.push_back(para);
            }
            // let sender = sender.clone();
            // let stdin_loop: Arc<Mutex<bool>> = stdin_loop.clone();

            thread_pool.install(|| {
                let stdin_loop_clone = Arc::clone(&stdin_loop);
                let command_loop_clone = Arc::clone(&command_loop);

                thread_pool.spawn(move || {
                    let mut exec = ParallelExecutor::new();
                    exec.commands = cmds;
                    let flag = exec.execute_commands(termination_control, command_loop_clone);
                    if flag && termination_control == 1 {
                        // println!("Terminating the execution");
                        *stdin_loop_clone.lock().unwrap() = false;
                        // sender.send(false).unwrap();
                        return;
                    }
                });
            });
        }
        // if !*stdin_loop.lock().unwrap() {
        //     break;
        // }
    }
}
