fn get<T>(v: &Vec<Vec<T>>, y: i32, x: i32) -> Option<&T> {
    if x >= 0 && y >= 0 {
        match &v.get(y as usize) {
            &Some(row) => row.get(x as usize),
            &None => None,
        }
    } else {
        None
    }
}

fn main() {
    let initial_state = vec![
        vec![false, true, false],
        vec![false, true, false],
        vec![false, true, false],
    ];

    let mut new_state = initial_state.to_vec();

    for (y, line) in initial_state.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            let x = x as i32;
            let y = y as i32;
            let neighbors = vec![
                get(&initial_state, y - 1, x - 1), get(&initial_state, y - 1, x), get(&initial_state, y - 1, x + 1),
                get(&initial_state, y,     x - 1),                                get(&initial_state, y,     x + 1),
                get(&initial_state, y + 1, x - 1), get(&initial_state, y + 1, x), get(&initial_state, y + 1, x + 1),
            ];

            let alive_neighbor_count = neighbors.iter().filter(|x| x.is_some()).filter(|x| *x.unwrap()).collect::<Vec<_>>().len();

            let new_cell = match (*cell, alive_neighbor_count) {
                (true, 2) => true,
                (_, 3) => true,
                (_, _) => false,
            };

            new_state[y as usize][x as usize] = new_cell;

            //println!("({}, {}): {} with {:?} = {}", x, y, *cell, alive_neighbor_count, new_cell);
        }
    }

    for line in new_state {
        for cell in line {
            if cell {
                print!("X");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
