fn main() {
    let mut game:Game = Game::init();
    game.play();
}
#[derive(Copy, Clone)]
enum Square {
    X,
    O
}

impl Square {
    fn to_string(&self)-> String {
        match self {
            Square::X => "X".to_string(),
            Square::O => "O".to_string()
        }
    }
}

struct Grid {
    squares: [Option<Square>; 9]
}

impl Grid {
    fn print(&self) {
        let grid_view: Vec<String>= self.squares.iter().enumerate().map(|(i, square)|{
            match square {
                Some(square_val)=>square_val.to_string(), 
                None => (i+1).to_string()
            }
        }).collect();
        println!("|-|-|-|\n|{}|{}|{}|\n|-|-|-|\n|{}|{}|{}|\n|-|-|-|\n|{}|{}|{}|\n|-|-|-|", &grid_view[0],&grid_view[1], &grid_view[2], &grid_view[3], &grid_view[4], &grid_view[5], &grid_view[6], &grid_view[7], &grid_view[8])
    }
}

struct Game {
    grid: Grid,
    turn: Square,
    winner: Option<Square>
}

impl Game {
    fn init() -> Self {
        Self {
            grid: Grid {squares:[None; 9]},
            turn: Square::X,
            winner: None
        }
    }

    fn take_turn(&mut self) {
        self.grid.print();
        println!("{}'s Turn: ", self.turn.to_string());
        let mut line = String::new();
        let move_position = std::io::stdin().read_line(&mut line).unwrap();
        self.grid.squares[move_position] = Some(self.turn);
        self.change_turn();
    }

    fn change_turn(&mut self) {
        match self.turn {
            Square::X => self.turn = Square::O,
            Square::O => self.turn = Square::X
        }
    }

    fn play(&mut self) {
        let mut end = false;
        while !end {
            self.take_turn();
            end = false;
        }
    }
}
