use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let forest: Vec<Vec<u8>> = stdin().lines().map(|l| l.unwrap().into_bytes()).collect();
    let (w, h) = (forest[0].len(), forest.len());

    let mut mx = 0;
    for tree_x in 0..w {
        for tree_y in 0..h {
            if tree_x == 0 || tree_y == 0 {
                continue;
            }
            let tree_height = forest[tree_y][tree_x];

            // ======== RIGHT ========
            let mut right = 0;
            for x in (tree_x + 1)..w {
                                right += 1;
                if forest[tree_y][x] >= tree_height {
                    break;
                }
            }
                        
            // ======== LEFT ========
            let mut left = 0;
            if tree_x != 0 {
                for x in (0..tree_x).rev() {
                                        left += 1;
                    if forest[tree_y][x] >= tree_height {
                        break;
                    }
                }
            }
            
            // ======== DOWN ========
            let mut down = 0;
            for y in (tree_y + 1)..h {
                                down += 1;
                if forest[y][tree_x] >= tree_height {
                    break;
                }
            }
            
            // ======== UP ========
            let mut up = 0;
            if tree_y != 0 {
                for y in (0..tree_y).rev() {
                                        up += 1;
                    if forest[y][tree_x] >= tree_height {
                        break;
                    }
                }
            }
            // =======================

            let dist_total = right * left * down * up;
            if dist_total > mx {
                mx = dist_total;
            }
        }
    }

    println!("{mx}");
    
    Ok(())
}
