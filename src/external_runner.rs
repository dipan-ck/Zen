use std::process::Command;

pub fn run_enternal_prog(args: &[&str]) {
    let mut child = match Command::new(args[0]).args(&args[1..]).spawn() {
        Ok(data) => data,
        Err(_) => {
            println!("{}: not found", args[0]);
            return;
        }
    };

    let _ = child.wait();
}
