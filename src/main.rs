use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write, Read};
use std::os::unix::io::AsRawFd;
// use std::convert::TryInto;

extern crate termios;
extern crate term_size;

// TODO currently unused, but we'll need arguments again, presently
fn parse_args(args : &mut VecDeque<String>) -> Option<u64> {
    if let Some(a) = args.get(0) {
        if &a[0..2] == "-n" {
            let n_lines = (&a[2..]).parse::<u64>().unwrap();
            args.pop_front();
            return Some(n_lines);
        } else {
            return None
        }
    } else {
        return None
    }
}

fn setup_term() -> File {
    use termios::*;

    let tty = File::open("/dev/tty").unwrap();
    let mut term = Termios::from_fd(tty.as_raw_fd()).unwrap(); // Unix only
    // Unset canonical mode, so we get characters immediately
    // Disable local echo
    term.c_lflag &= !(ICANON | ECHO);
    tcsetattr(tty.as_raw_fd(), TCSADRAIN, &term).unwrap();
    tty
}

fn reset_term() {
    use termios::*;

    let tty = File::open("/dev/tty").unwrap();
    let mut term = Termios::from_fd(tty.as_raw_fd()).unwrap(); // Unix only
    term.c_lflag |= ICANON | ECHO;
    tcsetattr(tty.as_raw_fd(), TCSADRAIN, &term).unwrap();
}


fn pager<R : Read, W : Write>(from :  &mut R, to : &mut W ) {
    let mut reader = BufReader::new(from);
    let mut writer = BufWriter::new(to);
    let mut buffer = vec![];

    let mut lines : usize = 0;


    let page_lines : usize = match term_size::dimensions() {
        Some((_w, h)) => h,
        None => 30
    };

    'files: while reader.read_until(b'\n', &mut buffer).unwrap() > 0 {
        writer.write(&buffer).unwrap();
        lines += 1;
        buffer.clear();

        let tty = setup_term();
        if lines % page_lines == 0 {
            // wait for input
            writer.flush().unwrap();
            'user: for byte in tty.bytes() {
                match byte.unwrap() {
                    b' ' => break 'user,
                    b'q' => {   // TODO should probably exit program, not only current file
                        break 'files;
                    }
                    _ => ()
                }
            }
        }
    }

}

fn main() -> io::Result<()> {
    let mut args : VecDeque<String> = env::args().collect();
    args.pop_front();           // discard command name
    if args.len() > 0 {
        let stdout = io::stdout();
        let mut out_lock = stdout.lock();
        for filename in args {
            let mut f = File::open(filename)?;
            pager(&mut f, &mut out_lock)
        }
    } else {
        let stdout = io::stdout();
        let mut out_lock = stdout.lock();
        let stdin = io::stdin();
        let mut in_lock = stdin.lock();
        pager(&mut in_lock, &mut out_lock)
    }
    reset_term();
    Ok(())
}
