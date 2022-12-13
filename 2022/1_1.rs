use std::{fs::File, io::Read, error::Error};

fn main() -> Result<(), Box<dyn Error>> {

    let mut src = String::new();
    File::open("1.txt")?
        .read_to_string(&mut src)?;
    
    let mut max: u32 = 0;
    for m in src.split("\n\n") {
        let mut cs = 0;
        for c in m.split('\n') {
            cs += c.parse::<u32>()?;
        }

        if cs > max {
            max = cs;
        }
    }

    println!("{max}");
    
    Ok(())
}
