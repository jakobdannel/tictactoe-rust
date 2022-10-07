#[derive(Debug, Clone, Copy, PartialEq)]
enum Field {
    X,
    O,
    Empty,
}

fn main() {
    let board: Vec<Vec<Field>> = vec![
        vec![Field::X, Field::X, Field::O],
        vec![Field::X, Field::O, Field::O],
        vec![Field::X, Field::O, Field::O],
    ];
    println!("{:?}", winner(board));
}

fn winner(board: Vec<Vec<Field>>) -> Field {
    let mut winner: Field = Field::Empty;
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
        if board[i][i] == Field::X {
            count_x += 1;
        }
        if board[i][i] == Field::O {
            count_o += 1;
        }
    }
    winner = morethanthree(count_x, count_o, winner);
    //Checks for diagonal wins from top right to bottom left
    count_x = 0;
    count_o = 0;
    for i in 0..3 {
        if board[i][2 - i] == Field::X {
            count_x += 1;
        }
        if board[i][2 - i] == Field::O {
            count_o += 1;
        }
    }
    winner = morethanthree(count_x, count_o, winner);
    winner
}

fn morethanthree(count_x: u32, count_o: u32, previous_winner: Field) -> Field {
    let mut winner: Field = previous_winner;
    if count_x == 3 {
        winner = Field::X;
    }
    if count_o == 3 {
        winner = Field::O;
    }
    winner
}
