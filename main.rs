use std::{
    env::args,
    fs::File,
    io::{stdin, stdout, BufRead, Read, Write},
};
fn eval(buf: &mut String, cells: &mut Vec<u32>, ptr: &mut usize, t: &mut dyn BufRead) {
    let mut loop_end = -1i128;
    for c in buf.char_indices() {
        if loop_end != -1 && c.0 as i128 <= loop_end {
            continue;
        }
        loop_end = -1;
        match c.1 {
            '+' => {
                let cell = *cells.get(*ptr).unwrap();
                *(cells.get_mut(*ptr).unwrap()) = cell + 1;
            }
            '-' => {
                let cell = *cells.get(*ptr).unwrap();
                *(cells.get_mut(*ptr).unwrap()) = cell - 1;
            }
            '<' => {
                if *ptr <= 0 {
                    panic!();
                }
                *ptr -= 1;
            }
            '>' => {
                *ptr += 1;
                while cells.len() <= *ptr {
                    cells.push(0u32)
                }
            }
            '.' => {
                let cell = *cells.get(*ptr).unwrap();
                print!("{}", char::from_u32(cell).unwrap());
                stdout().flush().unwrap();
            }
            ',' => {
                let mut ib = [0u8];
                t.read_exact(&mut ib).unwrap();
                *(cells.get_mut(*ptr).unwrap()) = ib[0] as u32;
            }
            '[' => {
                let mut bc = 1;
                for c2 in buf[c.0 + 1..buf.len()].char_indices() {
                    match c2.1 {
                        '[' => bc += 1,
                        ']' => bc -= 1,
                        _ => continue,
                    }
                    if bc < 0 {
                        panic!();
                    }
                    if bc <= 0 {
                        loop_end = (c2.0 + c.0 + 1) as i128;
                        break;
                    }
                }
                if loop_end == -1 {
                    panic!();
                }
                while (*cells.get(*ptr).unwrap()) != 0 {
                    eval(
                        &mut String::from(&buf[c.0 + 1..loop_end as usize]),
                        cells,
                        ptr,
                        t,
                    );
                }
                continue;
            }
            ']' => panic!(),
            _ => continue,
        }
    }
}
fn main() {
    let t = stdin();
    let mut cells = vec![0u32; 4096];
    let mut ptr = 0usize;
    let mut file = String::new();
    if args().len() < 2 {
        print!("File to interpret:");
        stdout().flush().unwrap();
        t.read_line(&mut file).unwrap();
        file=file.trim().to_string();
    } else {
        let mut atmp=true;
        for i in args() {
            if atmp {atmp=false;continue;}
            file += &(i + " ");
        }
        file = file[0..file.len() - 1].to_string();
    }
    {
        let mut buf = String::new();
        File::open(file).unwrap().read_to_string(&mut buf).unwrap();
        {
            eval(&mut buf, &mut cells, &mut ptr, &mut t.lock());
        }
    }
}
