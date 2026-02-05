use std::process;

pub fn run_enternal_prog(args: &Vec<String>) {
    let mut child = match process::Command::new(&args[0]).args(&args[1..]).spawn() {
        Ok(data) => data,
        Err(_) => {
            println!("{}: not found", args[0]);
            return;
        }
    };

    let _ = child.wait();
}
