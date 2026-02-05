fn main() {
    if let Err(err) = zen::run() {
        println!("Error ocured: {}", err);
    }
}

// wsl ~ -d Ubuntu-24.04
// cd /mnt/c/users/dipan/desktop/dev/zen
