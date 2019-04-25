use std::fmt;
use std::fmt::Debug;

#[derive(Clone, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

struct Line {
    start: Point,
    end: Point,
}

type Path = Vec<Point>;
type Occupied = Vec<Vec<bool>>;

//fn draw_line(line: Line, occupied: &mut Occupied, path: &mut Path) {
//    occupied[line.start.x][line.start.y] = true;
//    occupied[line.end.x][line.end.y] = true;
//
//    path.push(line);
//}

fn intersect(a: Line, b: Line) -> bool {
    false
}

fn draw_path(path: Path, size: usize, longest: &mut usize) {
    if path.len() > *longest {
        println!("{:?}", path);
        *longest = path.len();
    }
    let last_point = path.last().unwrap();

    for x in 0..size {
        for y in 0..size {
            if !path.contains(&Point { x, y }) {
                let mut new_path = path.clone();
                new_path.push(Point { x, y });
                draw_path(new_path, size, longest);
            }
        }
    }
}

fn main() {
    let size: usize = 4;

    //    let mut occupied: Occupied = (0..size).map(|x| {
    //        (0..size).map(|y| {
    //            false
    //        }
    //        ).collect()
    //    }).collect();

    let path = vec![Point { x: 0, y: 0 }];

    //    println!("{:?}", occupied);

    //    draw_line(Line {start: Point {x: 1, y: 1}, end: Point {x: 2, y: 2} }, &mut occupied, &mut path);

    let mut longest: usize = 0;
    draw_path(path, size, &mut longest);

    //    println!("{:?}", occupied);
}
