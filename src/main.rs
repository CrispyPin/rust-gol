extern crate rand;
use std::time;
use std::thread;

const SIZE: usize = 48;
const ALIVE: &str = "[]";
const DEAD: &str = "..";

type Board<T> = [[T; SIZE]; SIZE];


fn main() {
	let mut board: Board<bool> = [[false; SIZE]; SIZE];
	populate(&mut board);
	print!("\x1B[2J"); // clear screen

	loop {
		print!("\x1B[u"); // reset cursor

		#[cfg(debug_assertions)]
		{
			let draw_start = time::Instant::now();
			draw(board);
			println!("\n");
			println!("Time to draw: {:.0?}", draw_start.elapsed());
			
			let step_start = time::Instant::now();
			step(&mut board);
			println!("Time to step: {:.0?}", step_start.elapsed());
		}
		#[cfg(not(debug_assertions))]
		{
			draw(board);
			step(&mut board);
		}

		thread::sleep(time::Duration::from_millis(100));
	}
}

fn step(board: &mut Board<bool>) {
	let mut counts: Board<u8> = [[0; SIZE]; SIZE];
	// count neighbor cells
	for row in 0..SIZE {
		for col in 0..SIZE {
			let mut count: u8 = 0;
			for y in 0..3 {
				for x in 0..3 {
					count += board
							[(row + y + SIZE - 1) % SIZE]
							[(col + x + SIZE - 1) % SIZE] as u8;
				}
			}
			counts[row][col] = count;
		}
	}
	// apply rule
	for y in 0..SIZE {
		for x in 0..SIZE {
			let count = counts[y][x];
			board[y][x] = count == 3 || (count == 4 && board[y][x]);
		}
	}
}

fn populate(board: &mut Board<bool>) {
	for row in board.iter_mut() {
		for cell in row.iter_mut() {
			*cell = rand::random();
		}
	}
}

fn draw(board: Board<bool>) {
	let mut r: String;
	for row in board.iter() {
		r = String::new();
		for cell in row.iter() {
			if *cell {
				r += ALIVE;
			} else {
				r += DEAD;
			}
		}
		println!("{}", r);
	}
}
