pub fn echo(args: &[&str]) {
    if args.len() < 1 {
        println!("Not enough Arguments");
        return;
    }

    for i in args {
        print!("{i} ");
    }
    println!();
}
