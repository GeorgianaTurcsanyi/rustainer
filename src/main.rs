use libc::c_int;
use nix::libc;
use nix::sched::clone;
use nix::sys::signal::Signal::SIGCHLD;
use std::env::args;
use std::process::{Command, exit};

const STACK_SIZE: usize = 1024 * 1024; //1mb

fn run() -> isize {
    let args: Vec<String> = args().collect();
    if args.len() <= 2 {
        eprintln!("specify command to run")
    }

    // let cmd = &args[2];
    // let cmd_args = &args[3..];
    // let _ = Command::new(cmd)
    //     .args(cmd_args)
    //     .spawn()
    //     .unwrap()
    //     .wait()
    //     .expect("failed to execute process");

    println!("a child process");
    0
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("pass them arguments");
        exit(1);
    }
    unsafe {
        match args[1].as_str() {
            "run" => {
                let stack: &mut [u8; STACK_SIZE] = &mut [0; STACK_SIZE];
                let signal: Option<c_int> = Some(SIGCHLD as c_int);

                let child_callback: nix::sched::CloneCb = Box::new(run);

                let cp = clone(
                    child_callback,
                    stack,
                    nix::sched::CloneFlags::CLONE_NEWUTS,
                    signal,
                )
                .expect("should be able to run");

                nix::sys::wait::waitpid(cp, None).expect("cannot wait");
            }
            _ => {
                panic!("whyyy")
            }
        }
    }
    println!("hiii")
}
