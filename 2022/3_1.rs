use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>> {

    let mut total = 0;
    
    for l in stdin().lines() {
        let l = l?;
        let (s1, s2) = l.split_at(l.len() / 2);

        let mut c = '\0';

        'both: for c1 in s1.chars() {
            for c2 in s2.chars() {
                if c1 == c2 {
                    c = c1;
                    break 'both;
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
