use std::{io::stdin, error::Error};

#[derive(Debug)]
pub enum ElFile {
    Dir {
        name: String,
        parent: usize,
        children: Vec<usize>,

        cached_size: u64,
    },
    File {
        name: String,
        size: u64,
    }
}

impl ElFile {
    fn new_dir(parent: usize, name: &str) -> Self {
        Self::Dir { children: Vec::new(), parent, name: name.to_owned(), cached_size: 0 }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let it = stdin().lines()
        .map_while(|r| r.ok());

    let mut files = vec![ElFile::new_dir(0, "/")];
    let mut curr = 0;

    // TODO: kys
    let mut total_used = 0;

    for l in it.skip(1) {
        match &l[..4] {
            "$ cd" => {
                let name = &l[5..];
                let len = files.len();
                
                if let ".." = name {
                    let ElFile::Dir { parent, cached_size, .. } = files[curr] else { panic!("not in a directory xd"); };
                    let ElFile::Dir { cached_size: parent_size, .. } = &mut files[parent] else { panic!("not in a directory xd"); };
                    
                    *parent_size += cached_size;

                    curr = parent;
                } else {
                    let ElFile::Dir { children, .. } = &mut files[curr] else { panic!("not in a directory xd"); };
                    let nf = ElFile::new_dir(curr, name);
                    children.push(len);
                    files.push(nf);
                    curr = len;
                }
            }

            // ignore the ls command itself
            "$ ls" => (),

            // we're in an ls output
            _ => 'file: {
                let (s1, s2) = l.split_once(' ').unwrap();
                if let "dir" = s1 {
                    break 'file;
                }

                let len = files.len();
                let ElFile::Dir { children, cached_size, .. } = &mut files[curr] else { panic!("not in a directory xd"); };
                children.push(len);

                let size = s1.parse().unwrap();
                *cached_size += size;
                total_used += size;

                files.push(ElFile::File {
                    name: s2.to_owned(),
                    size,
                });
            },
        }
    }

    // add the last directory to the root

    list(&files, 0, 0);

    const TOTAL_STORAGE: u64 = 70_000_000;
    const NEEDED_STORAGE: u64 = 30_000_000;

    let total_free = TOTAL_STORAGE - total_used;

    let mut min = TOTAL_STORAGE;
    for f in files {
        let ElFile::Dir { cached_size: size, .. } = f else { continue };
        if total_free + size >= NEEDED_STORAGE && size < min {
            min = size;
        }
    }

    println!("{}", min);

    Ok(())
}

fn list(files: &[ElFile], curr: usize, ilev: u64) {
    for _ in 0..ilev {
        eprint!("  ");
    }
    match &files[curr] {
        ElFile::Dir { name, children, cached_size, .. } => {
            eprintln!("{}. {} (dir, cached size={})", curr, name, cached_size);
            for c in children {
                list(files, *c, ilev + 1);
            }
        },
        ElFile::File { name, size } => eprintln!("{}. {} (file, size={})", curr, name, size),
    }
}
