use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>> {

    let mut total: u32 = 0;
    for l in stdin().lines() {
        let l = l?;

        let (r1, r2) = l.split_once(',').unwrap();

        let (r1s, r1e) = r1.split_once('-').unwrap();
        let (r2s, r2e) = r2.split_once('-').unwrap();

        let (r1s, r1e) = (r1s.parse::<u32>()?, r1e.parse::<u32>()?);
        let (r2s, r2e) = (r2s.parse::<u32>()?, r2e.parse::<u32>()?);

        if (r1s >= r2s && r1e <= r2e) ||
           (r2s >= r1s && r2e <= r1e) {
            total += 1;
        }
    }

    println!("{total}");
    
    Ok(())
}
