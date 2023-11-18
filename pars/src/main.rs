// use libssh2_sys::libssh2_session_init_ex;
// use pars_libs::parse_line;
// use std::collections::VecDeque;
// use std::io::{stdout, BufRead, Read, Write};
// use std::net::TcpStream;
// use std::process::{Command, Stdio};
// use std::sync::{Arc, Mutex};
// use std::thread::{current, sleep};
// use std::time::Duration;
// mod ssh;
// use ssh::{Remote, RemoteCommand};

// use crate::ssh_module::Session;
// mod ssh_module;

// #[derive(Clone, Debug)]
// struct ParallelCommand {
//     command: String,
//     args: Vec<String>,
//     executed: bool,
//     exit_status: Option<i32>,
// }

// #[derive(Clone, Debug)]
// struct ParallelExecutor {
//     commands: VecDeque<ParallelCommand>,
//     mode: String,
// }

// impl ParallelExecutor {
//     fn new() -> Self {
//         Self {
//             commands: VecDeque::new(),
//             mode: String::from("Never"),
//         }
//     }

//     fn execute_remote_commands(
//         &mut self,
//         termination: i32,
//         command_loop: Arc<Mutex<bool>>,
//         sess:Arc::<Mutex< *mut libssh2_sys::LIBSSH2_SESSION>>,
//     ) -> bool {
//         let mut outputs = Vec::<_>::new();
//         let mut stop = false;
//         for cmd in self.commands.iter() {
//             if command_loop.lock().unwrap().clone() == false {
//                 break;
//             }
//             let out = ssh_module::send_command(*sess.lock().unwrap(), cmd.command.clone());
//             if out.1 == 0 {
//                 outputs.push(out.0);
//             } else {
//                 stop = true;
//                 if termination == 2 {
//                     command_loop.lock().unwrap().clone_from(&false);
//                 }
//                 break;
//             }
//         }
//         print_string(outputs);
//         stop
//     }

//     fn execute_commands(&mut self, termination: i32, command_loop: Arc<Mutex<bool>>) -> bool {
//         let mut outputs = Vec::<_>::new();

//         let mut stop = false;
//         for cmd in self.commands.iter() {
//             if command_loop.lock().unwrap().clone() == false {
//                 break;
//             }
//             // self.commands.iter().for_each(|cmd| {
//             let out = Command::new(cmd.command.as_str())
//                 .args(cmd.args.clone())
//                 .output();
//             match out {
//                 Ok(output) => {
//                     if output.status.code().unwrap() == 0 {
//                         outputs.push(output);
//                     } else {
//                         stop = true;
//                         if termination == 2 {
//                             command_loop.lock().unwrap().clone_from(&false);
//                         }
//                         break;
//                     }
//                 }
//                 Err(_) => {
//                     // if termination == 0 || termination == 1 {
//                     // println!("3323323");
//                     stop = true;
//                     if termination == 2 {
//                         command_loop.lock().unwrap().clone_from(&false);
//                     }
//                     break;
//                     // }
//                 }
//             }
//             // });
//         }
//         print_output(outputs);
//         stop
//     }
// }

// fn print_output(output: Vec<std::process::Output>) {
//     for i in 0..output.len() {
//         stdout().lock().write_all(&output[i].stdout).ok();
//     }
// }
// fn print_string(output: Vec<String>) {
//     for i in 0..output.len() {
//         stdout().lock().write_all(&output[i].as_bytes()).ok();
//     }
// }

// fn print_str(output: &str) {
//     // for i in 0..output.len() {
//     stdout().lock().write_all(&output.as_bytes()).ok();
//     // }
// }

// fn main() {
//     let args: Vec<String> = std::env::args().collect();

//     let mut threads_limit = 3;
//     let mut r_value = String::new();
//     let mut mode = String::from("single");
//     let mut remotes_str: Vec<String> = Vec::new();
//     let mut termination_control = 2;
//     let mut remotes = Vec::<String>::new();
//     let mut server_mode = false;
//     let mut sessions = Vec::<Session>::new();

//     for (index, arg) in args.iter().enumerate() {
//         //     if arg == "-J" || arg == "--parallel" {
//         //         if let Some(j_value) = args.get(index + 1) {
//         //             if let Ok(j) = j_value.parse::<u32>() {
//         //                 threads_limit = j;
//         //             }
//         //         }
//         //     } else
//         if arg == "-r" || arg == "--remote" {
//             server_mode = true;
//             match args.get(index + 1) {
//                 Some(r_arg) => {
//                     remotes_str.push(r_arg.clone());
//                     mode = String::from("server");
//                 }
//                 None => {
//                     println!("Error: Remote address is not provided");
//                     //exit
//                     std::process::exit(1);
//                 }
//             }
//         }
//         // else if arg == "-e" || arg == "--halt" {
//         //         if let Some(ags) = args.get(index + 1) {
//         //             match ags.as_str() {
//         //                 "never" => termination_control = 0,
//         //                 "lazy" => termination_control = 1,
//         //                 "eager" => termination_control = 2,
//         //                 _ => {
//         //                     println!("Error: Invalid argument for --halt")
//         //                 }
//         //             }
//         //         }
//         //     } else if arg == "-s" || arg == "--secondary" {
//         //         match args.get(index + 1) {
//         //             Some(s_arg) => {

//         //                 mode = s_arg.clone();
//         //             }
//         //             None => {
//         //                 println!("Error: Remote address is not provided");
//         //             }
//         //         }
//         //     }
//     }

//     if server_mode == true {
//         remotes_str.iter().for_each(|str| {
//             // let colon_idx = str.find(":");
//             let slash_idx = str.find("/");
//             if slash_idx.is_none() {
//                 println!("Error: Invalid remote address");
//                 std::process::exit(1);
//             }
//             // let rmt = Remote {
//             //     addr: str[..colon_idx.unwrap()].to_string(),
//             //     port: str[colon_idx.unwrap() + 1..slash_idx.unwrap()]
//             //         .parse::<u16>()
//             //         .unwrap(),
//             // };
//             let rmt = str[..slash_idx.unwrap()].to_string();
//             threads_limit = str[slash_idx.unwrap() + 1..]
//                 .parse::<i32>()
//                 .expect("Invalid port number");
//             remotes.push(rmt);
//         });

//         ssh_module::init_ssh();

//         // remotes.iter().for_each(|r| {
//         //     let mut s = Session {
//         //         sock: Err(std::io::Error::new(
//         //             std::io::ErrorKind::Other,
//         //             "Uninitialized",
//         //         )),
//         //         session: std::ptr::null_mut(),
//         //     };
//         //     ssh_module::verify_session(
//         //         &mut s,
//         //         r.clone(),
//         //         "azureuser".to_string(),
//         //         "/Users/orca/.ssh/sd.pem".to_string(),
//         //     );
//         //     sessions.push(s);
//         // });

//         let stdin = std::io::stdin();
//         let lines = stdin.lock().lines();
//         println!("{}", remotes_str[0].clone());

//         let mut s = Session {
//             sock: Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 "Uninitialized",
//             )),
//             session: std::ptr::null_mut(),
//         };
//         ssh_module::verify_session(
//             &mut s,
//             remotes[0].clone(),
//             "root".to_string(),
//             "/Users/orca/.ssh/github1".to_string(),
//         );

//         for line in lines {
//             let cmd = line.unwrap();

//             let res = ssh_module::send_command(s.session, cmd);

//             print!("{}", res.0);
//         }
//     }

//     let thread_pool = rayon::ThreadPoolBuilder::new()
//         .num_threads(threads_limit as usize)
//         .build()
//         .unwrap();

//     let stdin = std::io::stdin();
//     let lines = stdin.lock().lines();

//     // let mut loop_flag = true;
//     let stdin_loop = Arc::<Mutex<bool>>::new(Mutex::new(true));
//     let command_loop = Arc::<Mutex<bool>>::new(Mutex::new(true));
//     // let (sender, receiver) = std::sync::mpsc::channel();
//     for line in lines {
//         let commands: Vec<Vec<String>> = parse_line(&line.unwrap()).unwrap();
//         let mut cmds: VecDeque<ParallelCommand> = VecDeque::new();
//         if *stdin_loop.lock().unwrap() {
//             for command_args in commands {
//                 let para = ParallelCommand {
//                     command: command_args[0].clone(),
//                     args: command_args[1..].to_vec(),
//                     executed: false,
//                     exit_status: None,
//                 };
//                 cmds.push_back(para);
//             }
//             // let sender = sender.clone();
//             // let stdin_loop: Arc<Mutex<bool>> = stdin_loop.clone();

//             thread_pool.install(|| {
//                 let stdin_loop_clone = Arc::clone(&stdin_loop);
//                 let command_loop_clone = Arc::clone(&command_loop);
//                 // let sess = Arc::clone(&sessions[0].session);

//                 thread_pool.spawn(move || {
//                     let mut exec = ParallelExecutor::new();
//                     exec.commands = cmds;
//                     let mut flag = false;
//                     if server_mode == false {
//                         flag = exec.execute_commands(termination_control, command_loop_clone);
//                     } else {
//                         // flag = exec.execute_remote_commands(
//                         //     termination_control,
//                         //     command_loop_clone,
//                         //     sess ,
//                         // )
//                     }
//                     if flag && termination_control == 1 {
//                         // println!("Terminating the execution");
//                         *stdin_loop_clone.lock().unwrap() = false;
//                         // sender.send(false).unwrap();
//                         return;
//                     }
//                 });
//             });
//         }
//     }
// }

use std::{
    ffi::CString,
    io::BufRead,
    net::TcpStream,
    os::fd::{AsFd, AsRawFd},
    ptr::null,
    thread::sleep,
    time::Duration,
};

use libssh2_sys::*;

fn main() {
    unsafe {
        let rc = libssh2_init(0);
    }
    //sock addr
    let addr = "170.64.129.76:22";

    let sock: Result<TcpStream, std::io::Error> = TcpStream::connect(addr);
    if sock.is_err() {
        println!("error.");
        return;
    }
    unsafe {
        let session = libssh2_session_init_ex(None, None, None, std::ptr::null_mut());
        if session == std::ptr::null_mut() {
            print!("session error");
            return;
        }

        libssh2_session_set_blocking(session, 1);

        // loop {
        let mut rc = 0;
        loop {
            if let res = libssh2_session_handshake(session, sock.as_ref().unwrap().as_raw_fd()) {
                if res != LIBSSH2_ERROR_EAGAIN {
                    rc = res;
                    break;
                }
            }
        }

        if rc != 0 {
            return;
        }

        let fingerprint = libssh2_hostkey_hash(session, LIBSSH2_HOSTKEY_HASH_SHA256);
        println!("{:?}", fingerprint);
        // }

        // let nh = libssh2_knownhost_init(session);
        // if nh == std::ptr::null_mut() {
        //     println!("nh error");
        //     return ;
        // }
        let username = CString::new("root").expect("CString::new failed");
        let privk = CString::new("/Users/orca/.ssh/github1").expect("CString::new failed");
        let pass = CString::new("").expect("CString::new failed");

        let pass = "";
        loop {
            if let res = libssh2_userauth_publickey_fromfile_ex(
                session,
                username.as_ptr(),
                username.as_bytes().len() as u32,
                std::ptr::null(),
                privk.as_ptr(),
                std::ptr::null(),
            ) {
                println!("res {res}");
                if res != LIBSSH2_ERROR_EAGAIN {
                    rc = res;
                    break;
                }
            }
        }

        print_error(session);
        /////////////
        ///
        ///
        ///
        ///
        if rc != 0 {
            println!("error auth {}", rc);
            return;
        }
        // }

        let session_n = CString::new("session").expect("CString::new failed");
        //open channel
        let channel = libssh2_channel_open_ex(
            session,
            session_n.as_ptr(),
            7,
            LIBSSH2_CHANNEL_WINDOW_DEFAULT,
            LIBSSH2_CHANNEL_PACKET_DEFAULT,
            std::ptr::null(),
            0,
        );

        if channel == std::ptr::null_mut() {
            println!("frcgvkuyguyg");
        }
        print_error(session);

        let name: CString = CString::new("ps").unwrap();
        let val = CString::new("/root/ps").unwrap();
        libssh2_channel_setenv_ex(
            channel,
            name.as_ptr(),
            name.as_bytes().len() as u32,
            val.as_ptr(),
            val.as_bytes().len() as u32,
        );

        // let term = CString::new("vanilla").unwrap();
        // let aa = libssh2_channel_request_pty_ex(
        //     channel,
        //     term.as_ptr(),
        //     term.as_bytes().len() as u32,
        //     std::ptr::null(),
        //     0,
        //     80,
        //     24,
        //     0,
        //     0,
        // );

        let s = CString::new("exec").unwrap();
        let command = CString::new("ls").unwrap();

        let shl = libssh2_channel_process_startup(
            channel,
            s.as_ptr(),
            s.as_bytes().len() as u32,
            command.as_ptr(),
            command.as_bytes().len() as u32,
        );

        // let s = CString::new("shell").unwrap();
        // let shl = libssh2_channel_process_startup(
        //     channel,
        //     s.as_ptr(),
        //     s.as_bytes().len() as u32,
        //     std::ptr::null(),
        //     0,
        // );

        // println!("shl  {shl}");
        // print_error(session);

        let stdin = std::io::stdin();
        let lines = stdin.lock().lines();

        const BUF_SIZE: usize = 1024;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        let buflen = BUF_SIZE;
        let b = libssh2_channel_read_ex(channel, 0, buf.as_mut_ptr() as *mut i8, buflen);
        println!("{b}");
        println!("{}", String::from_utf8_lossy(&buf));

        return ;
        for line in lines {
            let cmd = line.unwrap();
            let cmd = CString::new("ls").unwrap();
            let written = libssh2_channel_write_ex(channel, 0, cmd.as_ptr(), cmd.as_bytes().len());
            // let _ = libssh2_channel_flush_ex(channel, 0);
            // let _ = libssh2_channel_eof(channel);
            if written < 0 {
                println!("written  {written}");
                return;
            }

            println!(" jnjni   ");
            const BUF_SIZE: usize = 1024;
            let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
            let buflen = BUF_SIZE;
            // sleep(Duration::from_secs(3));
            let b = libssh2_channel_read_ex(channel, 0, buf.as_mut_ptr() as *mut i8, buflen);

            if b == LIBSSH2_ERROR_EAGAIN.try_into().unwrap() {
                println!("eagain");
                continue;
            } else if b == 0 {
                println!("exit  ");
                return;
            }
            println!("{b}");
            println!("{}", String::from_utf8_lossy(&buf));
        }
    }

    // Rest of your program
}

fn print_error(session: *mut LIBSSH2_SESSION) {
    let mut error_msg = std::ptr::null_mut();
    let mut error_len = 0;

    // Call libssh2_session_last_error
    let error_code =
        unsafe { libssh2_session_last_error(session, &mut error_msg, &mut error_len, 1) };
    println!("errcode1111 :   {error_code}");
    unsafe {
        println!(
            "ffff  {:?}",
            String::from_utf8_lossy(std::ffi::CStr::from_ptr(error_msg).to_bytes())
        );
    }
}
