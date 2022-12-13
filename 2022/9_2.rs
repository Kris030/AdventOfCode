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

fn move_knot(next: &Pos, knot: &Pos) -> Pos {
    let d = next - knot;
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
    const BODY_COUNT: usize = 10 - 1;

    let mut head = Pos::new(0, 0);
    let mut body = [Pos::new(0, 0); BODY_COUNT]; 

    let mut visited_positions: HashSet<Pos> = HashSet::new();
    visited_positions.insert(head);

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

            body[0] += move_knot(&head, &body[0]);
            for b in 1..BODY_COUNT {
                body[b] += move_knot(&body[b - 1], &body[b]);
            }

            visited_positions.insert(body[BODY_COUNT - 1]);
        }
    }

    println!("{}", visited_positions.len());

    for y in (0..12).rev() {
        for x in 0..26 {
            let c = if visited_positions.contains(&Pos { x, y }) {
                '#'
            } else {
                '.'
            };
            eprint!("{}", c);
        }
        eprintln!();
    }

    Ok(())
}
