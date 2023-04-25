
use nix::{unistd::{fork, ForkResult, Pid}};
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

    println!("{}", syscalls["aaData"][0][1].as_str().unwrap());
    println!("Parent: Attempting to fork");

    match unsafe{fork()} {
    Ok(ForkResult::Parent{child} ) => {
    
        println!("Parent : Waiting for child to call traceme");
        match nix::sys::wait::waitpid(child, None) {
            Ok(_) => {},
            
            Err(err) => panic!("Parent: Could not wait for traceme call {}", err)  
        }


        loop {
            match nix::sys::ptrace::syscall(child, None) {
                Ok(_) => {},
                Err(_) => panic!("Could not ptrace for syscall")
            }
            
            match nix::sys::wait::waitpid(child, None) {
                Ok(_) => {},
                
                Err(err) => panic!("Parent: Could not wait {}", err)  
            }

            match nix::sys::ptrace::getregs(child) {
                Ok(regs) => {
                    println!("r15 {}",regs.r15);
                    println!("rax {}",regs.rax);
                    println!("rbx {}",regs.rbx);
                    println!("gs {}",regs.gs);
                },

                Err(_) => { 
                    println!("Parent: Child exited");
                    return;
                }
            };
        }
    }



    Ok(ForkResult::Child) => {
        match nix::sys::ptrace::traceme() {
            Ok(_) => println!("Child: Trace succcessful"),
            
            Err(err) => panic!("Child: Trace failed with error {}", err)           
        }
        
        { 
            exec::execvp("cat", &["cat","/etc/hosts"]);        
        }       
    }
    
    Err(_) => panic!("Parent: Fork failed")

    }


}
