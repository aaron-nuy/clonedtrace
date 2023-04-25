
use nix::{unistd::{fork, ForkResult}};
use std::env;
use json::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    if args.len() < 1 {
        panic!("Not enough arguments, quitting.");
    }

    let js = std::fs::read_to_string("syscalls.json").unwrap();
    let syscalls = parse(&js).unwrap();

    println!("Parent: Attempting to fork");

    match unsafe{fork()} {
    Ok(ForkResult::Parent{child} ) => {
    
        println!("Parent : Waiting for child to call traceme");
        match nix::sys::wait::waitpid(child, None) {
            Ok(_) => {},
            
            Err(err) => panic!("Parent: Could not wait for traceme call {}", err)  
        }

        let mut enterCall = false;
        loop {
            match nix::sys::ptrace::syscall(child, None) {
                Ok(_) => {},
                Err(_) => panic!("Could not ptrace for syscall")
            }
            
            match nix::sys::wait::waitpid(child, None) {
                Ok(_) => {},
                
                Err(err) => panic!("Parent: Could not wait {}", err)  
            }

            if !enterCall {
                match nix::sys::ptrace::getregs(child) {
                    Ok(regs) => {
                        let syscode = regs.orig_rax as usize;
                        println!("{}({},{},{},{},{})", syscalls["aaData"][syscode][1], regs.rdi, regs.rsi, regs.rdx, regs.r10, regs.r8 );
                    },

                    Err(_) => { 
                        println!("Parent: Child exited");
                        return;
                    }
                };
            }
            enterCall = !enterCall;
        }
    }



    Ok(ForkResult::Child) => {
        match nix::sys::ptrace::traceme() {
            Ok(_) => println!("Child: Trace succcessful"),
            
            Err(err) => panic!("Child: Trace failed with error {}", err)           
        }
        
        { 
            exec::execvp(&args[0], &args);           
        }       
    }
    
    Err(_) => panic!("Parent: Fork failed")

    }


}
