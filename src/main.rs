use std::env;
use std::io::{self, Read};
use std::fs::File;

fn main() -> io::Result<()> {
    let argv : Vec<String> = env::args().collect();
    let mut buffer = String::new();
    if let Some((_, args)) = argv.split_first() {
        if args.len() > 0 {
            for filename in args {
                let mut f = File::open(filename)?;
                f.read_to_string(&mut buffer)?;
                println!("{}", buffer);
            }
        } else {
            io::stdin().read_to_string(&mut buffer)?;
            println!("{}", buffer);
        }
    }
    Ok(())
}
