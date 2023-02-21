use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use crossterm::event::{read, Event, KeyCode, KeyEvent};

#[allow(unused_parens)]

#[derive(Clone)]
#[derive(PartialEq)]
enum Cell {
    HasMine,
    Empty,
    EmptyAndChecked
}

fn clrscr() {
    // Clear the terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg("cls").status().unwrap();
    } else {
        Command::new("sh").arg("-c").arg("clear").status().unwrap();
    }
}

fn print_table(ref_table: &[Vec<Cell>], table: &[Vec<String>], table_size: &[usize; 2], coor: &[usize; 2], unopened_count: i16, reveal: bool) {
    clrscr();

    // Print the borders and indices 
    print!("\r^_^|");
    for i in 0..table_size[1] {
        print!(" {:02}", i);
    }

    print!("\n---+");
    for _ in 0..table_size[1] {
        print!("---");
    }
    println!();

    // Print (masked) cells
    for i in 0..table_size[0] {
        print!("{:02} | ", i);
        for j in 0..table_size[1] {
            if !reveal {
                if i == coor[0] && j == coor[1] {
                    print!("[{}]", table[i][j]);
                } else {
                    print!(" {} ", table[i][j]);
                }
            } else {
                print!(" {} ", match ref_table[i][j] {
                    Cell::HasMine => "Q",
                    _ => "_"
                });
            }
        }
        println!();
    }

    // Print the bottom
    println!("--------------------------------");
    println!("[{:02}:{:02}] | Unopened / Flagged: {:02}", coor[0], coor[1], unopened_count);
}

fn set_difficulty(current: &mut i8, new: i8) {
    // Add/subtract difficulty by new (within limits)
    *current += new;
    *current = (*current).clamp(0, 2);

    // Print main menu with changed difficulty
    let difficulty: String = match *current {
        0 => String::from
        ("||         [EASY(9x9)]        ||\n ||        MEDIUM(16x16)       ||\n ||         HARD(25x25)        ||\n ||     ~~~~#MINES: 10~~~~     ||"),
        1 => String::from
        ("||          EASY(9x9)         ||\n ||       [MEDIUM(16x16)]      ||\n ||         HARD(25x25)        ||\n ||     ~~~~#MINES: 50~~~~     ||"),       
        _ =>  String::from
        ("||          EASY(9x9)         ||\n ||        MEDIUM(16x16)       ||\n ||        [HARD(25x25)]       ||\n ||     ~~~~#MINES:100~~~~     ||"),     
    };

    clrscr();
    println!(" OO============================OO");
    println!(" || **** MINESWEEPER v0.2 **** ||");
    println!(" || -----------o00o----------- ||");
    println!(" ||     VERSION 0.1. BY D.     ||");
    println!(" || -------------------------- ||");
    println!(" ||     ****DIFFICULTY****     ||");
    println!(" {}", difficulty);
    println!(" || -------------------------- ||");
    println!(" || [ENTER] Start / Open cell  ||"); 
    println!(" || [^][v][<][>] Move cursor   ||");
    println!(" || [F] Flag selected cell     ||");
    println!(" || [C] Custom mode            ||");
    println!(" || [Q] Quit game              ||");
    println!(" OO============================OO");
}

fn count_nearby_mines(table: &mut Vec<Vec<String>>, ref_table: &mut Vec<Vec<Cell>>, table_size: &[usize; 2], coor_i8: [i8; 2]) {
    let mut count: i8 = 0;
    let coor: [usize; 2] = [coor_i8[0] as usize, coor_i8[1] as usize];

    // Break if coordinates are out-of-bounds
    if coor_i8[0] < 0 || coor_i8[1] < 0 || 
        coor[0] > table_size[0] - 1 || coor[1] > table_size[1] - 1 { return; }

    // Break if cell is checked before to prevent infinite recursion
    if ref_table[coor[0]][coor[1]] != Cell::Empty { return; }

    // Check nearby mine(s) (8 cells)
    if coor_i8[0] - 1 >= 0 && coor_i8[1] - 1 >= 0 && ref_table[coor[0] - 1][coor[1] - 1] == Cell::HasMine {
        count += 1;
    }

    if coor_i8[0] - 1 >= 0 && coor_i8[1] - 1 >= 0 && ref_table[coor[0] - 1][coor[1] - 1] == Cell::HasMine {
        count += 1;
    }
    
    if coor_i8[0] - 1 >= 0 && ref_table[coor[0] - 1][coor[1]] == Cell::HasMine {
        count += 1;
    }

    if coor_i8[0] - 1 >= 0 && coor[1] + 1 < table_size[1] && ref_table[coor[0] - 1][coor[1] + 1] == Cell::HasMine {
        count += 1;
    }

    if coor_i8[1] - 1 >= 0 && ref_table[coor[0]][coor[1] - 1] == Cell::HasMine {
        count += 1;
    }

    if coor[1] + 1 < table_size[1] && ref_table[coor[0]][coor[1] + 1] == Cell::HasMine {
        count += 1;
    }

    if coor[0] + 1 < table_size[0] && coor_i8[1] - 1 >= 0 && ref_table[coor[0] + 1][coor[1] - 1] == Cell::HasMine {
        count += 1;
    }

    if coor[0] + 1 < table_size[0] && ref_table[coor[0] + 1][coor[1]] == Cell::HasMine {
        count += 1;
    }

    if coor[0] + 1 < table_size[0] && coor[1] + 1 < table_size[1] && ref_table[coor[0] + 1][coor[1] + 1] == Cell::HasMine {
        count += 1;
    }

    if count != 0 {
        // Print number of nearby mines
        table[coor[0]][coor[1]] = count.to_string();
    } else {
        // Repeat for nearby cells
        table[coor[0]][coor[1]] = "_".to_string();
        ref_table[coor[0]][coor[1]] = Cell::EmptyAndChecked;

        count_nearby_mines(table, ref_table, table_size, [coor_i8[0] - 1, coor_i8[1] - 1]);
        count_nearby_mines(table, ref_table, table_size, [coor_i8[0] - 1, coor_i8[1]]);
        count_nearby_mines(table, ref_table, table_size, [coor_i8[0] - 1, coor_i8[1] + 1]);
        count_nearby_mines(table, ref_table, table_size, [coor_i8[0], coor_i8[1] - 1]);
        count_nearby_mines(table, ref_table, table_size, [coor_i8[0], coor_i8[1] + 1]);
        count_nearby_mines(table, ref_table, table_size, [coor_i8[0] + 1, coor_i8[1] - 1]);
        count_nearby_mines(table, ref_table, table_size, [coor_i8[0] + 1, coor_i8[1]]);
        count_nearby_mines(table, ref_table, table_size, [coor_i8[0] + 1, coor_i8[1] + 1]);
    }
}

fn main() {
    let mut selected_difficulty: i8 = 1;
    let mut table_size: [usize; 2] = [16; 2];
    let mut mines_count: i16 = 40;
    let mut custom_mode_enabled: bool = false;

    // Print main menu
    set_difficulty(&mut selected_difficulty, 0);

    'MAIN_MENU: loop {
        //Detect key input
        match read().unwrap() {
            // Change difficulty
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                ..
            }) => set_difficulty(&mut selected_difficulty, -1),

            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => set_difficulty(&mut selected_difficulty, 1),

            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                ..
            }) => 'CUSTOM_MODE: {
                let mut choice: i8 = 0;
                
                loop {
                    // Print custom mode menu
                    clrscr();
                    println!(" OO============================OO");
                    println!(" || **** MINESWEEPER v0.2 **** ||");
                    println!(" || -----------o00o----------- ||");
                    println!(" ||     VERSION 0.1. BY D.     ||");
                    println!(" || -------------------------- ||");
                    println!(" ||     ***CUSTOM  MODE***     ||");
                    match choice {
                        0 => {
                            println!(" ||># OF ROW(S) (2 to 50): {:02}  ||", table_size[0]);
                            println!(" || # OF COL(S) (2 to 50): {:02}  ||", table_size[1]);
                            println!(" || # OF MINE(S)(1 to ROW*COL):||");
                        },
                        1 => {
                            println!(" || # OF ROW(S) (2 to 50): {:02}  ||", table_size[0]);
                            println!(" ||># OF COL(S) (2 to 50): {:02}  ||", table_size[1]);
                            println!(" || # OF MINE(S)(1 to ROW*COL):||");
                        },
                        _ => {
                            println!(" || # OF ROW(S) (2 to 50): {:02}  ||", table_size[0]);
                            println!(" || # OF COL(S) (2 to 50): {:02}  ||", table_size[1]);
                            println!(" ||># OF MINE(S)(1 to ROW*COL):||");
                        }
                    }
                    println!(" ||(DEFAULT: 0.2 * #CELLS) {:04}||", mines_count);
                    println!(" || -------------------------- ||");
                    println!(" || [ENTER] Start game /       ||");
                    println!(" ||         Open selected cell ||");
                    println!(" || [<][>] Select entry        ||");
                    println!(" || [^][v] Incr/Decr value     ||");
                    println!(" || [Q] Return & Discard all   ||");
                    println!(" OO============================OO");

                    // Detect key input (custom mode)
                    match read().unwrap() {
                        Event::Key(KeyEvent {
                            code: KeyCode::Up,
                            ..
                        }) => {
                            // Increase selected value (within limits)
                            match choice {
                                0 => if table_size[0] < 50 {
                                    table_size[0] += 1;
                                },
                                1 => if table_size[1] < 50 {
                                    table_size[1] += 1;
                                },
                                _ => if (mines_count as usize) < table_size[0] * table_size[1] {
                                    mines_count += 1;
                                }
                            }
                        },
            
                        Event::Key(KeyEvent {
                            code: KeyCode::Down,
                            ..
                        }) => {
                            // Decrease selected value (within limits)
                            match choice {
                                0 => if table_size[0] > 2 {
                                    table_size[0] -= 1;
                                },
                                1 => if table_size[1] > 2 {
                                    table_size[1] -= 1;
                                },
                                _ => if mines_count > 1 {
                                    mines_count -= 1;
                                }
                            }
                        },

                        // Change selection (within limits)
                        Event::Key(KeyEvent {
                            code: KeyCode::Left,
                            ..
                        }) => {
                            choice -= 1;
                            choice = choice.clamp(0, 2);
                        },
            
                        Event::Key(KeyEvent {
                            code: KeyCode::Right,
                            ..
                        }) => {
                            choice += 1;
                            choice = choice.clamp(0, 2);
                        },

                        // Quit custom mode
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('q'),
                            ..
                        }) => {
                            custom_mode_enabled = false;
                            set_difficulty(&mut selected_difficulty, 0);
                            break 'CUSTOM_MODE;
                        },

                        // Enter game, custom values checked, custom mode is enabled
                        Event::Key(KeyEvent {
                            code: KeyCode::Enter,
                            ..
                        }) => {
                            selected_difficulty = 3;
                            // Check if number of mines is larger or equal to number of cells 
                            // Thought: What if number of mines == number of cells - 1?
                            if (mines_count as usize) >= table_size[0] * table_size[1] {
                                mines_count = (table_size[0] * table_size[1] / 5) as i16;
                                if mines_count < 1 {
                                    mines_count = 1;
                                }
                            }
                            custom_mode_enabled = true;
                            break 'MAIN_MENU;
                        },
                        _ => ()
                    }
                }
            },

            // Quit
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => return,

            // Enter game
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => break,

            _ => ()
        }
    }
    
    // Set table size
    table_size[0] = match selected_difficulty {
        0 => 9,
        1 => 16,
        2 => 25,
        _ => table_size[0]
    };
    table_size[1] = match selected_difficulty {
        0 => 9,
        1 => 16,
        2 => 25,
        _ => table_size[1]
    };
    // Set number of mines for default table size values
    if !custom_mode_enabled {
        mines_count = match table_size[0] {
            9 => 10,
            16 => 50,
            25 => 100,
            _ => mines_count
        };
    }

    let mut ref_table: Vec<Vec<Cell>> = vec![vec![Cell::Empty; table_size[1]]; table_size[0]];
    let mut table: Vec<Vec<String>> = vec![vec![String::from("*"); table_size[1]]; table_size[0]];
    let mut first_click: bool = true;
    let mut coor: [usize; 2] = [0, 0];
    let mut unopened_count: i16 = 0;

    // Listen for key inputs in main game
    'LISTEN: loop {
        print_table(&ref_table, &table, &table_size, &coor, unopened_count, false);
        match read().unwrap() {
            // Move cursor
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                ..
            }) => {
                if coor[0] > 0 {
                    coor[0] -= 1;
                }
            },

            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                if coor[0] < table_size[0] {
                    coor[0] += 1;
                }
            },

            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                if coor[1] > 0 {
                    coor[1] -= 1;
                }
            },

            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                if coor[1] < table_size[1] {
                    coor[1] += 1;
                }
            },

            // Flag cell
            Event::Key(KeyEvent {
                code: KeyCode::Char('f'),
                ..
            }) => {
                if table[coor[0]][coor[1]] == "*" {
                    table[coor[0]][coor[1]] = "P".to_string();
                } else if table[coor[0]][coor[1]] == "P" {
                    table[coor[0]][coor[1]] = "*".to_string();
                }
            },

            // Quit game
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => return,

            // Open cell
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                // Set mines after first click
                if first_click {
                    let mut random = rand::thread_rng();
                    for _ in 0..mines_count {
                        let mut x: usize = random.gen_range(0..table_size[0]);
                        let mut y: usize = random.gen_range(0..table_size[1]);

                        // There must be enough number of mines & mines must be far enough from current coordinates
                        while ref_table[x][y] == Cell::HasMine || (coor[0] == x && coor[1] == y) ||
                            (((x as i8 - coor[0] as i8).abs() < 3 || (y as i8 - coor[1] as i8).abs() < 3) && !custom_mode_enabled) {
                            x = random.gen_range(0..table_size[0]);
                            y = random.gen_range(0..table_size[1]);
                        }

                        ref_table[x][y] = Cell::HasMine;
                    }

                    // Write table to debug.txt for debugging
                    let mut table_str = String::from("[");
                    for i in 0..table_size[0] {
                        table_str += "[";
                        for j in 0..table_size[1] {
                            table_str += &format!("{},", match ref_table[i][j] {
                                Cell::HasMine => "'Q'",
                                _ => "'_'"
                            });
                        }
                        table_str += "],\n";
                    }
                    table_str += "]";
                    
                    let debug_file = File::create("debug.txt");
                    match debug_file {
                        Ok(_) => match debug_file.unwrap().write_all(table_str.as_bytes()) {
                            Ok(_) => (),
                            Err(_e) => (),
                        },
                        Err(_e) => ()
                    };

                    first_click = false;
                }

                // Losing condition
                if ref_table[coor[0]][coor[1]] == Cell::HasMine {
                    print_table(&ref_table, &table, &table_size, &coor, unopened_count, true);
                    println!("Game over! Press any key to try again");
                    break 'LISTEN;
                } else {
                    count_nearby_mines(&mut table, &mut ref_table, &table_size, [coor[0] as i8, coor[1] as i8]);
                }

                // Count unopened cell for winning condition
                unopened_count = 0;
                for i in 0..table_size[0] {
                    for j in 0..table_size[1] {
                        if table[i][j] == *"*" || table[i][j] == *"P" {
                            unopened_count += 1;
                        }
                    }
                }

                // Winning condition
                if unopened_count == mines_count {
                    print_table(&ref_table, &table, &table_size, &coor, unopened_count, true);
                    println!("Congratulations! Press any key to continue");
                    break 'LISTEN;
                }
            },
            
            _ => ()
        }
    }

    // Pause terminal before new game
    read().unwrap();
    main();
}