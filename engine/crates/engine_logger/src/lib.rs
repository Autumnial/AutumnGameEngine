pub use colorize::*;
#[allow(unused_macros)]
#[macro_export]
macro_rules! error {
    ($message:expr) => {
        use std::process::exit;
        println!("{}", format!("{}", $message).red().bold());
        exit(0)
    };

    ($message:expr, $($additional:expr),*) =>{
        use std::process::exit;
        print!("{}", format!("{}", $message).red().bold());
        $(print!("{}", format!("{}", $additional).red().bold());)+
        println!();
        exit(0)
    }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! warning {
    ($message:expr) => {
        use std::process::exit;
        println!("{}", format!("{}", $message).yellow().bold());
    };

    ($message:expr, $($additional:expr),*) =>{
        use std::process::exit;
        print!("{}", format!("{}", $message).yellow().bold());
        $(print!("{}", format!("{}", $additional).yellow().bold());)+
        println!();
    }
}

