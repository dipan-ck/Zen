use std::{env, io, path::Path};

pub fn cd(path: &str, current_path: &mut String) -> Result<(), io::Error> {
    let path = Path::new(path);
    match env::set_current_dir(path) {
        Ok(_) => {
            return {
                *current_path = String::from(env::current_dir().unwrap().to_str().unwrap());
                Ok(())
            };
        }
        Err(_) => {
            println!("No such file or directory");
            Ok(())
        }
    }
}
