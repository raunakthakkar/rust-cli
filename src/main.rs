use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let tool_args: Vec<String> = std::env::args().collect();

    if std::env::args().nth(2).is_none() {
        print!("please provide a filename eg:test.txt");
        std::process::exit(1);
    }

    let file_name = PathBuf::from_str(if tool_args[2].contains(".txt") {
        &tool_args[2]
    } else {
        &tool_args[3]
    });

    match file_name {
        Ok(file) => {
            parse_file(tool_args[2].as_str(), file);
        }
        Err(err) => {
            print!("error in parsing file name {:?}", err);
            std::process::exit(1);
        }
    }
}

fn parse_file(flag: &str, file_name: PathBuf) {
    let file = File::open(file_name);
    // Create a buffer reader
    match file {
        Ok(file) => {
            let reader = BufReader::new(&file);
            match flag {
                "-l" => {
                    print!("{:?}", reader.lines().count());
                }
                "-m" => {
                    let mut word_count = 0;
                    for line in reader.lines() {
                        word_count += line.unwrap().split_whitespace().count();
                    }
                    print!("words {:?}", word_count);
                }
                "-o" => {
                    let mut char_count = 0;
                    for line in reader.lines() {
                        char_count += line.unwrap().split("").count();
                    }
                    print!("characters {:?}", char_count)
                }
                "-p" => {
                    print!("bytes {:?}", file.metadata().unwrap().size())
                }
                _ => {
                    if std::env::args()
                        .nth(2)
                        .is_some_and(|st| st.contains(".txt"))
                    {
                        let mut word_count = 0;
                        let mut char_count = 0;
                        let mut file_lines = 0;
                        for line in reader.lines() {
                            file_lines += 1;
                            word_count += line.as_ref().unwrap().split_whitespace().count();
                            char_count += line.unwrap().split("").count();
                        }
                        print!(
                            "{:?} {:?} {:?} {:?}",
                            file_lines,
                            word_count,
                            char_count,
                            file.metadata().unwrap().size()
                        );
                    }
                }
            }
        }
        Err(err) => {
            print!(" raunak {:?}", err.to_string());
            std::process::exit(1);
        }
    }
}
