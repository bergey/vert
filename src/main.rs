use std::env;
use std::io::{self};
use std::fs::File;

fn main() -> io::Result<()> {
    let argv : Vec<String> = env::args().collect();
    if let Some((_, args)) = argv.split_first() {
        if args.len() > 0 {
            let unlocked = io::stdout();
            let mut stdout = unlocked.lock();
            for filename in args {
                let mut f = File::open(filename)?;
                io::copy(&mut f, &mut stdout)?;
            }
        } else {
            let uout = io::stdout();
            let mut stdout = uout.lock();
            let uin = io::stdin();
            let mut stdin = uin.lock();
            io::copy(&mut stdin, &mut stdout)?;
        }
    }
    Ok(())
}
