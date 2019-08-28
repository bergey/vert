use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write, Read};
use std::os::unix::io::AsRawFd;

extern crate termios;

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

    // TODO don't open / close device every time
    let tty = File::open("/dev/tty").unwrap();
    let mut term = Termios::from_fd(tty.as_raw_fd()).unwrap(); // Unix only
    // let mut term = termios::tcgetattr(tty.as_raw_fd).unwrap(); // Unix only
    // Unset canonical mode, so we get characters immediately
    // Disable local echo
    term.c_lflag &= !(ICANON | ECHO);
    tcsetattr(tty.as_raw_fd(), TCSADRAIN, &term).unwrap();
    tty
}

fn copy_bounded<R : Read, W : Write>(from :  &mut R, to : &mut W, max_lines : u64 ) {
    let mut reader = BufReader::new(from);
    let mut writer = BufWriter::new(to);
    let mut buffer = vec![];

    let mut lines : u64 = 0;
    const PAGE_LINES : u64 = 30; // TODO from terminal

    'files: while lines < max_lines && reader.read_until(b'\n', &mut buffer).unwrap() > 0 {
        writer.write(&buffer).unwrap();
        lines += 1;
        buffer.clear();

        if lines % PAGE_LINES == 0 {
            // wait for input
            writer.flush().unwrap();
            'user: loop {
                let tty = setup_term();
                for byte in tty.bytes() {
                    match byte.unwrap() {
                        b' ' => break 'user,
                        b'q' => {
                            break 'files;
                        }
                        _ => ()
                    }
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut args : VecDeque<String> = env::args().collect();
    args.pop_front();           // discard command name
    let n_lines = parse_args(&mut args);
    if args.len() > 0 {
        let stdout = io::stdout();
        let mut out_lock = stdout.lock();
        for filename in args {
            let mut f = File::open(filename)?;
            if let Some(max_lines) = n_lines {
                copy_bounded(&mut f, &mut out_lock, max_lines)
            } else {
                io::copy(&mut f, &mut out_lock)?;
            }
        }
    } else {
        let stdout = io::stdout();
        let mut out_lock = stdout.lock();
        let stdin = io::stdin();
        let mut in_lock = stdin.lock();
        if let Some(max_lines) = n_lines {
            copy_bounded(&mut in_lock, &mut out_lock, max_lines)
        } else {
            io::copy(&mut in_lock, &mut out_lock)?;
        }
    }
    Ok(())
}
