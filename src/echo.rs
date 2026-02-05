pub fn echo(args: &[String]) {
    if args.is_empty() {
        println!("Not enough Arguments");
        return;
    }

    for i in args {
        print!("{i} ");
    }
    println!();
}
