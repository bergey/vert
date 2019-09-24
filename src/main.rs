use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, Write, Read};
use std::os::unix::io::AsRawFd;
// use std::convert::TryInto;

#[cfg(unix)]
extern crate termios;
#[cfg(unix)]
extern crate term_size;

#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
use kernel32::{GetStdHandle, GetConsoleMode, SetConsoleMode};
#[cfg(windows)]
use winapi::um::wincon::{ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT};
#[cfg(windows)]
use winapi::um::winbase::{STD_INPUT_HANDLE};

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

#[cfg(unix)]
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

#[cfg(windows)]
fn setup_term() { unsafe {
    let term = GetStdHandle(STD_INPUT_HANDLE); // TODO PIPE case
    let mut mode = 0;
    let ok = GetConsoleMode(term, &mut mode);
    if ok == 0 {
        panic!("Could not get Console Mode");
    }
    let ok = SetConsoleMode(term, mode & !ENABLE_ECHO_INPUT & !ENABLE_LINE_INPUT);
    if ok == 0 {
        panic!("Could not set Console Mode");
    }
}}

#[cfg(unix)]
fn reset_term() {
    use termios::*;

    let tty = File::open("/dev/tty").unwrap();
    let mut term = Termios::from_fd(tty.as_raw_fd()).unwrap(); // Unix only
    term.c_lflag |= ICANON | ECHO;
    tcsetattr(tty.as_raw_fd(), TCSADRAIN, &term).unwrap();
}

#[cfg(windows)]
#[inline(always)]
fn reset_term(_: &mut ()) { unsafe {
    let term = GetStdHandle(STD_INPUT_HANDLE); // TODO PIPE case
    let mut mode = 0;
    let ok = GetConsoleMode(term, &mut mode);
    if ok == 0 {
        panic!("Could not get Console Mode");
    }
    let ok = SetConsoleMode(term, mode | ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT);
    if ok == 0 {
        panic!("Could not set Console Mode");
    }
}}


fn pager<R : Read, W : Write>(reader :  &mut R, writer : &mut W ) {
    let mut buffer = [0; 1024];

    let (term_columns, term_lines) = match term_size::dimensions() {
        Some((w, h)) => (w, h-1),
        None => (80, 30)
    };

    let mut want_lines = term_lines;  // start with a full page; count down
    let mut columns = term_columns;   // for consistency, count down

    'chunks: while let Ok(size) = reader.read(&mut buffer) {
        if size == 0 {
            break;
        }
        let mut write_start = 0;    // start of next write
        let mut point = 0;          // next char when counting lines

            writer.flush().unwrap();

        loop {
            // find a subrange with the right number of lines
            while want_lines > 0 {
                let c = buffer[point];
                if c == b'\n' {
                    want_lines -= 1;
                    columns = term_columns;
                    point += 1;
                }
                else if columns == 0 {
                    // visual line, wrapped by terminal
                    want_lines -= 1;
                    columns = term_columns;
                    // don't increment point; this char needs to start the next line
                } else {
                    point += 1;
                    columns -= 1;
                }
                if point == size {
                    writer.write(&buffer[write_start..point]).unwrap();
                    continue 'chunks
                }
            }

            writer.write(&buffer[write_start..point]).unwrap();
            writer.flush().unwrap();
            write_start = point;

            let tty = setup_term();
            'user: for byte in tty.bytes() {
                match byte.unwrap() {
                    b' ' => {
                        want_lines = term_lines;
                        break
                    },
                    b'\n' => {
                        want_lines = 1;
                        break
                    }
                    b'\r' => { // RETURN on Windows
                        want_lines = 1;
                        break
                    }
                    b'q' | 27 => {
                        break 'chunks;
                    }
                    _ => (),
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
