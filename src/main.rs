use clap::Parser;

mod core;
mod shell;

/// Sets or Edit an path variable for the current seleted shell.
///
/// It will update or add or modify only added shell using this program.
/// This program will always add the path to every supported shell which are installed now.
/// Note: if shell is installed later then it need to config by manual or using appropriate
/// program command.
#[derive(Parser, Debug)]
#[command(author, version, long_about, about)]
struct Args {
    /// Path to add in shell.
    ///
    /// This will add any directory to the shell.
    /// It will add in all shell so donot use any shell specific path reference.
    #[arg(short, long, default_value = "")]
    path: String,

    /// Toggles add or delete the value
    ///
    /// By default it is false, so path will be added if it is set to `false` then
    /// path will be deleted from the shell env if it is added previously.
    #[arg(id = "remove", short, long)]
    remove_path: bool,

    /// This options used to sync all the path to current shell.
    ///
    /// This toggle add all the previously added paths(only using envset)
    /// current shell. User need to run this while in specific shell.
    /// If already the shell have configs then it will do nothing.
    #[arg(id = "sync", long, short)]
    sync: bool,
}

fn main() {
    let args = Args::parse();

    if args.remove_path {
        todo!("removes a paths from envset config.");
    }
}
