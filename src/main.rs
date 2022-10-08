use std::io::{Write, stdout, stdin};
#[derive(Debug, Clone, Copy, PartialEq)]
enum Square {
    X,
    O,
    Empty,
}

fn main() {
    let board: Vec<Vec<Square>> = vec![
        vec![Square::X, Square::X, Square::O],
        vec![Square::X, Square::O, Square::O],
        vec![Square::X, Square::O, Square::O],
    ];

    print!("Please enter the playing field:");
    let mut input = String::new();
    stdout().flush().ok();
    match stdin().read_line(&mut input) {
        Ok(_n) => println!("{}", input),
        Err(e) => println!("error: {}", e),
    }

    println!("{:?}", winner(board));
}

fn winner(board: Vec<Vec<Square>>) -> Square {
    let mut winner: Square = Square::Empty;
    let mut count_x;
    let mut count_o;
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

fn morethantwo(count_x: u32, count_o: u32, previous_winner: Square) -> Square {
    let mut winner: Square = previous_winner;
    if count_x == 3 {
        winner = Square::X;
    }
    if count_o == 3 {
        winner = Square::O;
    }
    winner
}

 
