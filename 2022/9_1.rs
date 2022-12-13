use std::{io::stdin, error::Error, ops::{Sub, AddAssign}, collections::HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
    fn sign(&self) -> Self {
        Pos {
            x: sign(self.x),
            y: sign(self.y),
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for &Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn sign(value: i32) -> i32 {
    if value > 0 {
        1
    } else if value < 0 {
        -1
    } else {
        0
    }
}

fn sim(head: &Pos, tail: &Pos) -> Pos {
    let d = head - tail;
    let sgn = d.sign();

    // +-----+
    // |..H..|
    // |.....|
    // |H.T.H|
    // |.....|
    // |..H..|
    // +-----+
    if d.x.abs() + d.y.abs() == 2 && (sgn.x != 0) ^ (sgn.y != 0) {
        return sgn;
    }

    // +-----+
    // |.....|
    // |.HHH.|
    // |.HTH.|
    // |.HHH.|
    // |.....|
    // +-----+
    if d.x.abs() + d.y.abs() <= 2 {
        return Pos::new(0, 0);
    }

    // +-----+
    // |.H.H.|
    // |H...H|
    // |..T..|
    // |H...H|
    // |.H.H.|
    // +-----+
    if d.x.abs() > d.y.abs() {
        Pos::new(d.x - sgn.x, d.y)
    } else {
        Pos::new(d.x, d.y - sgn.y)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut head = Pos::new(0, 0); 
    let mut tail = Pos::new(0, 0);

    let mut visited_positions = HashSet::new();
    visited_positions.insert(tail);

    for l in stdin().lines() {
        let l = l?;

        let (d, a) = l.split_once(' ').unwrap();
        let steps: i32 = a.parse()?;
        let direction = match d {
            "U" => Pos::new(0, 1),
            "D" => Pos::new(0, -1),
            "L" => Pos::new(-1, 0),
            "R" => Pos::new(1, 0),
            _ => unreachable!(),
        };

        for _ in 0..steps {
            head += direction;
            tail += sim(&head, &tail);
            visited_positions.insert(tail);
        }
    }

    println!("{}", visited_positions.len());
    
    Ok(())
}
