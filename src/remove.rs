use crate::config;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;


pub fn cmd_remove(matches: ArgMatches) {
    let tag = matches.value_of("tag").unwrap();
    let tags = config::parse_config();
    let tags_iter = tags.into_iter();
    let new_tags = tags_iter.filter(|x| x.tag != tag);
    config::write_tags(new_tags.collect());
}

pub fn remove_sub_command() -> App<'static, 'static> {
    return SubCommand::with_name("remove").about("remove tag").arg(
        Arg::with_name("tag")
            .takes_value(true)
            .help("The tag to remove."),
    );
}