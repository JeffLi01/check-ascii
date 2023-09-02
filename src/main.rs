use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::{PathBuf, Path};

use clap::Parser;

#[derive(Parser)]
struct Opts {
    files: Vec<PathBuf>,
}

fn check_ascii(data: &[u8]) -> BTreeMap<usize, u8> {
    let map = data.iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if *b > 127 {
                Some((i, *b))
            } else {
                None
            }
        })
        .collect();
    map
}

fn check_file<P: AsRef<Path> + Debug>(p: P) {
    let mut f = match File::open(&p) {
        Ok(f) => f,
        Err(err) => {
            println!("{:?}: open failed: {}", p, err);
            return;
        },
    };
    let mut data = vec![];
    let len = match f.read_to_end(&mut data) {
        Ok(len) => len,
        Err(err) => {
            println!("{:?}: read failed: {}", p, err);
            return;
        },
    };
    let map = check_ascii(&data);
    if !map.is_empty() {
        print!("{:?}[{} bytes]: ", p, len);
        map.iter().for_each(|(i, b)| print!(", {:x}h@{:x}h", b, i));
    }
}

fn main() {
    let opts = Opts::parse();
    opts.files.iter().for_each(|p| check_file(p));
}
