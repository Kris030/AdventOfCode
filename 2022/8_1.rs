use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let forest: Vec<Vec<u8>> = stdin().lines().map(|l| l.unwrap().into_bytes()).collect();
    let (w, h) = (forest[0].len(), forest.len());

    let mut visible = vec![vec![false; w]; h];
    
    // rows
    for y in 0..h {
        let mut max = forest[y][0];
        visible[y][0] = true;
        for x in 1..(w - 1) {
            // eprintln!("({x}; {y}) | {}", if forest[y][x] > max { "V" } else { "N" });
            if forest[y][x] > max {
                max = forest[y][x];
                visible[y][x] = true;
            }
        }

        let mut max = forest[y][w - 1];
        visible[y][w - 1] = true;
        for x in (1..(w - 1)).rev() {
            // eprintln!("({x}; {y}) | {}", if forest[y][x] > max { "V" } else { "N" });
            if forest[y][x] > max {
                max = forest[y][x];
                visible[y][x] = true;
            }
        }
    }

    // println!("Cols:");
    // cols
    for x in 0..w {
        let mut max = forest[0][x];
        visible[0][x] = true;
        for y in 1..(h - 1) {
            // eprintln!("({x}; {y}) | {}", if forest[y][x] > max { "V" } else { "N" });
            if forest[y][x] > max {
                max = forest[y][x];
                visible[y][x] = true;
            }
        }

        let mut max = forest[h - 1][x];
        visible[h - 1][x] = true;
        for y in (1..(h - 1)).rev() {
            // eprintln!("({x}; {y}) | {}", if forest[y][x] > max { "V" } else { "N" });
            if forest[y][x] > max {
                max = forest[y][x];
                visible[y][x] = true;
            }
        }
    }
    
    let mut vis = 0;
    for y in 0..h {
        for x in 0..w {
            if visible[y][x] {
                vis += 1;
                eprint!("O");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
    println!("{vis}");

    Ok(())
}
