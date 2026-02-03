use zen::zen;

fn main() {
    match zen() {
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
        _ => return,
    }
}

// wsl ~ -d Ubuntu-24.04
// cd /mnt/c/users/dipan/desktop/dev/zen
