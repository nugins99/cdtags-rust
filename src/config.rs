use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

pub struct Tag {
    pub path: PathBuf,
    pub tag: String,
}

pub fn parse_config() -> Vec<Tag> {
    // Open file...
    let file = File::open("/Users/joe/.config/cdtags/config").expect("Unable to read file.");
    let reader = BufReader::new(file);

    let mut tags: Vec<Tag> = Vec::new();

    for line in reader.lines() {
        let fields = line.unwrap();
        let split = fields.split(",");
        let vec = split.collect::<Vec<&str>>();

        let path = vec[0];
        let tag = vec[1];
        let t = Tag {
            path: PathBuf::from(path),
            tag: String::from(tag),
        };
        tags.push(t);
    }
    return tags;
}

pub fn write_tags(tags: Vec<Tag>) {
    let mut output =
        File::create("/Users/joe/.config/cdtags/config").expect("Unable to open file.");
    for tag in tags {
        let p = tag.path.into_os_string().into_string().unwrap();
        let t = tag.tag;
        writeln!(output, "{},{},", p, t).unwrap();
    }
}
