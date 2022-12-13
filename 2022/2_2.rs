use std::{io::{Read, stdin}, error::Error};

fn main() -> Result<(), Box<dyn Error>> {

    let mut src = String::new();
    stdin().read_to_end(unsafe { src.as_mut_vec() })?;

    let mut total: u32 = 0;
    for l in src.lines() {
        let mut cs = l.chars();
        let opp = cs.nth(0).unwrap();

        let end = cs.nth(1).unwrap();

        let moi = match end {
            // I win
            'Z' => match opp {
                // rock
                'A' => 1,
                // paper
                'B' => 2,
                // scissors
                _ => 0,
            },

            // I lose
            'X' => match opp {
                // rock
                'A' => 2,
                // paper
                'B' => 0,
                // scissors
                _ => 1,
            },

            // draw
            _ => match opp {
                // rock
                'A' => 0,
                // paper
                'B' => 1,
                // scissors
                _ => 2,
            },
        };
        
        total += moi + 1 + match end {
            // I win
            'Z' => 6,
            // I lose
            'X' => 0,
            // draw
            _ => 3,
        };
    }

    println!("{total}");
    
    Ok(())
}
