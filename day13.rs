use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;

struct Cart {
    x: usize,
    y: usize,
    heading: usize,
    next_turn: i32,
    id: i32,
    crashed: bool,
}

fn main() {

    let mut track: [[char;155];155] = [['?';155];155];
    let mut carts: Vec<Cart> = Vec::new();
    let input = fs::read_to_string("day13.input")
        .expect("Something went wrong reading the file");

    struct Heading {
        x_diff: i32,
        y_diff: i32,
    }

    let heading_left = Heading {x_diff: -1, y_diff: 0};
    let heading_up = Heading {x_diff: 0, y_diff: -1};
    let heading_right = Heading {x_diff: 1, y_diff: 0};
    let heading_down = Heading {x_diff: 0, y_diff: 1};

    let heading_order = [heading_left, heading_up, heading_right, heading_down];
    let mut id = 0;

    let mut points = HashMap::new();

    for (i,l) in input.lines().enumerate() {
        for (j,c) in l.chars().enumerate() {
            match c {
                '^' => {
                    id += 1;
                    let mut cart = Cart {
                        x: j,
                        y: i,
                        heading: 4,
                        next_turn: 0,
                        id,
                        crashed: false,
                    };
                    points.insert((cart.x, cart.y), cart.id);
                    cart.heading = 1;
                    track[i][j] = '|';
                    carts.push(cart);
                },
                'v' => {
                    id += 1;
                    let mut cart = Cart {
                        x: j,
                        y: i,
                        heading: 4,
                        next_turn: 0,
                        id,
                        crashed: false,
                    };
                    points.insert((cart.x, cart.y), cart.id);
                    cart.heading = 3;
                    track[i][j] = '|';
                    carts.push(cart);
                },
                '>' => {
                    id += 1;
                    let mut cart = Cart {
                        x: j,
                        y: i,
                        heading: 4,
                        next_turn: 0,
                        id,
                        crashed: false,
                    };
                    points.insert((cart.x, cart.y), cart.id);
                    cart.heading = 2;
                    track[i][j] = '-';
                    carts.push(cart);
                },
                '<' => {
                    id += 1;
                    let mut cart = Cart {
                        x: j,
                        y: i,
                        heading: 4,
                        next_turn: 0,
                        id,
                        crashed: false,
                    };
                    points.insert((cart.x, cart.y), cart.id);
                    cart.heading = 0;
                    track[i][j] = '-';
                    carts.push(cart);
                },
                _ => {
                    track[i][j] = c
                },
            }
        }
    }

    let mut round = 0;
    let mut crashed: Vec<i32> = Vec::new();
    loop {
        round += 1;
        carts.sort_by(sort_carts);
        for c in &mut carts {
            if crashed.contains(&c.id) {
                continue;
            }
            points.remove(&(c.x, c.y));
            c.x = (c.x as i32 + heading_order[c.heading].x_diff) as usize;
            c.y = (c.y as i32 + heading_order[c.heading].y_diff) as usize;

            if points.contains_key(&(c.x, c.y)) {
                crashed.push(*points.get(&(c.x, c.y)).unwrap());
                crashed.push(c.id);
                points.remove(&(c.x, c.y));
            } else {
                points.insert((c.x, c.y), c.id);
            }


            match track[c.y][c.x] {
                '/' => {
                    if c.heading == 3 || c.heading == 1 {
                        c.heading = (c.heading + 1) % 4;
                    } else {
                        c.heading = if c.heading == 0 {
                            3
                        } else {
                            c.heading - 1
                        };
                    }
                }
                '\\' => {
                    if c.heading == 0 || c.heading == 2 {
                        c.heading += 1;
                    } else {
                        c.heading = if c.heading == 0 {
                            3
                        } else {
                            c.heading - 1
                        };
                    }
                }
                '+' => {
                    match c.next_turn {
                        2 => {
                            c.heading = (c.heading + 1) % 4;
                        }
                        0 => {
                            c.heading = if c.heading == 0 {
                                3
                            } else {
                                c.heading - 1
                            };
                        }
                        _ => ()
                    }
                    c.next_turn = (c.next_turn + 1) % 3;
                }
                _ => ()
            }
        }

        if crashed.len() == carts.len() - 1 {
            for c in &carts {
                if !crashed.contains(&c.id) {
                    panic!("Last cart on {},{}", c.x, c.y)
                }
            }
        }
    }

}

fn sort_carts(a: &Cart, b: &Cart) -> Ordering {
    if a.y == b.y {
        a.x.cmp(&b.x)
    } else {
        a.y.cmp(&b.y)
    }
}
