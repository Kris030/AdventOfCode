use std::{io::stdin, error::Error, vec};

fn main() -> Result<(), Box<dyn Error>> {

    let mut s = vec![];
    for l in stdin().lines() {
        let l = l?;
        if l.is_empty() {
            break;
        }

        s.push(l);
    }
    let last_line = s.remove(s.len() - 1);
    let cols = (last_line.len() - 2) / 4 + 1;

    let mut ss = vec![Vec::new(); cols];

    for s in s.iter().rev() {
        let mut chrs = s.chars().skip(1);
        for i in 0..cols {
            let c = chrs.nth(0).unwrap();
            let _ = chrs.nth(2);

            if c != ' ' {
                ss[i].push(c);
            }
        }
    }

    let mut buff = Vec::new();
    for l in stdin().lines() {
        
        let l = l?;
        let mut cs = l.chars().rev();
        
        let to = cs.nth(0).unwrap() as usize - '0' as usize - 1;

        cs.nth(3);

        let from = cs.nth(0).unwrap() as usize - '0' as usize - 1;

        cs.nth(5);        
        cs.nth_back(4);
        
        let n = cs.rev().collect::<String>().parse()?;

        for _ in 0..n {
            let f = ss[from].pop().unwrap();
            buff.push(f);
        }

        while let Some(c) = buff.pop() {
            ss[to].push(c);
        }
    }

    for s in ss {
        print!("{}", s.last().unwrap());
    }
    println!("");

    Ok(())
}
