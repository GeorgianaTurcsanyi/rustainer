use libc::c_int;
use nix::libc;
use nix::sched::clone;
use nix::sys::signal::Signal::{SIGCHLD, SIGHUP};
use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid, getppid};
use std::env::args;
use std::process::{Command, exit};

const STACK_SIZE: usize = 10 * 1024 * 1024; // 10mb


fn run() -> isize {
    let _ = Command::new("/bin/bash")
        .args(["-i"])
        .spawn()
        .unwrap()
        .wait()
        .expect("failed to execute process");
    0
}

fn main() {
    // This yells illegal instruction
    unsafe {
        let stack: &mut [u8; STACK_SIZE] = &mut [0; STACK_SIZE];
        let signal: Option<c_int> = Some(SIGCHLD as c_int);

        let child_callback: nix::sched::CloneCb = Box::new(run);

        let _ = clone(
            child_callback,
            stack,
            nix::sched::CloneFlags::CLONE_NEWUTS,
            signal,
        )
        .expect("should be able to run");

        // nix::sys::wait::waitpid(cp, None).expect("cannot wait");
    }

    // This works, but, as expected, no isolation
    // let pid = unsafe { fork() };
    //
    // match pid.expect("Fork Failed: Unable to create child process!") {
    //     Child => println!(
    //         "Hello from child process with pid: {} and parent pid:{}",
    //         getpid(),
    //         getppid()
    //     ),
    //     Parent { child } => {
    //         wait();
    //         println!(
    //             "Hello from parent process with pid: {} and child pid:{}",
    //             getpid(),
    //             child
    //         );
    //     }
    // }
}
