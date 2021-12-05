use clap::ArgMatches;
use crate::config;

pub fn cmd_list(_matches: ArgMatches) {
    let tags = config::parse_config();
    for tag in tags {
        println!(
            "{}:\n\t{}",
            tag.tag,
            tag.path.into_os_string().into_string().unwrap()
        );
    }
}