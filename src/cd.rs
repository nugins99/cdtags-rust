use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use std::path::Path;
use std::path::PathBuf;

use crate::config;

pub fn cd_sub_command() -> App<'static, 'static> {
    return SubCommand::with_name("cd")
        .about("cd into directory with tag")
        .arg(
            Arg::with_name("tag")
                .takes_value(true)
                .help("The tag to cd into"),
        );
}

pub fn cmd_cd(matches: ArgMatches) {
    let tag = matches.value_of("tag").unwrap();

    if !tag.is_empty() {
        let ch = tag.chars().nth(0).unwrap();
        // Return to $OLDPWD
        if tag == "-" {
            println!("{}", std::env::var("OLDPWD").unwrap());
        }

        if ch == '/' || ch == '.' || Path::new(tag).is_dir() {
            println!("{}", tag);
        }
        // Ok - now we have:
        // a
        // a/b
        // But neither resolve to a directory.
        let cfg = config::parse_config();
        // let "tag" be a relative path
        let path = PathBuf::from(tag);
        let mut path_it = path.components();
        let first_element = path_it.next().expect("Unable to split line");
        let fe = first_element
            .as_os_str()
            .to_str()
            .expect("Unable to get first element as string.");
        println!("First elment: {}", fe);
        let mut remaining_components = PathBuf::new();
        for component in path_it {
            remaining_components.push(component);
        }
        //println!("Remaining elements: {}", remaining_components.into_os_string().into_string().unwrap());

        let found_tag = cfg.iter().find(|e| e.tag == fe);
        match found_tag {
            None => println!("{}", tag), // print current "tag".
            Some(element) => (|| {
                let mut p = element.path.clone();
                for rc in remaining_components.components() {
                    p.push(rc);
                }

                println!("{}", p.clone().into_os_string().into_string().unwrap())
            })(),
        }
    }
}
