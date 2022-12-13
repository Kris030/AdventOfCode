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

                files.push(ElFile::File {
                    name: s2.to_owned(),
                    size,
                });
            },
        }
    }

    list(&files, 0, 0);

    println!("{}", doit66(&files, 0));

    Ok(())
}

fn doit66(files: &[ElFile], curr: usize) -> u64 {
    match &files[curr] {
        ElFile::Dir { children, cached_size, .. } => {
            let mut sum = if *cached_size <= 100_000 { *cached_size } else { 0 };

            for c in children {
                sum += doit66(files, *c);
            }
            return sum;
        },
        _ => return 0,
    }
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
