extern crate rand;
use rand::Rng;

fn print_board(board: &Vec<Vec<u32>>){
    for i in 1..board.len()-1{
        for j in 1..board[i].len()-1{print!("{} " , board[i][j]);}
        println!();
    }
}

fn initialize(board: &mut Vec<Vec<u32>>){
    for i in 1..board.len()-1{
        for j in 1..board[i].len()-1{
            if rand::thread_rng().gen_range(0,10) < 3{board[i][j] = 1;}
            }
    }
}

fn tick(board: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new_board:Vec<Vec<u32>> = vec![vec![0;board[0].len()]; board.len()];
    for row in 1i32..10i32{
        for column in 1i32..10i32{
            let mut neighbors = 0u32;
            for i in -1i32..2i32{
                for j in -1i32..2i32{
                    if !(i == 0 && j == 0){
                     neighbors += board[(row+i) as usize][(column+j) as usize];
                    }
                  }
                }
            if (neighbors == 2 || neighbors == 3) && board[row as usize][column as usize] == 1{
                new_board[row as usize][column as usize] = 1;
                }
            else if board[row as usize][column as usize] == 0  && neighbors == 3 {
                new_board[row as usize][column as usize] = 1;
                }
            else if board[row as usize][column as usize] == 1 && (neighbors < 2 || neighbors > 3){
                new_board[row as usize][column as usize] = 0;
        }
            else{new_board[row as usize][column as usize] = board[row as usize][column as usize];}
     }
    }
    new_board
}

fn main(){
let mut life = vec![vec![0;11]; 11];
initialize(&mut life);
for i in 0..10 {
    print_board(&life);
    life = tick(&life);
    println!("-------------------------------------");
    }
}
