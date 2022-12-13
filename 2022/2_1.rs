use std::{io::{Read, stdin}, error::Error};

fn main() -> Result<(), Box<dyn Error>> {

    let mut src = String::new();
    stdin().read_to_end(unsafe { src.as_mut_vec() })?;

    let mut total: u32 = 0;
    for l in src.lines() {
        let mut cs = l.chars();
        let opp = (cs.nth(0).unwrap() as u32) - ('A' as u32);
        let moi = (cs.nth(1).unwrap() as u32) - ('X' as u32);
        
        total += moi + 1 + match (opp, moi) {
            // I win
            (0, 1) | (1, 2) | (2, 0) => 6,

            // I lose
            (0, 2) | (1, 0) | (2, 1) => 0,

            // draw
            _ => 3,
        };
    }

    println!("{total}");
    
    Ok(())
}
