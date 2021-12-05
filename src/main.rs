extern crate clap;
use clap::{App, ArgMatches, SubCommand};
use std::collections::HashMap;

mod cd;
mod add;
mod remove;
mod config;
mod list;
mod complete;

fn parse_args() -> (String, ArgMatches<'static>) {
    let app = App::new("cdtags")
        .version("1.0")
        .author("Joe Davidson")
        .subcommand(SubCommand::with_name("list").about("list tags"))
        .subcommand(add::add_sub_command())
        .subcommand(remove::remove_sub_command())
        .subcommand(cd::cd_sub_command())
        .subcommand(complete::complete_sub_command());

    let matches = app.get_matches();
    let cmd = matches.subcommand_name().unwrap();
    let submatches = matches.subcommand_matches(cmd).unwrap();
    return (cmd.to_string(), submatches.clone());
}

fn main() {
    // Parse the arguments
    let (cmd, matches) = parse_args();

    // Build dispatch table that maps command names to functions.
    type Callback = fn(ArgMatches);
    let mut cmd_map: HashMap<String, Callback> = HashMap::new();
    cmd_map.insert("list".to_string(), list::cmd_list);
    cmd_map.insert("add".to_string(), add::cmd_add);
    cmd_map.insert("remove".to_string(), remove::cmd_remove);
    cmd_map.insert("cd".to_string(), cd::cmd_cd);
    cmd_map.insert("complete".to_string(), complete::cmd_complete);

    // Run the command that matches the arguments.
    (cmd_map[&cmd])(matches);
}
