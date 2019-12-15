use std::process::Command;
extern crate clap; 
use clap::{Arg, App};

fn main() {
    let matches = App::new("homework mypipe")
                          .version("1.0")
                          .author("Souissi Mohamed")
                          .about("Small pipe-like program")
                          .arg(Arg::with_name("in")
                            .long("in")
                            .takes_value(true)
                            .required(true)
                            .requires("out"))
                          .arg(Arg::with_name("out")
                            .help("set the outputFile")
                            .required(true)
                            .takes_value(true)
                            .long("out"))
                          .get_matches();

    if(matches).is_present("in") {
        do_the_job(matches.value_of("in").unwrap().to_string(), matches.value_of("out").unwrap().to_string());
    } else {
        println!("Error while parsing, expected : mypipe --in <input> --out <output>");
    }
}

fn do_the_job(command1: String, command2: String) {
    let fortune = Command::new(command1).output().expect("error while executing command1");
    let message = String::from_utf8_lossy(&fortune.stdout).to_string();
    let cowsay_output = Command::new(command2).arg(message).output().expect("error while executing command2");
    println!("{}", String::from_utf8_lossy(&cowsay_output.stdout));
}