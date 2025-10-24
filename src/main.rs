use nix::sched::{CloneFlags, clone};
use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid, getppid};
use std::process::{Command, exit};

fn run() -> isize {
    nix::sched::unshare(CloneFlags::CLONE_NEWUTS)
        .map_err(|e| format!("unshare failed: {}", e))
        .unwrap();
    let _ = Command::new("/bin/bash")
        .args(["-i"])
        .spawn()
        .unwrap()
        .wait()
        .expect("failed to execute process");
    0
}

fn main() {
    let pid = unsafe { fork() };

    match pid.expect("Fork Failed: Unable to create child process!") {
        Child => {
            println!(
                "Hello from child process with pid: {} and parent pid:{}",
                getpid(),
                getppid()
            );
            run();
        }

        Parent { child } => {
            wait();
            println!(
                "Hello from parent process with pid: {} and child pid:{}",
                getpid(),
                child
            );
        }
    }
}
