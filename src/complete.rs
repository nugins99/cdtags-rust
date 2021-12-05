use crate::config;
use crate::config::Tag;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use std::fs::{self};
use std::io;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

fn path_to_string(path: &PathBuf) -> String {
    return path.as_os_str().to_os_string().into_string().unwrap();
}

fn component_to_string(path: &Component) -> String {
    return path.as_os_str().to_os_string().into_string().unwrap();
}

fn list_subdirs(path: &Path) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    for dentry in fs::read_dir(path).expect("Unable to read dir.") {
        let dentry = dentry.expect("Bad entry.");
        if dentry.path().is_dir() {

            let dentry_string = dentry.path().file_name().expect("...").to_os_string().into_string().unwrap();
            vec.push(dentry_string);
        }
    }

    return vec;
}

pub fn complete_sub_command() -> App<'static, 'static> {
    return SubCommand::with_name("complete")
        .about("complete tags")
        .arg(
            Arg::with_name("pattern")
                .takes_value(true)
                .help("Pattern to match"),
        );
}

pub fn complete_absolute_path(
    path: &Path,
    prefix: &String,
    replacement: &String,
) -> io::Result<()> {
    if path.is_dir() {
        let entries = list_subdirs(path);

        for entry in entries
        {
            println!("Entry: {}", entry);
        }

        for dentry in fs::read_dir(path)? {
            let dentry = dentry?;
            let dentry_string = component_to_string(&dentry.path().components().last().unwrap());
            
            if dentry_string.find(prefix) == Some(0) {
                if prefix.is_empty()
                {
                    println!("==> {}", path_to_string(&dentry.path()));
                }
                else 
                {
                    let mut new_path = PathBuf::from(replacement);
                    new_path.push(dentry_string);
                    println!("{}", path_to_string(&new_path));
                }
            }
        }
    } else {
        let parent = path.parent();
        if let Some(parent) = parent {
            let parent = PathBuf::from(parent);
            let leaf = path.file_name().unwrap().to_str().expect("Expecting a str");
            println!("Leaf: {:?}", leaf);
            if parent.is_dir() {
                for dentry in fs::read_dir(&parent)? {
                    let dentry = dentry?;
                    let dentry_leaf = dentry.file_name();
                    if dentry_leaf.clone().into_string().unwrap().starts_with(leaf) {
                        let full_path = parent.clone().push(&dentry_leaf);
                        println!("--> {:#?}", full_path);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_prefix(path: &PathBuf) -> String {
    let mut c = path.components();
    //let first_component = c.next().expect("Empty path?");
    let first_component = c.nth(0).expect("Empty path?");
    //println!("{}", first_component);
    return component_to_string(&first_component);
}

pub fn complete_tags(path: &PathBuf) -> Option<Tag> {
    let cfg = config::parse_config();
    let prefix = get_prefix(path);

    for element in cfg {
        if element.tag == prefix {
            println!("{}", element.tag);
            return Some(element);
        }

        if element.tag.find(&prefix) == Some(0) {
            println!("{}", element.tag);
        }
    }
    return None;
}

fn complete_relative_path(path: &Path) {
    let parent = std::env::current_dir().expect("Unable to get cwd");
    let prefix = path
        .as_os_str()
        .to_os_string()
        .into_string()
        .expect("Unable to convert prefix to stirng.");
    let replacement = String::from("");
    println!("complete relative: {:#?}; {}", parent, prefix);
    let _ = complete_absolute_path(&parent, &prefix, &replacement);
}

pub fn cmd_complete(matches: ArgMatches) {
    let tag = matches.value_of("pattern").unwrap().to_string();
    let path = PathBuf::from(tag.clone());
    if !tag.is_empty() {
        println!("Tag: {}", tag);
        if path.is_absolute() {
            println!("Path: {:#?}", path);
            let prefix = String::from("");
            let replacement = String::from("");
            let _ = complete_absolute_path(&path, &prefix, &replacement);
            return;
        }

        if path.components().count() == 1 {
            // path is something like "Movies"
            complete_tags(&path);
            complete_relative_path(&path);
        } else {
            // path is something like "Movies/IronMan"
            let _completed_tags = complete_tags(&path);
            //complete_tag_based_relative_path(&path);
            println!("Not implemented.");
        }
    }
}
