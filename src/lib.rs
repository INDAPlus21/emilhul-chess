use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 * 
 * PLAN:
 *  - Bitboard LERF (Little-Endian Rank-File mapping) representation
 */

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,

    // Bitboards
    white: u64,
    black: u64,
    pawn: u64,
    knight: u64,
    bishop: u64,
    rook: u64,
    queen: u64,
    king: u64, 
    empty: u64,
    
    // Array move table
    white_pawn_attacks: [u64; 64],
    black_pawn_attacks: [u64; 64],
}

impl Game {
    /// Initializes a new board with pieces.
    pub fn new() -> Game {
        let mut game = Game {
            /* initialize board, set active colour to white, ... */
            state: GameState::InProgress,

            // Inititalise bitboards. LSF bit last
            white:  0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111,
            black:  0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000,  
            pawn:   0b00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000,
            knight: 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            bishop: 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            rook:   0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            queen:  0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            king:   0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            empty:  0b00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000,

            white_pawn_attacks: [0; 64],
            black_pawn_attacks: [0; 64],
        };
        game
    }

    pub fn one_step(dir: &str, bit: u64) -> u64 {
        let not_a_file: u64 = 0b11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110;
        let not_h_file: u64 = 0b01111111_01111111_01111111_01111111_01111111_01111111_01111111_01111111;
        match dir {
            "north" => bit<<8,
            "south" => bit>>8,
            "east" => (bit<<1)&not_a_file,
            "no_ea" => (bit<<9)&not_a_file,
            "so_ea" => (bit>>7)&not_a_file,
            "west" => (bit>>1)&not_h_file,
            "no_we" => (bit<<7)&not_h_file,
            "so_we" => (bit>>9)&not_h_file,
            _ => bit
        }
    }

    fn initialize_move_table(&mut self) {
        let mut square_bit: u64 = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        for i in 0..64 { 
            self.white_pawn_attacks[i] = Game::one_step("no_ea", square_bit)|Game::one_step("no_we", square_bit);
            square_bit>>=1;
        }
        square_bit = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        for i in 0..64 { 
            self.black_pawn_attacks[i] = Game::one_step("so_ea", square_bit)|Game::one_step("so_we", square_bit);
            square_bit>>=1;
        }

    }

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        None
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _postion: String) -> Option<Vec<String>> {
        None
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// rnbqkbnr
/// pppppppp
/// ********
/// ********
/// ********
/// ********
/// PPPPPPPP
/// RNBQKBNR
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Converts the bitboards to an array to then format that as a string.
        
        // Initialize the array
        let mut board: [[char; 8]; 8] = [['*'; 8]; 8];
        
        // Get the intersection for every set of color and piece
        let white_pawn: u64 = self.white&self.pawn;
        let white_knight: u64 = self.white&self.knight;
        let white_bishop: u64 = self.white&self.bishop;
        let white_rook: u64 = self.white&self.rook;
        let white_queen: u64 = self.white&self.queen;
        let white_king: u64 = self.white&self.king;

        let black_pawn: u64 = self.black&self.pawn;
        let black_knight: u64 = self.black&self.knight;
        let black_bishop: u64 = self.black&self.bishop;
        let black_rook: u64 = self.black&self.rook;
        let black_queen: u64 = self.black&self.queen;
        let black_king: u64 = self.black&self.king;
        
        // For every bit check if there's a 1 for all of the color_type intersections
        for i in 0..64 {
            // Shift the bit intersection i spaces and intersect it with 1.
            // This will either yield 0 if the bit in the i:th spot is 0
            // and yields 1 if the bit in the i:th spot is 1
            // If it's 1 then there's a piece there and we add a char symbolizing the specific piece
            // i/8 = rank, i%8 = file
            if ((white_pawn)>>i)&1 == 1 { 
                board[i/8][i%8] = 'P';
            }
            if ((white_knight)>>i)&1 == 1 {
                    board[i/8][i%8] = 'N';
            }
            if ((white_bishop)>>i)&1 == 1 {
                    board[i/8][i%8] = 'B';
            }
            if ((white_rook)>>i)&1 == 1 {
                    board[i/8][i%8] = 'R';
            }
            if ((white_queen)>>i)&1 == 1 {
                    board[i/8][i%8] = 'Q';
            }
            if ((white_king)>>i)&1 == 1 {
                    board[i/8][i%8] = 'K';
            }

            if ((black_pawn)>>i)&1 == 1 {
                board[i/8][i%8] = 'p';
            }
            if ((black_knight)>>i)&1 == 1 {
                    board[i/8][i%8] = 'n';
            }
            if ((black_bishop)>>i)&1 == 1 {
                    board[i/8][i%8] = 'b';
            }
            if ((black_rook)>>i)&1 == 1 {
                    board[i/8][i%8] = 'r';
            }
            if ((black_queen)>>i)&1 == 1 {
                    board[i/8][i%8] = 'q';
            }
            if ((black_king)>>i)&1 == 1 {
                    board[i/8][i%8] = 'k';
            } 
        }

        // Create the string for the output
        let mut board_string: String = String::new();
        for file in 0..8 {
            board_string.push('\n');
            for rank in 0..8 {
                // For every file and rank push that char to the string.
                // 7-rank is added to reverse each rank. To display it as we'd usually represent the board.
                // Since we use lsf but loop over the bit 0..63 we add the ranks in reverse order,
                // Creating a mirrored board.
                board_string.push(board[7-file][rank]);
            }
        }
        write!(f, "{}", board_string)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialization
    #[test]
    fn game_init() {
        println!();
        print!("1... ");
        let mut game = Game::new();
        println!("{:?}", game);
        assert_eq!(game.get_game_state(), GameState::InProgress);
        println!("ok");

        print!("2... ");
        game.initialize_move_table();
        assert_eq!(game.white_pawn_attacks[0], 0);
        println!("ok");
        print!("3... ");
        assert_eq!(game.white_pawn_attacks[9], 0b10100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
        println!("ok");
    }

    #[test]
    fn shift_test() {
        println!();
        print!("1... ");
        let bit: u64 =                              0b00000000_00000000_00000000_10000000_00000000_00000000_00000000_00000000;
        assert_eq!(Game::one_step("north", bit),    0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000);
        println!("ok");
        print!("2... ");
        let bit: u64 =                              0b00000000_00000000_00000000_10000000_00000000_00000000_00000000_00000000;
        assert_eq!(Game::one_step("south", bit),    0b00000000_00000000_00000000_00000000_10000000_00000000_00000000_00000000);
        println!("ok");
        print!("3... ");
        let bit: u64 =                              0b00000000_00000000_00000000_10000000_00000000_00000000_00000000_00000000;
        assert_eq!(Game::one_step("east", bit),     0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
        println!("ok");
        print!("4... ");
        let bit: u64 =                              0b00000000_00000000_00000000_01000000_00000000_00000000_00000000_00000000;
        assert_eq!(Game::one_step("west", bit),     0b00000000_00000000_00000000_00100000_00000000_00000000_00000000_00000000);
        println!("ok");
        print!("5... ");
        let bit: u64 =                              0b00000000_00000000_00000000_01000000_00000000_00000000_00000000_00000000;
        assert_eq!(Game::one_step("no_ea", bit),    0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000);
        println!("ok");
        print!("6... ");
        let bit: u64 =                              0b00000000_00000000_00100000_00000000_00000000_00000000_00000000_00000000;
        assert_eq!(Game::one_step("no_we", bit),    0b00000000_00010000_00000000_00000000_00000000_00000000_00000000_00000000);
        println!("ok");
        print!("7... ");
        let bit: u64 =                              0b00000000_00000000_01000000_00000000_00000000_00000000_00000000_00000000;
        assert_eq!(Game::one_step("so_ea", bit),    0b00000000_00000000_00000000_10000000_00000000_00000000_00000000_00000000);
        println!("ok");
        print!("8... ");
        let bit: u64 =                              0b00000000_00000000_00000000_00000000_00100000_00000000_00000000_00000000;
        assert_eq!(Game::one_step("so_we", bit),    0b00000000_00000000_00000000_00000000_00000000_00010000_00000000_00000000);
        println!("ok");
    }
}
