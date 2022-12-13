use std::{io::stdin, error::Error, collections::VecDeque};

fn main() -> Result<(), Box<dyn Error>> {

    let mut src = String::new();
    stdin().read_line(&mut src)?;
    let mut s = src.chars();

    let mut prev = VecDeque::new();

    const SZ: usize = 14;

    for _ in 0..(SZ - 1) {
        prev.push_back(s.nth(0).unwrap());
    }
    
    let mut index = SZ;
    while let Some(c) = s.nth(0) {
        prev.push_back(c);

        let mut f = false;
        'a: for i in 0..SZ {
            for j in 0..SZ {
                if i != j && prev[i] == prev[j] {
                    f = true;
                    break 'a;
                }
            }
        }

        if !f {
            break;
        }

        index += 1;
        prev.pop_front();
    }

    println!("{}", index);

    Ok(())
}
