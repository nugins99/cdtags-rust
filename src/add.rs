use crate::config;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use std::path::PathBuf;

pub fn cmd_add(matches: ArgMatches) {
    let tag = matches.value_of("tag").unwrap();
    let path = matches.value_of("path").unwrap();
    println!(
        "Add: {} -> {}",
        matches.value_of("tag").unwrap(),
        matches.value_of("path").unwrap()
    );
    let mut tags = config::parse_config();
    let new_tag = config::Tag {
        tag: String::from(tag),
        path: PathBuf::from(path),
    };
    tags.push(new_tag);
    config::write_tags(tags);
}

pub fn add_sub_command() -> App<'static, 'static> {
    let cmd = SubCommand::with_name("add")
        .about("add tag")
        .arg(
            Arg::with_name("tag")
                .takes_value(true)
                .help("Tag to apply to path."),
        )
        .arg(
            Arg::with_name("path")
                .takes_value(true)
                .help("Path to apply to tag"),
        );
    return cmd;
}
