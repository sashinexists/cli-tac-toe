fn main() {
    let mut game: Game = Game::init();
    game.play();
}
#[derive(Copy, Clone, PartialEq)]
enum Square {
    X,
    O,
}

impl Square {
    fn to_string(&self) -> String {
        match self {
            Square::X => "X".to_string(),
            Square::O => "O".to_string(),
        }
    }
}

struct Grid {
    squares: [Option<Square>; 9],
}

impl Grid {
    fn print(&self) {
        let grid_view: Vec<String> = self
            .squares
            .iter()
            .enumerate()
            .map(|(i, square)| match square {
                Some(square_val) => square_val.to_string(),
                None => (i + 1).to_string(),
            })
            .collect();
        // println!(
        //     "|-|-|-|\n|{}|{}|{}|\n|-|-|-|\n|{}|{}|{}|\n|-|-|-|\n|{}|{}|{}|\n|-|-|-|",
        //     &grid_view[0],
        //     &grid_view[1],
        //     &grid_view[2],
        //     &grid_view[3],
        //     &grid_view[4],
        //     &grid_view[5],
        //     &grid_view[6],
        //     &grid_view[7],
        //     &grid_view[8]
        // );
        println!(
            "\n {} {} {} \n {} {} {} \n {} {} {} \n",
            &grid_view[0],
            &grid_view[1],
            &grid_view[2],
            &grid_view[3],
            &grid_view[4],
            &grid_view[5],
            &grid_view[6],
            &grid_view[7],
            &grid_view[8]
        )
    }

    fn end(&self) -> Option<Square> {
        if !self.squares[0].is_none()
            && !self.squares[1].is_none()
            && !self.squares[2].is_none()
            && self.squares[0].unwrap() == self.squares[1].unwrap()
            && self.squares[1].unwrap() == self.squares[2].unwrap()
        {
            self.squares[0]
        } else if !self.squares[3].is_none()
            && !self.squares[4].is_none()
            && !self.squares[5].is_none()
            && self.squares[3].unwrap() == self.squares[4].unwrap()
            && self.squares[4].unwrap() == self.squares[5].unwrap()
        {
            self.squares[3]
        } else if !self.squares[6].is_none()
            && !self.squares[7].is_none()
            && !self.squares[8].is_none()
            && self.squares[6].unwrap() == self.squares[7].unwrap()
            && self.squares[7].unwrap() == self.squares[8].unwrap()
        {
            self.squares[6]
        } else if !self.squares[0].is_none()
            && !self.squares[3].is_none()
            && !self.squares[6].is_none()
            && self.squares[0].unwrap() == self.squares[3].unwrap()
            && self.squares[3].unwrap() == self.squares[6].unwrap()
        {
            self.squares[0]
        } else if !self.squares[1].is_none()
            && !self.squares[4].is_none()
            && !self.squares[7].is_none()
            && self.squares[1].unwrap() == self.squares[4].unwrap()
            && self.squares[4].unwrap() == self.squares[7].unwrap()
        {
            self.squares[1]
        } else if !self.squares[2].is_none()
            && !self.squares[5].is_none()
            && !self.squares[8].is_none()
            && self.squares[2].unwrap() == self.squares[5].unwrap()
            && self.squares[5].unwrap() == self.squares[8].unwrap()
        {
            self.squares[2]
        } else if !self.squares[0].is_none()
            && !self.squares[4].is_none()
            && !self.squares[8].is_none()
            && self.squares[0].unwrap() == self.squares[4].unwrap()
            && self.squares[4].unwrap() == self.squares[8].unwrap()
        {
            self.squares[0]
        } else if !self.squares[2].is_none()
            && !self.squares[4].is_none()
            && !self.squares[6].is_none()
            && self.squares[2].unwrap() == self.squares[4].unwrap()
            && self.squares[4].unwrap() == self.squares[6].unwrap()
        {
            self.squares[2]
        } else {
            None
        }
    }
}

struct Game {
    grid: Grid,
    turn: Square,
    winner: Option<Square>,
}

impl Game {
    fn init() -> Self {
        Self {
            grid: Grid { squares: [None; 9] },
            turn: Square::X,
            winner: None,
        }
    }

    fn take_turn(&mut self) {
        self.grid.print();
        println!("{}'s Turn: ", self.turn.to_string());
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let trimmed_line = line.trim(); // Remove the newline character
        if let Ok(move_position) = trimmed_line.parse::<usize>() {
            self.grid.squares[move_position - 1] = Some(self.turn);
            self.change_turn();
        } else {
            println!("Invalid input. Please enter a valid number.");
        }
    }

    fn change_turn(&mut self) {
        match self.turn {
            Square::X => self.turn = Square::O,
            Square::O => self.turn = Square::X,
        }
    }

    fn is_winner(&self) -> Option<Square> {
        self.grid.end()
    }

    fn play(&mut self) {
        let mut end = false;
        while !end {
            self.take_turn();
            let winner = self.is_winner();
            if !winner.is_none() {
                let winner = winner.unwrap();
                self.grid.print();
                println!("{} wins!", winner.to_string());
                end = true;
            }
        }
    }
}
