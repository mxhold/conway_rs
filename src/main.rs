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

fn alive_neighbor_count(v: &Vec<Vec<bool>>, y: usize, x: usize) -> usize {
    let x = x as i32;
    let y = y as i32;
    let neighbors = vec![
        get(&v, y - 1, x - 1), get(&v, y - 1, x), get(&v, y - 1, x + 1),
        get(&v, y,     x - 1),                    get(&v, y,     x + 1),
        get(&v, y + 1, x - 1), get(&v, y + 1, x), get(&v, y + 1, x + 1),
    ];

    neighbors.iter().filter(|x| x.is_some()).filter(|x| *x.unwrap()).collect::<Vec<_>>().len()
}

fn print_state(v: &Vec<Vec<bool>>) {
    for line in v {
        for &cell in line {
            if cell {
                print!("X");
            } else {
                print!(".");
            }
        }
        print!("\n");
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

            let new_cell = match (*cell, alive_neighbor_count(&initial_state, y, x)) {
                (true, 2) => true,
                (_, 3) => true,
                (_, _) => false,
            };

            new_state[y as usize][x as usize] = new_cell;
        }
    }

    print_state(&new_state);
}
