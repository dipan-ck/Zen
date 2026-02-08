use std::{env, io};

pub fn cd(arguments: &Vec<String>) -> Result<(), io::Error> {
    env::set_current_dir(arguments[0].trim())?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use std::{env, fs};

//     use crate::cd::cd;

//     #[test]
//     fn cd_success_test() {
//         let original_curr_dir = env::current_dir().unwrap();

//         let temp_base = env::temp_dir();
//         let test_dir = temp_base.join("cd_test_dir_rust");

//         // ensure clean state
//         let _ = fs::remove_dir(&test_dir);
//         fs::create_dir(&test_dir).unwrap();

//         let mut changed_current_dir = String::new();

//         cd(test_dir.to_str().unwrap(), &mut changed_current_dir).unwrap();

//         //pwd should change to the test dir path
//         assert_eq!(test_dir.to_string_lossy(), changed_current_dir);
//         assert_eq!(env::current_dir().unwrap(), test_dir);

//         // cleanup
//         env::set_current_dir(&original_curr_dir).unwrap();
//         fs::remove_dir(&test_dir).unwrap();
//     }
//     #[test]
//     fn no_dir_found_cd_test() {
//         let mut current_path = String::from("unchanged");

//         let result = cd("/this/path/does/not/exist", &mut current_path);

//         // 1. It must fail
//         assert!(result.is_err());

//         // 2. State must NOT change
//         assert_eq!(current_path, "unchanged");
//     }
// }
