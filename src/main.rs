/*
MINESWEEPER
ref_table
    0 = no bomb
    -1 = bomb
    -2 = no bomb, nearby mines counted, no nearby mines
table
    * = unopened
    _ = opened
    P = flagged
    [*] = selected
    1, 2... = bomb(s) around
*/

use rand::Rng;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, exit};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

fn clrscr() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg("cls").status().unwrap();
    } else {
        Command::new("sh").arg("-c").arg("clear").status().unwrap();
    }
}

fn update_disp(ref_table: &Vec<Vec<i8>>, table: &Vec<Vec<String>>, table_size: usize, coor: &[usize; 2]) {
    clrscr();

    print!("\r^_^|");
    for i in 0..table_size {
        print!(" {:02}", i);
    }

    print!("\n---+");
    for _ in 0..table_size {
        print!("---");
    }
    print!("\n");

    for i in 0..table_size {
        print!("{:02} | ", i);
        for j in 0..table_size {
            if i == coor[0] && j == coor[1] {
                print!("[{}]", table[i][j]);
            } else {
                print!(" {} ", table[i][j]);
            }
        }
        println!("");
    }
}

fn set_difficulty(current: &mut i8, new: i8) {
    let difficulty: String;
    *current += new;
    if *current < 0 {
        *current = 0;
    } else if *current > 2 {
        *current = 2;
    }
    match *current {
        0 => difficulty = String::from
        ("||         [EASY(9x9)]        ||\n||        MEDIUM(16x16)       ||\n||         HARD(25x25)        ||\n||     ~~~~#MINES: 10~~~~     ||"),
        1 => difficulty = String::from
        ("||          EASY(9x9)         ||\n||       [MEDIUM(16x16)]      ||\n||         HARD(25x25)        ||\n||     ~~~~#MINES: 50~~~~     ||"),       
        _ =>  difficulty = String::from
        ("||          EASY(9x9)         ||\n||        MEDIUM(16x16)       ||\n||        [HARD(25x25)]       ||\n||     ~~~~#MINES:100~~~~     ||"),     
    }

    clrscr();
    println!("OO============================OO");
    println!("|| **** MINESWEEPER v0.1 **** ||");
    println!("|| -----------o00o----------- ||");
    println!("||     VERSION 0.1. BY D.     ||");
    println!("|| -------------------------- ||");
    println!("||     ****DIFFICULTY****     ||");
    println!("{}", difficulty);
    println!("|| -------------------------- ||");
    println!("|| [ENTER] Start / Open cell  ||"); 
    println!("|| [^][v][<][>] Move cursor   ||");
    println!("|| [F] Flag selected cell     ||");
    println!("|| [Q] Quit game              ||");
    println!("OO============================OO");
}

fn count_mines(table: &mut Vec<Vec<String>>, ref_table: &mut Vec<Vec<i8>>, cell: [i8; 2]) {
    if cell[0] < 0 || cell[1] < 0 { return; }
    let coor: [usize; 2] = [cell[0] as usize, cell[1] as usize];
    if ref_table[coor[0]][coor[1]] != 0 { return; }

    let mut count: i8 = 0;
    let table_size: i8 = ref_table.len() as i8;
    if cell[0] - 1 >= 0 && cell[1] - 1 >= 0 {
        if ref_table[coor[0] - 1][coor[1] - 1] == -1 {
            count += 1;
        }
    }
    if cell[0] - 1 >= 0 {
        if ref_table[coor[0] - 1][coor[1]] == -1 {
            count += 1;
        }
    }
    if cell[0] - 1 >= 0 && cell[1] + 1 < table_size {
        if ref_table[coor[0] - 1][coor[1] + 1] == -1 {
            count += 1;
        }
    }
    if cell[1] - 1 >= 0 {
        if ref_table[coor[0]][coor[1] - 1] == -1 {
            count += 1;
        }
    }
    if cell[1] + 1 < table_size {
        if ref_table[coor[0]][coor[1] + 1] == -1 {
            count += 1;
        }
    }
    if cell[0] + 1 < table_size && cell[1] - 1 >= 0 {
        if ref_table[coor[0] + 1][coor[1] - 1] == -1 {
            count += 1;
        }
    }
    if cell[0] + 1 < table_size {
        if ref_table[coor[0] + 1][coor[1]] == -1 {
            count += 1;
        }
    }
    if cell[0] + 1 < table_size && cell[1] + 1 < table_size {
        if ref_table[coor[0] + 1][coor[1] + 1] == -1 {
            count += 1;
        }
    }
    if count != 0 {
        table[coor[0]][coor[1]] = count.to_string();
    } else {
        table[coor[0]][coor[1]] = "_".to_string();
        ref_table[coor[0]][coor[1]] = -2;
        count_mines(table, ref_table, [cell[0] - 1, cell[1] - 1]);
        count_mines(table, ref_table, [cell[0] - 1, cell[1]]);
        count_mines(table, ref_table, [cell[0] - 1, cell[1] + 1]);
        count_mines(table, ref_table, [cell[0], cell[1] - 1]);
        count_mines(table, ref_table, [cell[0], cell[1] + 1]);
        count_mines(table, ref_table, [cell[0] + 1, cell[1] - 1]);
        count_mines(table, ref_table, [cell[0] + 1, cell[1]]);
        count_mines(table, ref_table, [cell[0] + 1, cell[1] + 1]);
    }
}

fn on_click(ref_table: &mut Vec<Vec<i8>>, table: &mut Vec<Vec<String>>, coor: &[usize; 2]) -> bool {
    if ref_table[coor[0]][coor[1]] == -1 {
        println!("Game over! Press any key to try again");
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => main(),
            _ => main()
        }
        return false;
    } else {
        count_mines(table, ref_table, [coor[0] as i8, coor[1] as i8]);
        return true;
    }
}

fn start_game(table_size: usize) {
    let mut ref_table: Vec<Vec<i8>> = vec![vec![0; table_size]; table_size];
    let mut table: Vec<Vec<String>> = vec![vec![String::from("*"); table_size]; table_size];
    let mut first_click: bool = true;
    let mut cell: [usize; 2] = [0, 0];
    let mut opened_count: i16 = 0;
    let num_mines: i16 = match table_size {
        9 => 10,
        16 => 50,
        25 => 100,
        _ => 40
    };
    
    'listen: loop {
        update_disp(&ref_table, &table, table_size, &cell);
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                ..
            }) => {
                if cell[0] > 0 {
                    cell[0] -= 1;
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                if cell[0] < table_size {
                    cell[0] += 1;
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                if cell[1] > 0 {
                    cell[1] -= 1;
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                if cell[1] < table_size {
                    cell[1] += 1;
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('f'),
                ..
            }) => {
                if table[cell[0]][cell[1]] == "*" {
                    table[cell[0]][cell[1]] = "P".to_string();
                } else if table[cell[0]][cell[1]] == "P" {
                    table[cell[0]][cell[1]] = "*".to_string();
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
                    for _ in 0..num_mines {
                        let (mut x, mut y): (i8, i8) = (0, 0);
                        while ref_table[x as usize][y as usize] == -1 || ((x - cell[0] as i8).abs() < 3 && (y - cell[1] as i8).abs() < 3) {
                            x = random.gen_range(0..table_size as i8);
                            y = random.gen_range(0..table_size as i8);
                        }
                        ref_table[x as usize][y as usize] = -1;
                    }

                    let mut debug = String::from("[");
                    for i in 0..table_size {
                        debug += "[";
                        for j in 0..table_size {
                            debug += &format!("{},", ref_table[i][j]);
                        }
                        debug += "],";
                    }
                    debug += "]";
                    
                    let mut file = File::create("debug.txt");
                    file.unwrap().write_all(debug.as_bytes());

                    first_click = false;
                }
                if !on_click(&mut ref_table, &mut table, &cell) {
                    break 'listen;
                }
                opened_count += 1;
                if opened_count == (table_size * table_size) as i16 - num_mines {
                    println!("Congratulations!");
                    break 'listen;
                }
            },
            _ => ()
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
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
                code: KeyCode::Enter,
                ..
            }) => {
                start_game(match selected_difficulty {
                    0 => 9,
                    1 => 16,
                    2 => 25,
                    _ => 16
                });
                break;
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => return,
            _ => ()
        }
    }
}

//DISCARD
//use rdev::{listen, Event, EventType, Key};
/*
        listen(move |event| {
            match event.event_type {
                EventType::KeyRelease(Key::UpArrow) => {
                    let mut clicked = false;
                    if cell[0] > 0 && !clicked {
                        cell[0] -= 1;
                        clicked = true;
                    }
                    update_disp(&table, table_size, &cell);
                    thread::sleep(time::Duration::from_millis(1000));
                    //clicked = false;
                },
                EventType::KeyRelease(Key::DownArrow) => {
                    let mut clicked = false;
                    if cell[0] < table_size {
                        cell[0] += 1;
                        clicked = true;
                    }
                    update_disp(&table, table_size, &cell);
                    thread::sleep(time::Duration::from_millis(1000));
                    clicked = false;
                },
                EventType::KeyRelease(Key::LeftArrow) => {
                    let mut clicked = false;
                    if cell[1] > 0 {
                        cell[1] -= 1;
                        clicked = true;
                    }
                    update_disp(&table, table_size, &cell);
                    thread::sleep(time::Duration::from_millis(1000));
                    clicked = false;
                },
                EventType::KeyRelease(Key::RightArrow) => {
                    let mut clicked = false;
                    if cell[1] < table_size {
                        cell[1] += 1;
                        clicked = true;
                    }
                    update_disp(&table, table_size, &cell);
                    thread::sleep(time::Duration::from_millis(1000));
                    clicked = false;
                },
                EventType::KeyRelease(Key::KeyF) => {
                    let mut clicked = false;
                    if table[cell[0]][cell[1]] == "*" {
                        table[cell[0]][cell[1]] = "|>".to_string();
                    } else if table[cell[0]][cell[1]] == "|>" {
                        table[cell[0]][cell[1]] = "*".to_string();
                    }
                    clicked = true;
                    update_disp(&table, table_size, &cell);
                    thread::sleep(time::Duration::from_millis(1000));
                    clicked = false;
                },
                EventType::KeyRelease(Key::KeyQ) => process::exit(0x01),
                _ => ()
            }
        });
        */
        /*
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.to_lowercase() == "q\r\n" {
            return;
        } else if input.to_lowercase() == "debug\r\n" {
            for i in 0..=15 {
                print!("{:02} | ", i);
                for j in 0..=15 {
                    if ref_table[i][j] != 0 {
                        print!("1 ");
                    } else {
                        print!("0 ");
                    }
                }
                println!("");
            }
            std::io::stdin().read_line(&mut input).unwrap();
            return;
        }
        
        let mut cell: [usize; 2] = [ 
        match &input[0..1].to_uppercase() as &str {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            "D" => 3,
            "E" => 4,
            "F" => 5,
            "G" => 6,
            "H" => 7,
            "I" => 8,
            "J" => 9,
            "K" => 10,
            "L" => 11,
            "M" => 12,
            "N" => 13,
            "O" => 14,
            "P" => 15,
            _ => 16
        }
        , match input[1..3].parse::<usize>() {
            Ok(_) => input[1..3].parse::<usize>().unwrap(),
            Err(_) => 16
        }];
        
        
        if cell[0] == 16 || cell[1] == 16 {
            println!("Error: Invalid input!");
        } else {
            opened_
            if &input[3..4].to_lowercase() == "f" {
                table[cell[0]][cell[1]] = "|>".to_string();
            } else if ref_table[cell[0]][cell[1]] == -1 {
                print!("Game over! Try again? (Y/N):");
                //std::io::stdin().read_line(&mut input).unwrap();
                if input.to_lowercase() == "y\r\n" {
                    //
                } else {
                    println!("Goodbye!");
                    return;
                }
            } else {/*
                let mut nearby_mines: i8 = 0;
                if ref_table[cell[0] - 1][cell[1] - 1] {
                    nearby_mines += 1;
                }
                if ref_table[cell[0]][cell[1] - 1] {
                    nearby_mines += 1;
                }
                if ref_table[cell[0] + 1][cell[1] - 1] {
                    nearby_mines += 1;
                }
                if ref_table[cell[0] - 1][cell[1]] {
                    nearby_mines += 1;
                }
                if ref_table[cell[0] + 1][cell[1]] {
                    nearby_mines += 1;
                }
                if ref_table[cell[0] - 1][cell[1] + 1] {
                    nearby_mines += 1;
                }
                if ref_table[cell[0]][cell[1] + 1] {
                    nearby_mines += 1;
                }
                if ref_table[cell[0] + 1][cell[1] + 1] {
                    nearby_mines += 1;
                }
                table[cell[0]][cell[1]] = nearby_mines.to_string();
                opened_*/
            }
            if opened_count == 256 - num_mines {
                println!("Congratulations!");
            }
        }*/
/*
    listen(move |event| {
        match event.event_type {
            EventType::KeyRelease(Key::UpArrow) => set_difficulty(&mut selected_difficulty, -1),
            EventType::KeyRelease(Key::DownArrow) => set_difficulty(&mut selected_difficulty, 1),
            EventType::KeyRelease(Key::Return) => start_game(match selected_difficulty {
                0 => 9,
                1 => 16,
                2 => 25,
                _ => 16
            }),
            EventType::KeyRelease(Key::KeyQ) => return,
            _ => ()
        }
    });*/
//let mut mines_count: i16 = 0;
    /*'seed: for i in 0..table_size  {
        for j in 0..table_size {
            let mine: bool = rand::random();
            if mine {
                mines_ //TODO: OVERFLOW
                ref_table[i][j] = -1;
            }
            if mines_count == num_mines {
                break 'seed;
            }
        }

    }*/
    /* 
    let mut random = rand::thread_rng();
    for _ in 0..num_mines {
        let (mut x, mut y): (usize, usize);
        x = random.gen_range(0..table_size);
        y = random.gen_range(0..table_size);
        while ref_table[x][y] == -1 {
            x = random.gen_range(0..table_size);
            y = random.gen_range(0..table_size);
        }
        ref_table[x][y] = -1;
    }
    */
    /*
        if count_mines(table, ref_table, &coor) != 0 {
            ref_table[coor[0]][coor[1]] = count_mines(table, ref_table, &coor);
        } else {
            //TODO: 
            table[coor[0]][coor[1]] = "_".to_string();
        }*/