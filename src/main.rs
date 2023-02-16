use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use crossterm::event::{read, Event, KeyCode, KeyEvent};

fn clrscr() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg("cls").status().unwrap();
    } else {
        Command::new("sh").arg("-c").arg("clear").status().unwrap();
    }
}

fn print_table(table: &[Vec<String>], table_size: usize, coor: [usize; 2], unopened_count: i16) {
    clrscr();

    print!("\r^_^|");
    for i in 0..table_size {
        print!(" {:02}", i);
    }

    print!("\n---+");
    for _ in 0..table_size {
        print!("---");
    }
    println!();

    for i in 0..table_size {
        print!("{:02} | ", i);
        for j in 0..table_size {
            if i == coor[0] && j == coor[1] {
                print!("[{}]", table[i][j]);
            } else {
                print!(" {} ", table[i][j]);
            }
        }
        println!();
    }

    println!("--------------------------------");
    println!("[{:02}:{:02}] | Unopened / Flagged: {:02}", coor[0], coor[1], unopened_count);
}

fn set_difficulty(current: &mut i8, new: i8) {
    *current += new;
    *current = (*current).clamp(0, 2);

    let difficulty: String = match *current {
        0 => String::from
        ("||         [EASY(9x9)]        ||\n||        MEDIUM(16x16)       ||\n||         HARD(25x25)        ||\n||     ~~~~#MINES: 10~~~~     ||"),
        1 => String::from
        ("||          EASY(9x9)         ||\n||       [MEDIUM(16x16)]      ||\n||         HARD(25x25)        ||\n||     ~~~~#MINES: 50~~~~     ||"),       
        _ =>  String::from
        ("||          EASY(9x9)         ||\n||        MEDIUM(16x16)       ||\n||        [HARD(25x25)]       ||\n||     ~~~~#MINES:100~~~~     ||"),     
    };

    clrscr();
    println!("OO============================OO");
    println!("|| **** MINESWEEPER v0.1 **** ||");
    println!("|| -----------o00o----------- ||");
    println!("||     VERSION 0.1. BY D.     ||");
    println!("|| -------------------------- ||");
    println!("||     ****DIFFICULTY****     ||");
    println!("{}", difficulty);
    println!("|| -------------------------- ||");
    println!("|| [ENTER] Start / Open coor  ||"); 
    println!("|| [^][v][<][>] Move cursor   ||");
    println!("|| [F] Flag selected coor     ||");
    println!("|| [Q] Quit game              ||");
    println!("OO============================OO");
}

fn count_nearby_mines(table: &mut Vec<Vec<String>>, ref_table: &mut Vec<Vec<i8>>, coor: [i8; 2]) {
    let table_size: i8 = ref_table.len() as i8;
    let mut count: i8 = 0;

    if coor[0] < 0 || coor[1] < 0 || 
        coor[0] > table_size - 1 || coor[1] > table_size - 1 { return; }

    let cell: [usize; 2] = [coor[0] as usize, coor[1] as usize];

    if ref_table[coor[0] as usize][coor[1] as usize] != 0 { return; }

    if coor[0] - 1 >= 0 && coor[1] - 1 >= 0 && ref_table[cell[0] - 1][cell[1] - 1] == -1 {
        count += 1;
    }

    if coor[0] - 1 >= 0 && coor[1] - 1 >= 0 && ref_table[cell[0] - 1][cell[1] - 1] == -1 {
        count += 1;
    }
    
    if coor[0] - 1 >= 0 && ref_table[cell[0] - 1][cell[1]] == -1{
        count += 1;
    }

    if coor[0] - 1 >= 0 && coor[1] + 1 < table_size && ref_table[cell[0] - 1][cell[1] + 1] == -1{
        count += 1;
    }

    if coor[1] - 1 >= 0 && ref_table[cell[0]][cell[1] - 1] == -1 {
        count += 1;
    }

    if coor[1] + 1 < table_size && ref_table[cell[0]][cell[1] + 1] == -1 {
        count += 1;
    }

    if coor[0] + 1 < table_size && coor[1] - 1 >= 0 && ref_table[cell[0] + 1][cell[1] - 1] == -1 {
        count += 1;
    }

    if coor[0] + 1 < table_size && ref_table[cell[0] + 1][cell[1]] == -1 {
        count += 1;
    }

    if coor[0] + 1 < table_size && coor[1] + 1 < table_size && ref_table[cell[0] + 1][cell[1] + 1] == -1 {
        count += 1;
    }

    if count != 0 {
        table[cell[0]][cell[1]] = count.to_string();
    } else {
        table[cell[0]][cell[1]] = "_".to_string();
        ref_table[cell[0]][cell[1]] = -2;

        count_nearby_mines(table, ref_table, [coor[0] - 1, coor[1] - 1]);
        count_nearby_mines(table, ref_table, [coor[0] - 1, coor[1]]);
        count_nearby_mines(table, ref_table, [coor[0] - 1, coor[1] + 1]);
        count_nearby_mines(table, ref_table, [coor[0], coor[1] - 1]);
        count_nearby_mines(table, ref_table, [coor[0], coor[1] + 1]);
        count_nearby_mines(table, ref_table, [coor[0] + 1, coor[1] - 1]);
        count_nearby_mines(table, ref_table, [coor[0] + 1, coor[1]]);
        count_nearby_mines(table, ref_table, [coor[0] + 1, coor[1] + 1]);
    }
}

fn main() {
    let mut selected_difficulty: i8 = 1;
    set_difficulty(&mut selected_difficulty, 0);

    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                ..
            }) => set_difficulty(&mut selected_difficulty, -1),

            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => set_difficulty(&mut selected_difficulty, 1),

            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => return,

            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => break,

            _ => ()
        }
    }
    
    let table_size = match selected_difficulty {
        0 => 9,
        1 => 16,
        2 => 25,
        _ => 16
    };
    let mut ref_table: Vec<Vec<i8>> = vec![vec![0; table_size]; table_size];
    let mut table: Vec<Vec<String>> = vec![vec![String::from("*"); table_size]; table_size];
    let mut first_click: bool = true;
    let mut coor: [usize; 2] = [0, 0];
    let mut unopened_count: i16 = 0;
    let mines_count: i16 = match table_size {
        9 => 10,
        16 => 50,
        25 => 100,
        _ => 40
    };

    'listen: loop {
        print_table(&table, table_size, coor, unopened_count);
        match read().unwrap() {
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
                if coor[0] < table_size {
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
                if coor[1] < table_size {
                    coor[1] += 1;
                }
            },

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

            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => return,

            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                if first_click {
                    let mut random = rand::thread_rng();
                    for _ in 0..mines_count {
                        let (mut x, mut y): (usize, usize) = (0, 0);

                        while ref_table[x][y] == -1 || 
                            (((x - coor[0]) as i8).abs() < 3 && ((y - coor[1]) as i8).abs() < 3) {
                            x = random.gen_range(0..table_size);
                            y = random.gen_range(0..table_size);
                        }

                        ref_table[x][y] = -1;
                    }

                    let mut table_str = String::from("[");
                    for i in 0..table_size {
                        table_str += "[";
                        for j in 0..table_size {
                            table_str += &format!("{},", ref_table[i][j]);
                        }
                        table_str += "],";
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

                if ref_table[coor[0]][coor[1]] == -1 {
                    println!("Game over! Press any key to try again");
                    break 'listen;
                } else {
                    count_nearby_mines(&mut table, &mut ref_table, [coor[0] as i8, coor[1] as i8]);
                }

                unopened_count = 0;
                for i in 0..table_size {
                    for j in 0..table_size {
                        if table[i][j] == *"*" || table[i][j] == *"P" {
                            unopened_count += 1;
                        }
                    }
                }

                if unopened_count == mines_count {
                    println!("Congratulations! Press any key to continue");
                    break 'listen;
                }
            },
            _ => ()
        }
    }

    read().unwrap();
    main();
}

//TODO: i8/i16/u...
//DISCARD
/*
fn start_game(table_size: usize) {
    let mut ref_table: Vec<Vec<i8>> = vec![vec![0; table_size]; table_size];
    let mut table: Vec<Vec<String>> = vec![vec![String::from("*"); table_size]; table_size];
    let mut first_click: bool = true;
    let mut coor: [usize; 2] = [0, 0];
    let mut unopened_count: i16 = 0;
    let mines_count: i16 = match table_size {
        9 => 10,
        16 => 50,
        25 => 100,
        _ => 40
    };
    
    'listen: loop {
        print_table(&table, table_size, coor, unopened_count);
        match read().unwrap() {
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
                if coor[0] < table_size {
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
                if coor[1] < table_size {
                    coor[1] += 1;
                }
            },

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

            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => return,

            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                if first_click {
                    let mut random = rand::thread_rng();
                    for _ in 0..mines_count {
                        let (mut x, mut y): (usize, usize) = (0, 0);

                        while ref_table[x][y] == -1 || 
                            (((x - coor[0]) as i8).abs() < 3 && ((y - coor[1]) as i8).abs() < 3) {
                            x = random.gen_range(0..table_size);
                            y = random.gen_range(0..table_size);
                        }

                        ref_table[x][y] = -1;
                    }

                    let mut table_str = String::from("[");
                    for i in 0..table_size {
                        table_str += "[";
                        for j in 0..table_size {
                            table_str += &format!("{},", ref_table[i][j]);
                        }
                        table_str += "],";
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

                if ref_table[coor[0]][coor[1]] == -1 {
                    println!("Game over! Press any key to try again");
                    break 'listen;
                } else {
                    count_nearby_mines(&mut table, &mut ref_table, [coor[0] as i8, coor[1] as i8]);
                }

                unopened_count = 0;
                for i in 0..table_size {
                    for j in 0..table_size {
                        if table[i][j] == *"*" || table[i][j] == *"P" {
                            unopened_count += 1;
                        }
                    }
                }

                if unopened_count == mines_count {
                    println!("Congratulations! Press any key to continue");
                    break 'listen;
                }
            },
            _ => ()
        }
    }

    read().unwrap();
    main();
}
*/
/*
    if coor[0] - 1 >= 0 && coor[1] - 1 >= 0 {
        if ref_table[cell[0] - 1][cell[1] - 1] == -1 {
            count += 1;
        }
    }
    
    if coor[0] - 1 >= 0 {
        if ref_table[cell[0] - 1][cell[1]] == -1 {
            count += 1;
        }
    }

    if coor[0] - 1 >= 0 && coor[1] + 1 < table_size {
        if ref_table[cell[0] - 1][cell[1] + 1] == -1 {
            count += 1;
        }
    }

    if coor[1] - 1 >= 0 {
        if ref_table[cell[0]][cell[1] - 1] == -1 {
            count += 1;
        }
    }

    if coor[1] + 1 < table_size {
        if ref_table[cell[0]][cell[1] + 1] == -1 {
            count += 1;
        }
    }

    if coor[0] + 1 < table_size && coor[1] - 1 >= 0 {
        if ref_table[cell[0] + 1][cell[1] - 1] == -1 {
            count += 1;
        }
    }

    if coor[0] + 1 < table_size {
        if ref_table[cell[0] + 1][cell[1]] == -1 {
            count += 1;
        }
    }

    if coor[0] + 1 < table_size && coor[1] + 1 < table_size {
        if ref_table[cell[0] + 1][cell[1] + 1] == -1 {
            count += 1;
        }
    }
    */
                    /*start_game(match selected_difficulty {
                    0 => 9,
                    1 => 16,
                    2 => 25,
                    _ => 16
                });*/