use std::collections::VecDeque;
use std::io::{self, stdout, BufRead, Read, Write};
use std::process::Command;
use std::sync::{Arc, Mutex};
mod ssh;
use ssh::{parse_line, Remote, RemoteCommand};

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
    count: Arc<Mutex<i32>>,
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
    fn execute_remote_commands(&mut self, stdin: Arc<Mutex<std::process::ChildStdin>>) {
        self.commands_str.push('\n');
        // println!("cmd_str: {}",self.commands_str);
        stdin
            .lock()
            .unwrap()
            .write_all((self.commands_str).as_bytes())
            .unwrap();
        stdin.lock().unwrap().flush().unwrap();
    }

    //execute local commands
    fn execute_commands(&mut self, termination: i32, command_loop: Arc<Mutex<bool>>) -> bool {
        let mut outputs = Vec::<_>::new();

        let mut stop = false;
        for cmd in self.commands.iter() {
            if !*command_loop.lock().unwrap() {
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
    for item in &output {
        print_str(&String::from_utf8_lossy(&item.stdout));
    }
}

fn print_str(output: &str) {
    // for i in 0..output.len() {
    stdout().lock().write_all(output.as_bytes()).ok();
    // }
}

/// Parses a JSON string into a vector of `Message` structs.
///
/// # Arguments
///
/// * `str` - The input JSON string as a byte slice.
/// * `size` - The size of the byte slice to be parsed.
///
/// # Returns
///
/// Returns a `Result` containing the parsed vector of `Message` structs if successful,
/// or a `serde_json::Error` if the parsing fails.
fn parse_json(str: &[u8], size: usize) -> Result<Vec<Message>, serde_json::Error> {
    let str_slice = std::str::from_utf8(&str[..size]).unwrap();

    let res: Result<Vec<Message>, serde_json::Error> = serde_json::from_str(str_slice);

    res
}

fn resolve_json_results(str: &[u8], size: usize) -> bool {
    let result = parse_json(str, size);

    let mut status = false;
    match result {
        Ok(msgs) => {
            msgs.iter().for_each(|msg| {
                if msg.status == 0 {
                    print_str(&msg.msg);
                    status = true;
                } else {
                    status = false;
                }
            });
        }
        Err(_e) => {
            status = false;
        }
    }
    status
}

/// Finds the remote with the lowest load.
///
/// Given a vector of `Pipes`, this function iterates through each pipe and finds the remote
/// with the lowest load. The load is determined by the `count` field of each pipe. The function
/// returns the index of the remote with the lowest load.
///
/// # Arguments
///
/// * `pipes` - A vector of `Pipes` representing the remote pipes.
///
/// # Returns
///
/// The index of the remote with the lowest load, or -1 if no remote is found.
fn find_remote_with_lowest_load(pipes: &[Pipes]) -> i32 {
    let mut min = -1;
    let mut min_count = i32::MAX;
    for (i, p) in pipes.iter().enumerate() {
        let count = *p.count.lock().unwrap();
        if count < min_count && count >= 0 {
            min_count = count;
            min = i as i32;
        }
    }
    min
}

fn start() {
    let args: Vec<String> = std::env::args().collect();

    let mut threads_limit = 2;
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
                Some(_s_arg) => {
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
            let colon_idx = str.find(':');
            let slash_idx = str.find('/');
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

        let start = "pars";
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
                .remote_spawn(rmt)
                .expect("spawn failed");
            // let mut child_in = cmd.stdin.take().unwrap();
            // let mut child_out = cmd.stdout.take().unwrap();

            let pipe = Pipes {
                child_in: Arc::new(Mutex::new(cmd.stdin.take().unwrap())),
                child_out: Arc::new(Mutex::new(cmd.stdout.take().unwrap())),
                count: Arc::new(Mutex::new(0)),
            };
            pipes.push(pipe);
        });
    }
    if mode == "server" {
        for p in &pipes {
            let stdout_clone = Arc::clone(&p.child_out);
            let count_clone = Arc::clone(&p.count);
            std::thread::spawn(move || loop {
                let mut std_lock = stdout_clone.lock().unwrap();
                // let mut count_lock = count_clone.lock().unwrap();

                let mut output = [0; 2048];

                let mut bufreader = io::BufReader::new(&mut *std_lock);

                let size = bufreader.read(output.as_mut()).unwrap();

                let status = resolve_json_results(&output, size);

                if !status {
                    //if termination mode is eager or lazy, disable the remote server when receive an error
                    if termination_control == 1 || termination_control == 2 {
                        *count_clone.lock().unwrap() = -1;
                    } else {
                        *count_clone.lock().unwrap() -= 1;
                    }
                } else {
                    *count_clone.lock().unwrap() -= 1;
                }
            });
        }
    }

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads_limit as usize)
        .build()
        .unwrap();

    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let stdin_loop = Arc::<Mutex<bool>>::new(Mutex::new(true));
    let command_loop = Arc::<Mutex<bool>>::new(Mutex::new(true));
    // let (sender, receiver) = std::sync::mpsc::channel();

    for line in lines {
        let com_str = line.unwrap();

        if *stdin_loop.lock().unwrap() {
            if mode == "server" {
                if pipes.len() == 1 {
                    let stdin_clone = Arc::clone(&pipes[0].child_in);
                    // thread_pool.spawn(move || {
                    let mut exec = ParallelExecutor::new();
                    exec.commands_str = com_str;
                    exec.execute_remote_commands(stdin_clone);
                } else {
                    let mut exec = ParallelExecutor::new();
                    exec.commands_str = com_str;
                    let min = find_remote_with_lowest_load(&pipes);
                    //if no remote is available, skip the command

                    if min < 0 {
                        continue;
                    }
                    let min = min as usize;
                    let count_clone = Arc::clone(&pipes[min].count);
                    let stdin_clone = Arc::clone(&pipes[min].child_in);
                    *count_clone.lock().unwrap() += 1;
                    exec.execute_remote_commands(stdin_clone);
                }
                // });
            } else {
                let mut cmds: VecDeque<ParallelCommand> = VecDeque::new();

                let commands: Vec<Vec<String>> = parse_line(&com_str).unwrap();

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

                        if flag && (termination_control == 1 || termination_control == 2) {
                            *stdin_loop_clone.lock().unwrap() = false;
                            // return;
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_valid_input() {
        let input =
            r#"[{"id": 0, "status": 0, "msg": "Hello"}, {"id": 1, "status": 1, "msg": "World"}]"#;

        let result = parse_json(input.as_bytes(), input.len());
        let vecs = result.unwrap();

        assert_eq!(vecs[0].msg, "Hello");
        assert_eq!(vecs[1].msg, "World");
    }

    #[test]
    fn test_parse_json_invalid_input() {
        let input = r#"[{"status": 0, "msg": "Hello"}, {"status": 1, "msg": "World"}"#;

        let result = parse_json(input.as_bytes(), input.len());

        assert!(result.is_err());
    }
}
