use std::io::{Write, stdout, stdin};
#[derive(Debug, Clone, Copy, PartialEq)]
enum Square {
    X,
    O,
    Empty,
}

fn main() {
    print!("Please enter the playing field (X,O, ,...):");
    let mut input: String = String::new();
    stdout().flush().ok();
    match stdin().read_line(&mut input) {
        Ok(_n) => println!("Input accepted."),
        Err(e) => println!("error: {}", e),
    }
    input.pop(); //Removes the newline from the string
    let temp_board: Vec<Square>= input.split_terminator(",").filter(|x| ["X","O"," "].contains(x)).map(|s| {convert_string_to_square(s)}).collect(); //Converts string to one dimensional vector with Square enum type
    let board: Vec<_> = temp_board.chunks(3).map(|slice| slice.to_vec()).collect(); //Convert one dimensional vector to two dimensions
    print_board(&board);
    println!("Winner: {:?}", winner(board));
}

fn winner(board: Vec<Vec<Square>>) -> Square {
    let mut winner: Square = Square::Empty;
    let mut count_x: usize;
    let mut count_o: usize;
    //Checks for vertical and horizontal wins
    for x in 0..3 {
        if board[x][0] == board[x][1] && board[x][1] == board[x][2] {
            return board[x][0];
        }
        if board[0][x] == board[1][x] && board[1][x] == board[2][x] {
            return board[0][x];
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
    winner
}

fn morethantwo(count_x: usize, count_o: usize, previous_winner: Square) -> Square { //Checks if counter variables are larger than two and keeps the previous winner if not
    let mut winner: Square = previous_winner;
    if count_x == 3 {
        winner = Square::X;
    }
    if count_o == 3 {
        winner = Square::O;
    }
    winner
}

fn print_board(board: &Vec<Vec<Square>>) {
    println!("+---+---+---+");
    for x in 0..3 {
        for y in 0..3 {
            if board[x][y] != Square::Empty {
                print!("| {:?} ",board[x][y]);
            } else {
                print!("|   ");
            }
        }
        print!("|\n");
       println!("+---+---+---+");
    }
} 

fn convert_string_to_square (input: &str) -> Square { //Converts an String input into a Square enum
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