use crate::core::{add_env, init_setup};

mod core;
mod shell;

fn usage() {
    println!("Usage: envset <PATH_VALUE>");
    println!("\n\nSets or Edit an path variable for the current seleted shell.");
    println!("It will update or add or modify only added shell using this program");
    println!("\nExample:");
    println!("  envset ~/mybin");
    //println!("  envset MY_VAR some_value");
    //println!("\nif no arguments are provided, the env variable will be deleted");
    //println!(
    //  "Note: For PATH variable, it will not delete the whole PATH variable, only remove the not exist paths"
    //);
    //println!("Example: envset PATH");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 1 {
        usage();
        return;
    }

    let env_value = args.get(1).unwrap().clone();
    //let env_value = args.get(2).unwrap().clone();

    init_setup();

    //println!("env_name: {}, env_value: {}", env_name, env_value);

    add_env(env_value);
}
