use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>> {

    let mut total = 0;
    
    let mut lines = stdin().lines();

    loop {
        // if we don't have a first line then we can break,
        // otherwise we know the number of lines is divisble by 3
        let Some(l1) = lines.next() else { break; };
        let l1 = l1?;
        let l2 = lines.next().unwrap()?;
        let l3 = lines.next().unwrap()?;
        
        let mut c = '\0';

        'both: for c1 in l1.chars() {
            for c2 in l2.chars() {
                for c3 in l3.chars() {
                    if c1 == c2 && c2 == c3 {
                        c = c1;
                        break 'both;
                    }
                }
            }
        }

        total += match c {
            'a'..='z' => c as u32 - 'a' as u32,
            'A'..='Z' => c as u32 - 'A' as u32 + 26,
            _ => panic!("What...?")
        } + 1;
    }
    
    println!("{total}");
    
    Ok(())
}
