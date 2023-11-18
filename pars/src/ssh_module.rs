use std::{ffi::CString, net::TcpStream, os::fd::AsRawFd, sync::{Arc, Mutex}};

use libssh2_sys::*;

pub struct Session {
    pub sock: Result<TcpStream, std::io::Error>,
    pub session: *mut libssh2_sys::LIBSSH2_SESSION,
}

pub fn init_ssh() {
    unsafe {
        let rc = libssh2_init(0);
        if rc != 0 {
            println!("error init");
            return;
        }
    }
}

pub fn verify_session(s: &mut Session, addr: String, username: String, privateKey: String) {
    unsafe {
        s.sock = TcpStream::connect(addr);
        if s.sock.is_err() {
            println!("1error.");
            return;
        }

        s.session = libssh2_session_init_ex(None, None, None, std::ptr::null_mut());
        if s.session == std::ptr::null_mut() {
            print!("session error");
            return;
        }

        let mut rc = 0;
        loop {
            if let res = libssh2_session_handshake(s.session, s.sock.as_ref().unwrap().as_raw_fd())
            {
                if res != LIBSSH2_ERROR_EAGAIN {
                    rc = res;
                    break;
                }
            }
        }

        if rc != 0 {
            return;
        }

        let fingerprint = libssh2_hostkey_hash(s.session, LIBSSH2_HOSTKEY_HASH_SHA256);
        // println!("{:?}", fingerprint);

        let username = CString::new(username).expect("CString::new failed");
        let privk = CString::new(privateKey).expect("CString::new failed");
        // let pass = CString::new("").expect("CString::new failed");
        loop {
            if let res = libssh2_userauth_publickey_fromfile_ex(
                s.session,
                username.as_ptr(),
                username.as_bytes().len() as u32,
                std::ptr::null(),
                privk.as_ptr(),
                std::ptr::null(),
            ) {
                // println!("res {res}");
                if res != LIBSSH2_ERROR_EAGAIN {
                    rc = res;
                    break;
                }
            }
        }

        print_error(s.session);
        if rc != 0 {
            println!("error auth {}", rc);
            return;
        }

        // session
    }
}

pub fn send_command(sess: *mut LIBSSH2_SESSION, cmd: String) -> (String, i32) {
    let session_n = CString::new("session").expect("CString::new failed");
    unsafe {
        //open channel
        let channel = libssh2_channel_open_ex(
            sess,
            session_n.as_ptr(),
            7,
            LIBSSH2_CHANNEL_WINDOW_DEFAULT,
            LIBSSH2_CHANNEL_PACKET_DEFAULT,
            std::ptr::null(),
            0,
        );

        print_error(sess);

        let s = CString::new("exec").unwrap();
        let command = CString::new(cmd).unwrap();

        let shl = libssh2_channel_process_startup(
            channel,
            s.as_ptr(),
            s.as_bytes().len() as u32,
            command.as_ptr(),
            command.as_bytes().len() as u32,
        );

        print_error(sess);

        const BUF_SIZE: usize = 1024;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        let buflen = BUF_SIZE;

        let b = libssh2_channel_read_ex(channel, 0, buf.as_mut_ptr() as *mut i8, buflen);

        let mut rc = 0;
        if libssh2_channel_close(channel) == 0 {
            rc = libssh2_channel_get_exit_status(channel);
            println!("exit status: {}", rc);
        }

        libssh2_channel_free(channel);
        // channel = std::ptr::null_mut();

        print_error(sess);
        // println!("{b}");
        (String::from_utf8_lossy(&buf).to_string(), rc)
    }
}

fn close_connection(s: Session) {
    unsafe {
        if s.session != std::ptr::null_mut() {
            libssh2_session_disconnect_ex(
              s.session,
                SSH_DISCONNECT_BY_APPLICATION,
                CString::new("Shutdown").unwrap().as_ptr(),
                CString::new("").unwrap().as_ptr(),
            );
            libssh2_session_free(s.session);
        }

        if s.sock.is_ok() {
            s.sock.unwrap().shutdown(std::net::Shutdown::Both);
        }
    }
}

fn exit_libssh() {
    unsafe {
        libssh2_exit();
    }
}

// fn main() {
//     unsafe {
//         let rc = libssh2_init(0);
//     }
//     //sock addr
//     let addr = "20.248.197.32:22";

//     let sock: Result<TcpStream, std::io::Error> = TcpStream::connect(addr);
//     if sock.is_err() {
//         println!("error.");
//         return;
//     }
//     unsafe {
//         let session = libssh2_session_init_ex(None, None, None, std::ptr::null_mut());
//         if session == std::ptr::null_mut() {
//             print!("session error");
//             return;
//         }

//         // libssh2_session_set_blocking(session, 0);

//         // loop {
//         let mut rc = 0;
//         loop {
//             if let res = libssh2_session_handshake(session, sock.as_ref().unwrap().as_raw_fd()) {
//                 if res != LIBSSH2_ERROR_EAGAIN {
//                     rc = res;
//                     break;
//                 }
//             }
//         }

//         if rc != 0 {
//             return;
//         }

//         let fingerprint = libssh2_hostkey_hash(session, LIBSSH2_HOSTKEY_HASH_SHA256);
//         println!("{:?}", fingerprint);
//         // }

//         // let nh = libssh2_knownhost_init(session);
//         // if nh == std::ptr::null_mut() {
//         //     println!("nh error");
//         //     return ;
//         // }
//         let username = CString::new("azureuser").expect("CString::new failed");
//         let privk = CString::new("/root/pars/sd.pem").expect("CString::new failed");
//         let pass = CString::new("").expect("CString::new failed");

//         loop {
//             if let res = libssh2_userauth_publickey_fromfile_ex(
//                 session,
//                 username.as_ptr(),
//                 username.as_bytes().len() as u32,
//                 std::ptr::null(),
//                 privk.as_ptr(),
//                 std::ptr::null(),
//             ) {
//                 println!("res {res}");
//                 if res != LIBSSH2_ERROR_EAGAIN {
//                     rc = res;
//                     break;
//                 }
//             }
//         }

//         print_error(session);
//         /////////////
//         ///
//         ///
//         ///
//         ///
//         if rc != 0 {
//             println!("error auth {}", rc);
//             return;
//         }
//         // }

//         let session_n = CString::new("session").expect("CString::new failed");
//         let cmd = CString::new("ls -l").expect("CString::new failed");

//         //open channel
//         let channel = libssh2_channel_open_ex(
//             session,
//             session_n.as_ptr(),
//             7,
//             LIBSSH2_CHANNEL_WINDOW_DEFAULT,
//             LIBSSH2_CHANNEL_PACKET_DEFAULT,
//             std::ptr::null(),
//             0,
//         );

//         if channel == std::ptr::null_mut() {
//             println!("frcgvkuyguyg");
//         }
//         print_error(session);

//         //request pyt
//         // let term: CString = CString::new("vanilla").unwrap();

//         // let pty  = libssh2_channel_request_pty_ex(channel, term.as_ptr(), term.as_bytes().len() as u32, std::ptr::null(), 0, 80, 24, 0, 0);

//         let s = CString::new("exec").unwrap();
//         let command = CString::new("uptime").unwrap();

//         let shl = libssh2_channel_process_startup(
//             channel,
//             s.as_ptr(),
//             s.as_bytes().len() as u32,
//             command.as_ptr(),
//             command.as_bytes().len() as u32,
//         );

//         println!("shl  {shl}");
//         print_error(session);

//         // let cmd = CString::new("uptime").expect("CString::new failed");

//         // let a = libssh2_channel_write_ex(channel, 0, cmd.as_ptr(), "uptime".as_bytes().len());

//         println!(" jnjni   {a}");
//         const BUF_SIZE: usize = 1024;
//         let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
//         let buflen = BUF_SIZE;
//         // sleep(Duration::from_secs(3));
//         let b = libssh2_channel_read_ex(channel, 0, buf.as_mut_ptr() as *mut i8, buflen);

//         println!("{b}");
//         println!("{}", String::from_utf8_lossy(&buf));

//         loop {
//             let session_n = CString::new("session").expect("CString::new failed");
//             let cmd = CString::new("ls -l").expect("CString::new failed");

//             //open channel
//             let channel = libssh2_channel_open_ex(
//                 session,
//                 session_n.as_ptr(),
//                 7,
//                 LIBSSH2_CHANNEL_WINDOW_DEFAULT,
//                 LIBSSH2_CHANNEL_PACKET_DEFAULT,
//                 std::ptr::null(),
//                 0,
//             );

//             if channel == std::ptr::null_mut() {
//                 println!("frcgvkuyguyg");
//             }
//             print_error(session);

//             //request pyt
//             // let term: CString = CString::new("vanilla").unwrap();

//             // let pty  = libssh2_channel_request_pty_ex(channel, term.as_ptr(), term.as_bytes().len() as u32, std::ptr::null(), 0, 80, 24, 0, 0);

//             let s = CString::new("exec").unwrap();
//             let command = CString::new("uname -a").unwrap();

//             let shl = libssh2_channel_process_startup(
//                 channel,
//                 s.as_ptr(),
//                 s.as_bytes().len() as u32,
//                 command.as_ptr(),
//                 command.as_bytes().len() as u32,
//             );

//             // let a = libssh2_channel_write_ex(channel, 0, cmd.as_ptr(), "uptime".as_bytes().len());

//             //   println!(" jnjni   {a}");

//             const BUF_SIZE: usize = 1024;
//             let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
//             let buflen = BUF_SIZE;
//             // sleep(Duration::from_secs(3));
//             let b = libssh2_channel_read_ex(channel, 0, buf.as_mut_ptr() as *mut i8, buflen);

//             println!("{b}");
//             println!("{}", String::from_utf8_lossy(&buf));

//             sleep(Duration::from_secs(3));
//         }
//     }

//     // Rest of your program
// }

fn print_error(session: *mut LIBSSH2_SESSION) {
    let mut error_msg = std::ptr::null_mut();
    let mut error_len = 0;

    // Call libssh2_session_last_error
    let error_code =
        unsafe { libssh2_session_last_error(session, &mut error_msg, &mut error_len, 1) };
    if error_code == 0 {
        return;
    }
    println!("errcode:   {error_code}");
    unsafe {
        println!(
            "{:?}",
            String::from_utf8_lossy(std::ffi::CStr::from_ptr(error_msg).to_bytes())
        );
    }
}
