
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, write, Pid}};

fn main() {

    let childpid : Pid;

    match unsafe{fork()} {
        Ok(ForkResult::Parent{child} ) => childpid = child,

        Ok(ForkResult::Child) => {
            match nix::sys::ptrace::traceme() {
                Ok(_) => println!("Trace succcessful"),
                
                Err(err) => panic!("Trace failed with error {}", err)           
            }

            unsafe { libc::_exit(0) }       
        }


        Err(_) => panic!("Fork failed")
     }
}
