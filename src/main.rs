extern crate rand;
use std::time;
use std::thread;

const SIZE: usize = 48;

type Board = ( [[bool; SIZE]; SIZE] );
type BoardCount = ( [[u8; SIZE]; SIZE] );


fn main() {
	let mut board: Board = [[false; SIZE]; SIZE];
	//draw(board);
	populate(&mut board);
	loop {
		println!("\n\n\n\n\n\n\n\n\n\n\n\n");
		draw(board);
		step(&mut board);
		thread::sleep(time::Duration::from_millis(100));
	}
}

fn step(board: &mut Board) {
	let mut counts: BoardCount = [[0; SIZE]; SIZE];
	for row in 0..SIZE {
		for col in 0..SIZE {
			let mut count: u8 = 0;
			for y in 0..3 {
				for x in 0..3 {
					if board
					[(row + y + SIZE - 1) % SIZE]
					[(col + x + SIZE - 1) % SIZE] {
						count += 1;
					}
				}
			}
			counts[row][col] = count;
		}
	}
	for y in 0..SIZE {
		for x in 0..SIZE {
			let count = counts[y][x];
			
			if board[y][x] {
				board[y][x] = count == 3 || count == 4;
			} else {
				board[y][x] = count == 3;
			}
		}
	}
}


fn populate(board: &mut Board) {
	for row in board.iter_mut() {
		for cell in row.iter_mut() {
			*cell = rand::random();
		}
	}
}

fn draw(board: Board) {
	for row in board.iter() {
		for cell in row.iter() {
			if *cell {
				print!("[]");
			} else {
				print!("  ");
			}
		}
		println!();
	}
}
