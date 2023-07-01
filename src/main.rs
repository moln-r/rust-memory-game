use rand::Rng;

struct Tile {
    value: char,
    found: bool,
}

struct SelectedTiles {
    first: (usize, usize),
    second: Option<(usize, usize)>,
}

fn main() {
    println!("Welcome to the memory game!");

    let size: (usize, usize) = get_game_size();
    let game: Vec<Vec<Tile>> = generate_game(size.0, size.1);

    // print_cheat_sheet(&game);
    play(game);
}

fn get_game_size() -> (usize, usize) {
    let mut x;
    let mut y;
    loop {
        x = get_numeric_input("Provide horizontal size for the game:");
        y = get_numeric_input("Provide vertical size for the game:");
        if x * y % 2 != 0 {
            println!("Game must contain even number of tiles, current size is odd, try again!")
        } else {
            break;
        }
    }
    return (x, y);
}

fn get_numeric_input(command: &str) -> usize {
    loop {
        println!("{} ", command);

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        // println!("Received input: {}", &line);

        let parsed_result: Result<i32, _> = line.trim().parse();

        match parsed_result {
            Ok(parsed_value) => {
                if parsed_value > 0 && parsed_value <= 7 {
                    return parsed_value as usize;
                }
            }
            Err(_error) => {
                // println!("Error: {:?}", error)
            }
        };

        println!("Invalid input, try again!");
    }
}

fn generate_game(game_size_x: usize, game_size_y: usize) -> Vec<Vec<Tile>> {
    let game_size: usize = game_size_x * game_size_y;
    let mut possible_tile_values: Vec<char> = Vec::new();
    for char in 'a'..'z' {
        possible_tile_values.push(char);
        possible_tile_values.push(char);
        if possible_tile_values.len() == game_size {
            break;
        }
    }

    let mut game: Vec<Vec<Tile>> = Vec::new();
    for _x in 0..game_size_x {
        let mut row: Vec<Tile> = Vec::new();
        for _y in 0..game_size_y {
            let tile_value: char = possible_tile_values.remove(rand::thread_rng().gen_range(0..possible_tile_values.len()));

            let tile = Tile { value: tile_value, found: false };
            row.push(tile);
        }
        game.push(row);
    }

    return game;
}

#[allow(dead_code)]
fn print_cheat_sheet(game: &Vec<Vec<Tile>>) {
    for row in game {
        for tile in row {
            print!(" {} ", tile.value)
        }
        println!()
    }
    print!("\n\n\n")
}

fn play(mut game: Vec<Vec<Tile>>) {
    loop {
        if finished(&game) {
            break;
        }
        print_game(&game, None);

        println!("Choose first guess!");
        let mut selected_tiles = SelectedTiles {
            first: (get_numeric_input("Which row?") - 1, get_numeric_input("Which column?") - 1),
            second: None,
        };
        print_game(&game, Option::from(&selected_tiles));

        println!("Choose second guess!");
        loop {
            selected_tiles.second = Option::from((get_numeric_input("Which row?") - 1, get_numeric_input("Which column?") - 1));
            if selected_tiles.first == selected_tiles.second.unwrap() {
                println!("You must not have the same guess twice!");
            } else {
                break;
            }
        }
        print_game(&game, Option::from(&selected_tiles));

        if is_match(&game, &selected_tiles) {
            if let Some(row) = game.get_mut(selected_tiles.first.0) {
                if let Some(tile) = row.get_mut(selected_tiles.first.1) {
                    tile.found = true;
                }
            }

            if let Some(second) = selected_tiles.second {
                if let Some(row) = game.get_mut(second.0) {
                    if let Some(tile) = row.get_mut(second.1) {
                        tile.found = true;
                    }
                }
            }
        }

        print!("\n\n\n")
    }
}

fn finished(game: &Vec<Vec<Tile>>) -> bool {
    for vec in game {
        for tile in vec {
            if !tile.found {
                return false;
            }
        }
    }
    println!("Congrats, the game is finished!");
    return true;
}

fn print_game(game: &Vec<Vec<Tile>>, selected_tile: Option<&SelectedTiles>) {
    let mut first_selected = false;
    let mut second_selected = false;

    let mut first = (0, 0);
    let mut second = (0, 0);

    match selected_tile {
        Some(sel) => {
            first_selected = true;
            first = sel.first;
        }
        None => {}
    }
    if first_selected {
        match selected_tile.unwrap().second {
            Some(sec) => {
                second_selected = true;
                second = sec
            }
            None => {}
        }
    }

    for row_i in 0..game.len() {
        let row = game.get(row_i).unwrap();
        for col_i in 0..row.len() {
            let tile = row.get(col_i).unwrap();

            let mut force_print = false;
            if first_selected && first.0 == row_i && first.1 == col_i {
                force_print = true;
            } else if !force_print && second_selected && second.0 == row_i && second.1 == col_i {
                force_print = true;
            }

            let value_to_print = if tile.found || force_print { tile.value } else { '?' };
            print!(" {} ", value_to_print)
        }
        println!()
    }
}

fn is_match(game: &Vec<Vec<Tile>>, selected_tiles: &SelectedTiles) -> bool {
    let first = selected_tiles.first;
    let second = selected_tiles.second.unwrap();

    if let Some(row1) = game.get(first.0) {
        if let Some(tile1) = row1.get(first.1) {
            if let Some(row2) = game.get(second.0) {
                if let Some(tile2) = row2.get(second.1) {
                    if tile1.value == tile2.value {
                        println!("That's a match!");
                        return true;
                    }
                }
            }
        }
    }
    return false;
}
