use std::io::{stdin, stdout, Write};
#[derive(Debug, Clone, Copy, PartialEq)]
enum Square {
    X,
    O,
    Empty,
}

impl From<&str> for Square {
    ///Converts an String input into a Square enum
    fn from(input: &str) -> Square {
        let output: Square;
        if input == "X" {
            output = Square::X;
        } else if input == "O" {
            output = Square::O;
        } else {
            output = Square::Empty;
        }
        output
    }
}

fn main() {
    print!("Please enter the playing field (X,O, ,...):");
    let mut input: String = String::new();
    stdout().flush().ok();
    match stdin().read_line(&mut input) {
        Ok(_n) => println!("Input accepted."),
        Err(e) => println!("error: {}", e),
    }
    //Removes the newline from the string
    input.pop();
    //Converts string to one dimensional vector with Square enum type
    let temp_board: Vec<Square> = input
        .split_terminator(",")
        .filter(|x| ["X", "O", " "].contains(x))
        .map(|s| s.into())
        .collect();
    //Convert one dimensional vector to two dimensions
    let board: Vec<_> = temp_board.chunks(3).map(|slice| slice.to_vec()).collect();
  
    print_board(&board);
    match winner(board) {
        Ok(_n) => println!("{:?}", _n),
        Err(e) => println!("Dimensions of given board are not 3x3. Reason: {}", e)
    }
}

///Determines the winner of a given board
fn winner(board: Vec<Vec<Square>>) -> Result<Square,String> {
    let mut winner: Square = Square::Empty;
    let mut count_x: usize;
    let mut count_o: usize;
    //Check if given board is valid
    if board.len() != 3 {
        return Err("Wrong amount of rows.".to_string());
    }
    if board[2].len() != 3 {
        return Err("Wrong amount of columns.".to_string());
    }

    //Checks for vertical and horizontal wins
    for x in 0..3 {
        if board[x][0] == board[x][1] && board[x][1] == board[x][2] {
            return Ok(board[x][0]);
        }
        if board[0][x] == board[1][x] && board[1][x] == board[2][x] {
            return Ok(board[0][x]);
        }
    }
    count_x = 0;
    count_o = 0;
    //Checks for diagonal wins from top left to bottom right
    for i in 0..3 {
        if board[i][i] == Square::X {
            count_x += 1;
        }
        if board[i][i] == Square::O {
            count_o += 1;
        }
    }
    winner = morethantwo(count_x, count_o, winner);
    //Checks for diagonal wins from top right to bottom left
    count_x = 0;
    count_o = 0;
    for i in 0..3 {
        if board[i][2 - i] == Square::X {
            count_x += 1;
        }
        if board[i][2 - i] == Square::O {
            count_o += 1;
        }
    }
    winner = morethantwo(count_x, count_o, winner);
    Ok(winner)
}

///Checks if counter variables are larger than two and keeps the previous winner if not
fn morethantwo(count_x: usize, count_o: usize, previous_winner: Square) -> Square {
    let mut winner: Square = previous_winner;
    if count_x == 3 {
        winner = Square::X;
    }
    if count_o == 3 {
        winner = Square::O;
    }
    winner
}

///Prints a given board with borders
fn print_board(board: &Vec<Vec<Square>>) {
    println!("+---+---+---+");
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            if board[x][y] != Square::Empty {
                print!("| {:?} ", board[x][y]);
            } else {
                print!("|   ");
            }
        }
        print!("|\n");
        println!("+---+---+---+");
    }
}
