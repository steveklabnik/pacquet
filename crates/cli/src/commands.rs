use clap::{Arg, Command};

fn add_package_command() -> Command {
    Command::new("add").about("Add a package").arg(Arg::new("package"))
}

pub fn get_commands() -> Command {
    Command::new("pacquet")
        .bin_name("pacquet")
        .version("alpha")
        .author("Yagiz Nizipli")
        .arg_required_else_help(true)
        .subcommand(add_package_command())
}
