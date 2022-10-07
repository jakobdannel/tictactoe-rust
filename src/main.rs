#[derive(Debug)]
#[derive(PartialEq)]
enum Field {
    X,
    O,
    Empty
}

fn main() {
    let board: Vec<Vec<Field>> = vec![vec![Field::X,Field::X,Field::O],
                                      vec![Field::X,Field::O,Field::O],
                                      vec![Field::X,Field::O,Field::O]];
    println!("{:?}",winner(board));
}

fn winner(board: Vec<Vec<Field>>) -> Field {
    let mut winner: Field = Field::Empty;
    let mut count_x;
    let mut count_o;
    for x in 0..3 {
        count_x = 0;
        count_o = 0;
        for y in 0..3 {
            if board[x][y] == Field::X {
                count_x += 1;
            }
            if board[x][y] == Field::O {
                count_o += 1;
            }
        }
        if count_x == 3 {
            winner = Field::X;
        }
        if count_o == 3 {
            winner = Field::O;
        }
    }
    for y in 0..3 {
        count_x = 0;
        count_o = 0;
        for x in 0..3 {
            if board[x][y] == Field::X {
                count_x += 1;
            }
            if board[x][y] == Field::O {
                count_o += 1;
            }
        }
        if count_x == 3 {
            winner = Field::X;
        }
        if count_o == 3 {
            winner = Field::O;
        }
    }
    count_x = 0;
    count_o = 0;
    for i in 0..3 {
        
        if board[i][i] == Field::X {
            count_x += 1;
        }
        if board[i][i] == Field::O {
            count_o += 1;
        }
        
    }
    if count_x == 3 {
        winner = Field::X;
    }
    if count_o == 3 {
        winner = Field::O;
    }
    count_x = 0;
    count_o = 0;
    for i in 0..3 { 
        if board[i][2-i] == Field::X {
            count_x += 1;
        }
        if board[i][2-i] == Field::O {
            count_o += 1;
        } 
    }
    if count_x == 3 {
        winner = Field::X;
    }
    if count_o == 3 {
        winner = Field::O;
    }
    winner
}
