// use shlex;
use std::io::Result;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Output};
// use home;

/// This describes a remote machine that we can connect to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Remote {
    pub addr: String,
    pub port: u16,
}

/// This is solely implemented for the `Command` type.
/// It adds three methods which allow the "command" type
/// to behave the same on a remote machine as it does on a local.
pub trait RemoteCommand {
    /// Acts identically to the `output` method on `Command`,
    /// except that it runs the command on the remote machine.
    fn remote_output(&mut self, remote: &Remote) -> Result<Output>;

    /// Acts identically to the `spawn` method on `Command`,
    /// except that it runs the command on the remote machine.
    fn remote_spawn(&mut self, remote: &Remote) -> Result<Child>;

    /// Acts identically to the `status` method on `Command`,
    /// except that it runs the command on the remote machine.
    fn remote_status(&mut self, remote: &Remote) -> Result<ExitStatus>;
}

fn reconstruct_ssh_command(remote: &Remote, command: &Command) -> Command {
    let mut cmd = Command::new("ssh");
    cmd.arg("-p").arg(remote.port.to_string());
    cmd.arg(remote.addr.clone());

    // Check whether ~/.ssh/cs6991/cs6991-id exists
    let path: PathBuf = [
        home::home_dir().unwrap().to_str().unwrap(),
        ".ssh",
        "cs6991",
        "cs6991-id",
    ]
    .iter()
    .collect();
    if path.is_file() {
        cmd.arg("-i");
        cmd.arg(path);
    }

    cmd.arg("--");
    cmd.arg(command.get_program());
    cmd.args(command.get_args());

    cmd
}

impl RemoteCommand for Command {
    fn remote_output(&mut self, remote: &Remote) -> Result<Output> {
        let mut cmd = reconstruct_ssh_command(remote, self);
        cmd.output()
    }

    fn remote_spawn(&mut self, remote: &Remote) -> Result<Child> {
        let mut cmd = reconstruct_ssh_command(remote, self);
        let cmd = cmd.stdin(std::process::Stdio::piped());
        let cmd = cmd.stdout(std::process::Stdio::piped());
        cmd.spawn()
    }

    fn remote_status(&mut self, remote: &Remote) -> Result<ExitStatus> {
        let mut cmd = reconstruct_ssh_command(remote, self);
        cmd.status()
    }
}

/// This takes a line of input, and splits it into a vector of commands.
pub fn parse_line(s: &str) -> Option<Vec<Vec<String>>> {
    let mut cmd = vec![];
    let mut cmds = vec![];
    for token in shlex::split(s)? {
        if token == ";" && !cmd.is_empty() {
            cmds.push(cmd);
            cmd = vec![];
        } else {
            let starts_with_split = token.starts_with(';');
            let ends_with_split = token.ends_with(';');
            let token = token.trim_matches(';').to_string();
            if starts_with_split && !cmd.is_empty() {
                cmds.push(cmd);
                cmd = vec![];
            }
            if !token.is_empty() {
                cmd.push(token);
            }
            if ends_with_split && !cmd.is_empty() {
                cmds.push(cmd);
                cmd = vec![];
            }
        }
    }
    if !cmd.is_empty() {
        cmds.push(cmd);
    }
    Some(cmds)
}

/// write tests for parse_line
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("echo hello"),
            Some(vec![vec!["echo".to_string(), "hello".to_string()]])
        );
        assert_eq!(
            parse_line("echo hello; echo world"),
            Some(vec![
                vec!["echo".to_string(), "hello".to_string()],
                vec!["echo".to_string(), "world".to_string()]
            ])
        );
        assert_eq!(
            parse_line("echo hello; echo world;"),
            Some(vec![
                vec!["echo".to_string(), "hello".to_string()],
                vec!["echo".to_string(), "world".to_string()]
            ])
        );
        assert_eq!(
            parse_line("echo hello; echo world; "),
            Some(vec![
                vec!["echo".to_string(), "hello".to_string()],
                vec!["echo".to_string(), "world".to_string()]
            ])
        );
        assert_eq!(
            parse_line("echo hello; echo world; echo"),
            Some(vec![
                vec!["echo".to_string(), "hello".to_string()],
                vec!["echo".to_string(), "world".to_string()],
                vec!["echo".to_string()]
            ])
        );
        assert_eq!(
            parse_line("echo hello ; echo world; echo;"),
            Some(vec![
                vec!["echo".to_string(), "hello".to_string()],
                vec!["echo".to_string(), "world".to_string()],
                vec!["echo".to_string()]
            ])
        );
        assert_eq!(
            parse_line("echo hello ;echo world; echo ;"),
            Some(vec![
                vec!["echo".to_string(), "hello".to_string()],
                vec!["echo".to_string(), "world".to_string()],
                vec!["echo".to_string()]
            ])
        );
        assert_eq!(
            parse_line("echo hello; echo world; echo ; "),
            Some(vec![
                vec!["echo".to_string(), "hello".to_string()],
                vec!["echo".to_string(), "world".to_string()],
                vec!["echo".to_string()]
            ])
        );
        assert_eq!(
            parse_line("echo hello; echo world; echo ; ;"),
            Some(vec![
                vec!["echo".to_string(), "hello".to_string()],
                vec!["echo".to_string(), "world".to_string()],
                vec!["echo".to_string()]
            ])
        );
        assert_eq!(
            parse_line("echo 'hello; yeet'; echo world; echo ; ;"),
            Some(vec![
                vec!["echo".to_string(), "hello; yeet".to_string()],
                vec!["echo".to_string(), "world".to_string()],
                vec!["echo".to_string()]
            ])
        );
    }
}
