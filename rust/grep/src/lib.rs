use failure::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs

#[derive(Debug, PartialEq)]
enum Flagtype {
    PrintLineNumbers, // -n
    CaseInsensitive,  // -i
    PrintFilenames,   // -l
    MatchEntireLine,  // -x
    InvertedResult,   // -v
}

#[derive(Debug)]
pub struct Flags {
    flags: Vec<Flagtype>,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut r = Vec::new();
        for i in flags.into_iter() {
            if i == &"-n" {
                r.push(Flagtype::PrintLineNumbers)
            } else if i == &"-i" {
                r.push(Flagtype::CaseInsensitive)
            } else if i == &"-l" {
                r.push(Flagtype::PrintFilenames)
            } else if i == &"-x" {
                r.push(Flagtype::MatchEntireLine)
            } else if i == &"-v" {
                r.push(Flagtype::InvertedResult)
            }
        }
        Flags { flags: r }
    }
}

fn line_matches(flags: &Flags, pattern: &str, line_comp: &str) -> bool {
    let mut p = pattern.to_string();
    let mut l = line_comp.to_string();
    if flags.flags.contains(&Flagtype::CaseInsensitive) {
        p = pattern.to_lowercase();
        l = line_comp.to_lowercase();
    }
    let does_match = if flags.flags.contains(&Flagtype::MatchEntireLine) {
        l == p
    } else {
        l.contains(&p)
    };

    if flags.flags.contains(&Flagtype::InvertedResult) {
        !does_match
    } else {
        does_match
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut results = Vec::new();
    println!("{:?}", flags);
    for i in files.iter() {
        match File::open(i) {
            Ok(f) => {
                for (j, x) in BufReader::new(f).lines().enumerate() {
                    match x {
                        Ok(l) => {
                            if line_matches(flags, pattern, &l) {
                                println!("{:?}", l);
                                let print_f = flags.flags.contains(&Flagtype::PrintFilenames);
                                let print_n = flags.flags.contains(&Flagtype::PrintLineNumbers);

                                if print_f {
                                    results.push((*i).to_string());
                                    break;
                                } else if print_n && files.len() == 1 {
                                    results.push(format!("{}:{}", j + 1, l));
                                } else if print_n && files.len() != 1 {
                                    results.push(format!("{}:{}:{}", *i, j + 1, l));
                                } else if !print_n && files.len() != 1 {
                                    results.push(format!("{}:{}", *i, l));
                                } else {
                                    results.push(format!("{}", l));
                                }
                            }
                        }
                        Err(e) => {
                            return Err(Error::from(e));
                        }
                    }
                }
            }
            Err(e) => {
                return Err(Error::from(e));
            }
        }
    }
    Ok(results)
}
