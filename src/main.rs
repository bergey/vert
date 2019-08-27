use std::env;
/* use std::io::{self, Read}; */

fn main() {
     let argv : Vec<String> = env::args().collect();
     match argv.split_first() {
          Some((_, args)) => println!("{}", args.join(" ")),
          None => println!("Nothing to echo.")
     }
}
