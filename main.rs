pub fn new() -> Self {
		Self {
			board: ['.', 42],
			turn: 0, //turn == 0 -> Os tur
		}
	}

pub fn place_bricka(&self, movi: i64) -> bool {
		let mut was_valid: bool = false;
		for i in 0..6 {
			let x = movi+i*6;
			if self.board[x as usize] == 'O' or self.board[x as usize] == 'X' {
				continue;
			}
			if self.turn == 0 {self.board[x as usize] = 'O'}
			else {self.board[x as usize] = 'X'}
			was_valid = true;
		}
		if !was_valid {
			println!("Invalid Move!");
			return false;
		}
		self.turn = 1-self.turn;
		return true;
	}

pub fn has_won(&self) -> char {
		for i in 0..42 {
			if self.board[i as usize] != '.' {
				let x : i64 = i%8;
				let y : i64 = i/8;
				let left = x-3;
				let up = y+3;
				let right = x+3;
				let mut has_winner: bool = false;
				
				let diagl1 = (x-1)+(y+1)*6;
				let diagl2 = (x-2)+(y+2)*6;
				let diagl3 = (x-3)+(y+3)*6;
				
				let left1 = i-1;
				let left2 = i-2;
				let left3 = i-3;

				let up1 = i+6;
				let up2 = i+12;
				let up3 = i+18;

				let diagr1 = (x+1)+(y+1)*6;
				let diagr2 = (x+2)+(y+2)*6;
				let diagr3 = (x+3)+(y+3)*6;

	
				if (left >= 0 && up < 7 && self.board[i as usize] == self.board[diagl1 as usize] && self.board[i] == self.board[diagl2 as usize] && self.board[i as usize] == self.board[diagl3 as usize]) || (left >= 0 && self.board[i as usize] == self.board[left1 as usize] && self.board[i as usize] == self.board[left2 as usize] && self.board[i as usize] == self.board[left3 as usize]) || (up < 7 && self.board[i as usize] == self.board[up1 as usize] && self.board[i as usize] == self.board[up2 as usize] && self.board[i as usize] == self.board[up3 as usize]) || (right < 7 && up < 7 && self.board[i as usize] == self.board[diagr1 as usize] && self.board[i as usize] == self.board[diagr2 as usize] && self.board[i as usize] == self.board[diagr3 as usize]){has_winner = true;}

				if has_winner {return self.board[i as usize]}
				
			}

			return '.';
			
		}
	
	}
