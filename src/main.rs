
use nix::{unistd::{fork, ForkResult, Pid}};

fn main() {

    println!("Parent: Attempting to fork");
    match unsafe{fork()} {

    Ok(ForkResult::Parent{child} ) => {
    
        println!("Parent : Waiting for child");
        match nix::sys::wait::waitpid(child, None) {
            Ok(_) => println!("Parent: Finished waiting"),
            
            Err(err) => panic!("Parent: Could not wait {}", err)  
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
