use std::env;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let argv : Vec<String> = env::args().collect();
    let mut buffer = String::new();
    if let Some((_, args)) = argv.split_first() {
        if args.len() > 0 {
            println!("{}", args.join(" ")) // TODO files
        } else {
            println!("trying to read from STDIN");
            let l = io::stdin().read_to_string(&mut buffer)?;
            println!("read {} bytes", l);
            println!("{}", buffer);
        }
    }
    Ok(())
}
