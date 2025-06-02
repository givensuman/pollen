use colored::Colorize;

pub fn error<S: AsRef<str>>(message: S) {
    println!("{}", Colorize::red(message.as_ref()));
    std::process::exit(1);
}

pub fn warning<S: AsRef<str>>(message: S) {
    println!("{}", Colorize::yellow(message.as_ref()));
}

pub fn success<S: AsRef<str>>(message: S) {
    println!("{}", Colorize::green(message.as_ref()));
}
