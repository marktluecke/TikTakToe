use std::io;
use std::error::Error;
use std::fmt;
use std::process;  
use rand::Rng; 


enum MinimaxRet {
    Score(i8),
    ScoreMove((i8, i8)),
}

#[derive(Debug)]
struct SpaceOccupied {
    details: String,
}

impl fmt::Display for SpaceOccupied {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}
impl SpaceOccupied {
    fn new(msg: &str) -> SpaceOccupied {
        SpaceOccupied{details: msg.to_string()}
    }
}

impl Error for SpaceOccupied {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
struct Board {
    field: [[i8; 3]; 3],
    ended: bool,
    winner: i8,
    c_player: bool,
    bot_player: i8,
}

impl Board {
    fn new(bot_player: i8) -> Self {
        return Self {
            field: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
            ended: false,
            winner: 0,
            c_player: true,
            bot_player: bot_player,
        };
    }

    fn make_move(&mut self, index: i8) -> Result<Board, SpaceOccupied> {
        let y: usize = {index / 3} as usize;
        let x: usize = {index % 3} as usize;

        let mut return_field = Board {
            field: self.field,
            ended: self.ended,
            winner: self.winner,
            c_player: self.c_player,
            bot_player: self.bot_player,
        };

        if self.c_player {
            if self.field[y][x] != 0 {
                return Err(SpaceOccupied::new("Space already occupied!"));
            }
            return_field.field[y][x] = 1;
            return_field.c_player = false;

        } else {
            if self.field[y][x] != 0 {
                return Err(SpaceOccupied::new("Space already occupied!"));
            }
            return_field.field[y][x] = -1;
            return_field.c_player = true;
        }
        return_field.check_for_end();
        Ok(return_field)
    }

    fn get_moves(&mut self) -> Vec<i8> {
        let mut moves: Vec<i8> = Vec::new();

        for (y, row) in self.field.iter().enumerate() { 
            for (x, col) in row.iter().enumerate() {
                if *col == 0 {
                    moves.push((y * 3 + x) as i8); 
                }
            }
        }
        return moves;
    }

    fn get_depth(&mut self) -> i8 {
        let mut emtpty_spaces: i8 = 0;

        for row in self.field.iter() {
            for col in row.iter() {
                if *col == 0 {
                    emtpty_spaces += 1;
                }
            }
        }
        return 9 - emtpty_spaces;
    }
    
    fn check_for_end(&mut self) {
        if self.get_moves().is_empty() {
            self.ended = true;
        }
        self.check_for_win();
    }

    fn check_for_win(&mut self) {
        for num in 0..3 {
            let start = self.field[0][num];
            if start == self.field[1][num] && start == self.field[2][num] && start != 0 {
                self.winner = start;
                self.ended = true;
            }
        }
    
        for num in 0..3 {
            let start = self.field[num][0];
            if start == self.field[num][1] && start == self.field[num][2] && start != 0 {
                self.winner = start;
                self.ended = true;
            }
        }
    
        if self.field[0][0] == self.field[1][1] && self.field[0][0] == self.field[2][2] && self.field[0][0] != 0 {
            self.winner = self.field[0][0];
            self.ended = true;
        }
        if self.field[0][2] == self.field[1][1] && self.field[0][2] == self.field[2][0] && self.field[0][2] != 0 {
            self.winner = self.field[0][2];
            self.ended = true;
        }       
    }

    fn print(&self) {
        let symbol_empty: &str = "   ";
        let symbol_circle: &str = " O ";
        let symbol_cross: &str = " X ";

        let mut out = String::from("\n");

        for (i, row) in self.field.iter().enumerate() { 
            for (y, col) in row.iter().enumerate() {
                if *col == 0 {
                    out.push_str(&symbol_empty);
                } else if *col == -1 {
                    out.push_str(&symbol_circle);
                } else if *col == 1 {
                    out.push_str(&symbol_cross);
                }
                if y != 2 {
                    out.push_str("|")
                }
            }
            out.push_str("\n"); 
            if i != 2 {
                out.push_str("-----------\n");
            }     
        }
        println!("{}", out);
    }
}


fn get_user_input(field: &mut Board) -> i8 {
    loop {
        if field.c_player {
            println!("It's X's turn! Select an empty space!");
        } else {
            println!("It's O's turn! Select an empty space!");
        }
        

        let mut index = String::from("");

        io::stdin().read_line(&mut index).expect("Failed to read line!");
        let index: i8 = match index.trim().parse() {
            Ok(indx) => indx,
            Err(_) => {println!("Please enter a valid number! (0-8)"); continue},
        };

        if index < 9 && index >= 0 {
            if field.get_moves().contains(&index) {
                return index;
            } else {
                println!("Space already occupied!")
            }
            
        } else {
            println!("Index must be between 0 and 8!")
        }
    }
}

fn explain() {
    println!("\n\n\nWelcome to TikTakToe!");
    println!("---------------------\n\n\n");
    println!("The spaces of the TikTakToe-Board are indexed from 0-8.\n");
    println!(" 0 | 1 | 2 \n 3 | 4 | 5 \n 6 | 7 | 8 \n");
    println!("To play you have to enter a valid index and your symbol will be placed!\n\n");
}

fn setup() -> Board {
    println!("How many human players are there? (1/2): ");
    loop {
        let mut players: String = String::from("");
        io::stdin().read_line(&mut players).expect("Failed to read line!");
        let players: u8 = match players.trim().parse() {
            Ok(players) => players,
            Err(_) => {println!("Please enter a valid number! (1/2): "); continue;}
        }; 

        if players == 2 {
            println!("Two players! Have fun!");
            return Board::new(0);
        } else if players == 1 {
            println!("\nYou versus a bot! Do you want to be X or O? (x/o): ");
            loop {
                let mut start: String = String::from("");
                io::stdin().read_line(&mut start).expect("Failed to read line!");

                if start.trim().to_uppercase() == String::from("X") {
                    return Board::new(-1);
                } else if start.trim().to_uppercase() == String::from("O") {
                    return Board::new(1);
                } else {
                    println!("Please enter a valid response! (x/o): ")
                }
            }  
        } else {
            println!("Please enter a valid number! (1/2): ");
        }
    }
}

fn rand_best_mv(scores: &Vec<i8>) -> usize {
    let max_score = match scores.iter().max() {       
        Some(max) => *max,
        _ => 0,
    };

    let mut max_score_indexes: Vec<usize> = Vec::new();
    for (indx, score) in scores.iter().enumerate() {
        if *score == max_score {
            max_score_indexes.push(indx)
        }
    }
    let mut rng = rand::thread_rng();
    return max_score_indexes[rng.gen_range(0..max_score_indexes.len())];
}

fn score(field: &mut Board) -> i8 { 
    if field.winner == 0 {
        return 0;
    } else if field.bot_player == field.winner {
        return 11 - field.get_depth();            
    } else if field.bot_player != field.winner {
        return -11 + field.get_depth();
    }
    return 0;
}

fn minimax(field: &mut Board) -> MinimaxRet {
    if field.ended {return MinimaxRet::Score(score(field));}

    let mut scores: Vec<i8> = Vec::new();
    let mut next_moves: Vec<i8> = Vec::new();

    for possible_move in field.get_moves().iter() {
        let mut possible_game = field.make_move(*possible_move).expect("Error in Logic: Space already in use!");
        match minimax(&mut possible_game) {
            MinimaxRet::Score(score) => {scores.push(score);},
            MinimaxRet::ScoreMove((score, _b_move)) => {scores.push(score);},
        }
        next_moves.push(*possible_move);       
    } 

    if (!field.c_player && field.bot_player == -1) | (field.c_player && field.bot_player == 1) { // bot ist immer maximizer
        let index = rand_best_mv(&scores);
        return MinimaxRet::ScoreMove((scores[index], next_moves[index]));
             
    } else if (field.c_player && field.bot_player == -1) | (!field.c_player && field.bot_player == 1) {
        let min_score = match scores.iter().min() {
            Some(min) => *min,
            _ => 0,
        };
        let min_score_index = scores.iter().position(|&r| r == min_score).unwrap(); 
        return MinimaxRet::ScoreMove((min_score, next_moves[min_score_index]));

    } else {
        println!("Fatal Error!");
        return MinimaxRet::Score(-99)
    }
}

fn main() {
    explain();
    loop {
        let mut field: Board = setup();
        println!("\n\nPlay TikTakToe: \n");

        loop {
            field.print();
            if field.bot_player == 0 {
                let index = get_user_input(&mut field);
                field = field.make_move(index).expect("Error");
            } else {
                if (field.c_player && field.bot_player == -1) | (!field.c_player && field.bot_player == 1) {
                    let index = get_user_input(&mut field);
                    field = field.make_move(index).expect("Error");
        
                } else {
                    match minimax(&mut field) {
                        MinimaxRet::Score(score) => {println!("{:?}", score);},
                        MinimaxRet::ScoreMove((_score, b_move)) => {
                            field = field.make_move(b_move).expect("Error");
                        },
                    }
                }
            }
        
            if field.ended {
                match field.winner {
                    -1 => {field.print(); println!("O won!");  break},
                    1 => {field.print(); println!("X won!");  break},
                    0 => {field.print(); println!("It's a tie!"); break},
                    _ => (),
                }
            }         
        }
        println!("\nDo you want to play again? (y/n): ");
        loop {
            let mut again: String = String::from("");
            io::stdin().read_line(&mut again).expect("Failed to read line!");

            if again.trim().to_uppercase() == String::from("Y") {
                break;
            } else if again.trim().to_uppercase() == String::from("N") {
                process::exit(0x000);
            } else {
                println!("Please enter a valid response! (y/n): ")
            }
        } 
    }  
}
