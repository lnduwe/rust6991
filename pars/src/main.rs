use pars_libs::parse_line;
use std::collections::VecDeque;
use std::io::{self, stderr, stdout, BufRead, Read, Write};
// use std::io::{self, stderr, stdout, BufRead, Read, Write};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread::{current, sleep};
use std::time::Duration;
mod ssh;
use ssh::{Remote, RemoteCommand};

#[derive(Clone, Debug)]
struct ParallelCommand {
    command: String,
    args: Vec<String>,
}
#[macro_use]
extern crate serde;
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Message {
    id: i32,
    status: i32,
    msg: String,
}

struct Pipes {
    child_in: Arc<Mutex<std::process::ChildStdin>>,
    child_out: Arc<Mutex<std::process::ChildStdout>>,
}

#[derive(Clone, Debug)]
struct ParallelExecutor {
    commands: VecDeque<ParallelCommand>,
    commands_str: String,
}

impl ParallelExecutor {
    fn new() -> Self {
        Self {
            commands: VecDeque::new(),
            commands_str: String::new(),
        }
    }
    fn execute_remote_commands(
        &mut self,
        // termination: i32,
        // command_loop: Arc<Mutex<bool>>,
        stdin: Arc<Mutex<std::process::ChildStdin>>,
    ) {
        // let mut cmd_str = String::new();
        // for cmd in self.commands.iter() {
        //     //append cmd to cmd_str
        //     let space = String::from(" ");
        //     let mut args: String = String::new();
        //     cmd.args.iter().for_each(|arg| {
        //         args.push_str(&(space.clone() + arg));
        //     });

        //     cmd_str.push_str(&(cmd.command.clone() + &args + "; "));
        // }
        //     println!("cmd_str: {}", self.commands_str);
        // sleep(Duration::from_secs(1));
        self.commands_str.push_str("\n");
        // println!("cmd_str: {}",self.commands_str);
        stdin
            .lock()
            .unwrap()
            .write_all((self.commands_str).as_bytes())
            .unwrap();
        stdin.lock().unwrap().flush().unwrap();

        // let mut output = [0; 2048];

        // let mut bufreader = io::BufReader::new(&mut *stdout);

        // let size = bufreader.read(output.as_mut()).unwrap();

        // let res_str = String::from_utf8_lossy(&output[..size]).to_string();

        // let result: Result<Vec<Message>, serde_json::Error> = serde_json::from_str(&res_str);
        // let mut stop = false;

        // // println!("{}", res_str);

        // match result {
        //     Ok(msgs) => {
        //         msgs.iter().for_each(|msg| {
        //             if msg.status == 0 {
        //                 print_str(&msg.msg);
        //             } else {
        //                 stop = true;
        //                 // if termination == 2 {
        //                 //     command_loop.lock().unwrap().clone_from(&false);
        //                 // }
        //             }
        //         });
        //     }
        //     Err(e) => {
        //         println!("Error: {}", e);
        //         stop = true;
        //         if termination == 0 || termination == 1 {
        //             // print_str();
        //         }
        //         // else if termination == 2 {
        //         //     command_loop.lock().unwrap().clone_from(&false);
        //         // }
        //     }
        // }

        // stop
        // println!("{}", String::from_utf8_lossy(&output));
        // print_str(&String::from_utf8_lossy(&output));
    }

    //execute local commands

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

fn print_str(output: &str) {
    // for i in 0..output.len() {
    stdout().lock().write_all(&output.as_bytes()).ok();
    // }
}

fn start() {
    // let rmt = Remote {
    //     addr: String::from("170.64.129.76"),
    //     port: 22,
    // };

    // let mut cmd = Command::new("sh")
    //     .remote_spawn(&rmt)
    //     .expect("Error spawn");

    // let mut child_in = cmd.stdin.take().unwrap();
    // let mut child_out = cmd.stdout.take().unwrap();
    // let mut child_err = cmd.stderr.take().unwrap();

    // // let buf = "/bin/false\n";
    // let buf = "uname -a\n";

    // loop {
    //     child_in.write_all(buf.as_bytes()).unwrap();
    //     child_in.flush().unwrap();

    //     let mut buf = [0; 1024];
    //     // let mut output = Vec::new();
    //     let mut output = String::new();
    //     // let mut bufreader = io::BufReader::new(&mut child_out);
    //     // child_out.

    //     // child_out.read_to_string(&mut output);
    //     // bufreader.read_until(b'\n', &mut output).unwrap();

    //     // bufreader.read(output.as_mut()).unwrap();
    //     // println!("f {}", String::from_utf8_lossy(&output));

    //     match child_out.read(&mut buf) {
    //       Ok(0) => break, // No more data
    //       Ok(n) => output.push_str(std::str::from_utf8(&buf[..n]).unwrap()),
    //       Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break, // No more data, non-blocking
    //       Err(e) => panic!("Error reading from child process: {}", e),
    //   }
    //     print_str(&output);

    //     // let mut output = [0; 1024];
    //     // let mut bufreader = io::BufReader::new(&mut child_err);

    //     // bufreader.read(output.as_mut()).unwrap();
    //     // println!("{}", String::from_utf8_lossy(&output));
    //     sleep(Duration::from_secs(1));
    // }

    // return;

    let args: Vec<String> = std::env::args().collect();

    let mut threads_limit = 2;
    let mut r_value = String::new();
    let mut mode = String::from("single");
    let mut remotes_str: Vec<String> = Vec::new();
    let mut termination_control = 0;
    let mut remotes = Vec::<Remote>::new();
    // let mut pipes: Arc<Mutex<Vec<Pipes>>> = Arc::new(Mutex::new(Vec::new()));
    let mut pipes: Vec<Pipes> = Vec::new();

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
                    mode = String::from("server");
                }
                None => {
                    println!("Error: Remote address is not provided");
                    //exit
                    std::process::exit(1);
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
                Some(s_arg) => {
                    //  mode = s_arg.clone();
                }
                None => {
                    println!("Error: Remote address is not provided");
                }
            }
        }
    }

    if !remotes_str.is_empty() {
        remotes_str.iter().for_each(|str| {
            // println!("str: {}", str);
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
            threads_limit = str[slash_idx.unwrap() + 1..]
                .parse::<u32>()
                .expect("Invalid port number");
            remotes.push(rmt);
        });

        let mut start = "/root/ps1";
        let mut term = String::new();
        if termination_control == 1 {
            term.push_str("lazy");
        } else if termination_control == 2 {
            term.push_str("eager");
        } else {
            term.push_str("never");
        }

        let args = format!("{} -e {} -J {}\n", start, term, threads_limit);
        println!("args: {}", args);

        remotes.iter().for_each(|rmt| {
            let mut cmd = Command::new(args.as_str())
                .remote_spawn(&rmt)
                .expect("spawn failed");
            // let mut child_in = cmd.stdin.take().unwrap();
            // let mut child_out = cmd.stdout.take().unwrap();

            let pipe = Pipes {
                child_in: Arc::new(Mutex::new(cmd.stdin.take().unwrap())),
                child_out: Arc::new(Mutex::new(cmd.stdout.take().unwrap())),
            };
            pipes.push(pipe);
        });
    }

    let stdout_clone = Arc::clone(&pipes[0].child_out);
    std::thread::spawn(move || loop {
        let mut std_lock = stdout_clone.lock().unwrap();
        let mut output = [0; 2048];

        let mut bufreader = io::BufReader::new(&mut *std_lock);

        let size = bufreader.read(output.as_mut()).unwrap();

        let res_str = String::from_utf8_lossy(&output[..size]).to_string();

        let result: Result<Vec<Message>, serde_json::Error> = serde_json::from_str(&res_str);
        let mut stop = false;

        match result {
            Ok(msgs) => {
                msgs.iter().for_each(|msg| {
                    if msg.status == 0 {
                        print_str(&msg.msg);
                    }
                });
            }
            Err(e) => {}
        }
    });

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
    let stdin_clone = Arc::clone(&pipes[0].child_in);

    for line in lines {
        // let commands: Vec<Vec<String>> = parse_line(&line.unwrap()).unwrap();
        let com_str = line.unwrap();

        let mut cmds: VecDeque<ParallelCommand> = VecDeque::new();
        if *stdin_loop.lock().unwrap() {
            // for command_args in commands {
            //     let para = ParallelCommand {
            //         command: command_args[0].clone(),
            //         args: command_args[1..].to_vec(),
            //     };
            //     cmds.push_back(para);
            // }

            if mode == "server" {
                let stdin_lock = Arc::clone(&stdin_clone);
                // thread_pool.spawn(move || {
                let mut exec = ParallelExecutor::new();
                exec.commands_str = com_str;
                // let mut flag = false;

                exec.execute_remote_commands(
                    // termination_control,
                    // command_loop_clone,
                    stdin_lock,
                );
                // });
            } else {
                thread_pool.install(|| {
                    let stdin_loop_clone = Arc::clone(&stdin_loop);
                    let command_loop_clone = Arc::clone(&command_loop);

                    thread_pool.spawn(move || {
                        let mut exec = ParallelExecutor::new();
                        exec.commands = cmds;
                        let flag = exec.execute_commands(termination_control, command_loop_clone);

                        if flag && termination_control == 1 {
                            *stdin_loop_clone.lock().unwrap() = false;
                            return;
                        }
                    });
                });
            }
        }
    }
}

fn main() {
    start();
}
