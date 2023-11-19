use pars_libs::parse_line;
use std::collections::VecDeque;
use std::io::{stdout, BufRead, Write};
use std::process::Command;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
struct ParallelCommand {
    command: String,
    args: Vec<String>,
}

#[macro_use]
extern crate serde;
#[derive(Clone, Debug, Serialize)]
struct Message {
    id: usize,
    status: i32,
    msg: String,
}

#[derive(Clone, Debug)]
struct ParallelExecutor {
    commands: VecDeque<ParallelCommand>,
}

impl ParallelExecutor {
    fn new() -> Self {
        Self {
            commands: VecDeque::new(),
        }
    }

    fn execute_commands(&mut self, termination: i32, command_loop: Arc<Mutex<bool>>) -> bool {
        let mut stop = false;
        let mut msgs: Vec<Message> = Vec::new();
        // let mut i = 0;
        for (i, cmd) in self.commands.iter().enumerate() {
            if !*command_loop.lock().unwrap() {
                break;
            }
            let out = Command::new(cmd.command.as_str())
                .args(cmd.args.clone())
                .output();
            let mut msg = Message {
                id: i,
                status: 0,
                msg: String::from(""),
            };
            match out {
                Ok(output) => {
                    if output.status.code().unwrap() == 0 {
                        msg.status = 0;
                        msg.msg = String::from_utf8(output.stdout.clone()).unwrap();
                        msgs.push(msg);
                    } else {
                        msg.status = output.status.code().unwrap();
                        msg.msg = String::from_utf8(output.stderr.clone()).unwrap();
                        stop = true;
                        msgs.push(msg);
                        if termination == 2 {
                            command_loop.lock().unwrap().clone_from(&false);
                        }
                        break;
                        // }
                    }
                }
                Err(_) => {
                    msg.status = 1;
                    msg.msg = String::from("Error: Command not found");
                    msgs.push(msg);
                    stop = true;
                    if termination == 2 {
                        command_loop.lock().unwrap().clone_from(&false);
                    }
                    break;
                    // }
                }
            }
            // i += 1;
        }
        print_str(&serde_json::to_string(&msgs).unwrap());
        stop
    }
}

fn print_str(output: &str) {
    stdout().lock().write_all(output.as_bytes()).ok();
    stdout().lock().write_all(b"\n").ok();
}

fn start() {
    let args: Vec<String> = std::env::args().collect();

    let mut threads_limit = 2;
    // let mut r_value = String::new();
    // let mut mode = String::from("single");
    let mut remotes_str: Vec<String> = Vec::new();
    let mut termination_control = 0;
    // let mut remotes = Vec::<Remote>::new();

    for (index, arg) in args.iter().enumerate() {
        if arg == "-J" || arg == "--parallel" {
            if let Some(j_value) = args.get(index + 1) {
                if let Ok(j) = j_value.parse::<u32>() {
                    threads_limit = j;
                }
            }
        } else if arg == "-r" || arg == "--remote" {
            match args.get(index + 1) {
                Some(r_arg) => {
                    remotes_str.push(r_arg.clone());
                }
                None => {
                    // println!("Error: Remote address is not provided");
                    //exit
                    // std::process::exit(1);
                }
            }
        } else if arg == "-e" || arg == "--halt" {
            if let Some(ags) = args.get(index + 1) {
                match ags.as_str() {
                    "never" => termination_control = 0,
                    "lazy" => termination_control = 1,
                    "eager" => termination_control = 2,
                    _ => {
                        println!("Error: Invalid argument for --halt")
                    }
                }
            }
        } else if arg == "-s" || arg == "--secondary" {
            match args.get(index + 1) {
                Some(_s_arg) => {}
                None => {
                    // println!("Error: Remote address is not provided");
                }
            }
        }
    }

    // println!("");
    // println!("e {}", termination_control);

    // remotes_str.iter().for_each(|str| {
    //     let colon_idx = str.find(":");
    //     let slash_idx = str.find("/");
    //     if colon_idx.is_none() || slash_idx.is_none() {
    //         println!("Error: Invalid remote address");
    //         std::process::exit(1);
    //     }
    //     let rmt = Remote {
    //         addr: str[..colon_idx.unwrap()].to_string(),
    //         port: str[colon_idx.unwrap() + 1..slash_idx.unwrap()]
    //             .parse::<u16>()
    //             .unwrap(),
    //     };
    //     threads_limit = str[slash_idx.unwrap() + 1..]
    //         .parse::<u32>()
    //         .expect("Invalid port number");
    //     remotes.push(rmt);
    // });

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
        // sleep(Duration::from_secs(1));
        //   println!("{}", line.as_ref().unwrap());
        let commands: Vec<Vec<String>> = parse_line(&line.unwrap()).unwrap();
        let mut cmds: VecDeque<ParallelCommand> = VecDeque::new();
        if *stdin_loop.lock().unwrap() {
            for command_args in commands {
                let para = ParallelCommand {
                    command: command_args[0].clone(),
                    args: command_args[1..].to_vec(),
                };
                cmds.push_back(para);
            }
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
                        // return;
                    }
                });
            });
        }
    }
}

fn main() {
    start();
}
