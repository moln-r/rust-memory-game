use rand::Rng;

use crate::input::get_numeric_input;

pub struct Game {
    board: Vec<Vec<Tile>>,
}

struct Tile {
    value: char,
    found: bool,
}

struct SelectedTiles {
    first: (usize, usize),
    second: Option<(usize, usize)>,
}

impl Game {
    #[allow(dead_code)]
    pub(crate) fn print_cheat_sheet(&self) {
        for row in &self.board {
            for tile in row {
                print!(" {} ", tile.value)
            }
            println!()
        }
        print!("\n\n\n")
    }

    pub(crate) fn play(&mut self) {
        loop {
            if finished(&self.board) {
                break;
            }
            self.print(None);

            println!("Choose first guess!");
            let mut selected_tiles = SelectedTiles {
                first: (
                    get_numeric_input("Which row?") - 1,
                    get_numeric_input("Which column?") - 1,
                ),
                second: None,
            };
            self.print(Option::from(&selected_tiles));

            println!("Choose second guess!");
            loop {
                selected_tiles.second = Option::from((
                    get_numeric_input("Which row?") - 1,
                    get_numeric_input("Which column?") - 1,
                ));
                if selected_tiles.first == selected_tiles.second.unwrap() {
                    println!("You must not have the same guess twice!");
                } else {
                    break;
                }
            }
            self.print(Option::from(&selected_tiles));

            if self.is_match(&selected_tiles) {
                if let Some(row) = self.board.get_mut(selected_tiles.first.0) {
                    if let Some(tile) = row.get_mut(selected_tiles.first.1) {
                        tile.found = true;
                    }
                }

                if let Some(second) = selected_tiles.second {
                    if let Some(row) = self.board.get_mut(second.0) {
                        if let Some(tile) = row.get_mut(second.1) {
                            tile.found = true;
                        }
                    }
                }
            }

            print!("\n\n\n")
        }
    }

    fn print(&self, selected_tile: Option<&SelectedTiles>) {
        let mut first_selected = false;
        let mut second_selected = false;

        let mut first = (0, 0);
        let mut second = (0, 0);

        if let Some(sel) = selected_tile {
            first_selected = true;
            first = sel.first;
        }
        if first_selected {
            if let Some(sec) = selected_tile.unwrap().second {
                second_selected = true;
                second = sec
            }
        }

        for row_i in 0..self.board.len() {
            let row = self.board.get(row_i).unwrap();
            for col_i in 0..row.len() {
                let tile = row.get(col_i).unwrap();

                let mut force_print = false;
                if first_selected && first.0 == row_i && first.1 == col_i {
                    force_print = true;
                }
                if second_selected && second.0 == row_i && second.1 == col_i {
                    force_print = true;
                }

                let value_to_print = if tile.found || force_print {
                    tile.value
                } else {
                    '?'
                };
                print!(" {} ", value_to_print)
            }
            println!()
        }
    }

    fn is_match(&self, selected_tiles: &SelectedTiles) -> bool {
        let first = selected_tiles.first;
        let second = selected_tiles.second.unwrap();

        if let Some(row1) = self.board.get(first.0) {
            if let Some(tile1) = row1.get(first.1) {
                if let Some(row2) = self.board.get(second.0) {
                    if let Some(tile2) = row2.get(second.1) {
                        if tile1.value == tile2.value {
                            println!("That's a match!");
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn new(size_column: usize, size_row: usize) -> Self {
        let mut possible_tile_values: Vec<char> = get_possible_tile_values(size_column * size_row);

        let mut game: Vec<Vec<Tile>> = Vec::new();
        for _x in 0..size_column {
            let mut row: Vec<Tile> = Vec::new();
            for _y in 0..size_row {
                let tile_value: char = possible_tile_values
                    .remove(rand::thread_rng().gen_range(0..possible_tile_values.len()));

                let tile = Tile {
                    value: tile_value,
                    found: false,
                };
                row.push(tile);
            }
            game.push(row);
        }

        Game { board: game }
    }
}

fn get_possible_tile_values(game_size: usize) -> Vec<char> {
    let mut possible_tile_values: Vec<char> = Vec::new();
    for char in 'a'..='z' {
        possible_tile_values.push(char);
        possible_tile_values.push(char);
        if possible_tile_values.len() == game_size {
            break;
        }
    }
    possible_tile_values
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
    true
}
