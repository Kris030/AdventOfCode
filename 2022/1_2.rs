use std::{fs::File, io::Read, error::Error};

fn main() -> Result<(), Box<dyn Error>> {

    let mut src = String::new();
    File::open("1.txt")?
        .read_to_string(&mut src)?;
    
    let mut max: [u32; 3] = [0, 0, 0];
    
    for m in src.split("\n\n") {
        let mut cs = 0;
        for c in m.lines() {
            cs += c.parse::<u32>()?;
        }

        let mut mini = 0;
        for i in 1..3 {
            if max[mini] > max[i] {
                mini = i;
            }
        }
        
        if cs > max[mini] {
            max[mini] = cs;
        }
    }

    let mut sum = 0;
    for n in max {
        sum += n;
    }

    println!("{}", sum);
    
    Ok(())
}
