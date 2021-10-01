use std::fmt;
mod tests;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

pub struct Game {
    /* Then do checks and pins, porbably pretty easy. Promotion. Add turn timer.
     * If I have extra time first fix good representative unit tests,
     * then document the code and lastly clean up the code. If all this is done great! Go to sleep, you've earned it :)
     */
    state: GameState,
    turn: usize,
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
    // Move table
    white_pawn_attacks: [u64; 64],
    black_pawn_attacks: [u64; 64],
    knight_moves: [u64; 64],
    king_moves: [u64; 64],
    // Masks
    file_mask: [u64; 8],
    rank_mask: [u64; 8],
    diagonal_mask: [u64; 15],
    anti_diagonal_mask: [u64; 15],
}

impl Game {
    /// Initializes a new board with pieces.
    pub fn new() -> Game {
        let mut game = Game {
            /* initialize board, set active colour to white, ... */
            state: GameState::InProgress,
            turn: 1,

            // Inititalize bitboards. LSF bit last
            white: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111,
            black: 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
            pawn: 0b00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000,
            knight: 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            bishop: 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            rook: 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            queen: 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            king: 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            empty: 0b00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000,

            // Inititalize move tables to 0. Generated in initalize_move_tables()
            white_pawn_attacks: [0; 64],
            black_pawn_attacks: [0; 64],
            knight_moves: [0; 64],
            king_moves: [0; 64],
            // Initialize masks
            file_mask: [
                0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001, // a
                0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010, // *
                0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100, // *
                0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000, // *
                0b00010000_00010000_00010000_00010000_00100000_00010000_00010000_00010000, // *
                0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000, // *
                0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000, // *
                0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000, // h
            ],
            rank_mask: [
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111, // 1
                0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000, // *
                0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000, // *
                0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000, // *
                0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000, // *
                0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000, // *
                0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000, // *
                0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // 8
            ],
            diagonal_mask: [
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001, // a1 -> a1 (0)
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000010, // * 1
                0b00000000_00000000_00000000_00000000_00000000_00000001_00000010_00000100, // * 2
                0b00000000_00000000_00000000_00000000_00000001_00000010_00000100_00001000, // * 3
                0b00000000_00000000_00000000_00000001_00000010_00000100_00001000_00010000, // * 4
                0b00000000_00000000_00000001_00000010_00000100_00001000_00010000_00100000, // * 5
                0b00000000_00000001_00000010_00000100_00001000_00010000_00100000_01000000, // * 6
                0b00000001_00000010_00000100_00001000_00010000_00100000_01000000_10000000, // h1 -> a8  (7)
                0b00000010_00000100_00001000_00010000_00100000_01000000_10000000_00000000, // * 8
                0b00000100_00001000_00010000_00100000_01000000_10000000_00000000_00000000, // * 9
                0b00001000_00010000_00100000_01000000_10000000_00000000_00000000_00000000, // * 10
                0b00010000_00100000_01000000_10000000_00000000_00000000_00000000_00000000, // * 11
                0b00100000_01000000_10000000_00000000_00000000_00000000_00000000_00000000, // * 12
                0b01000000_10000000_00000000_00000000_00000000_00000000_00000000_00000000, // * 13
                0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000, // h8 -> h8 (14)
            ],
            anti_diagonal_mask: [
                // s/8 + 7 - s%8
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000000, // a8 -> a8 (0) s = 7
                0b00000000_00000000_00000000_00000000_00000000_00000000_10000000_01000000, // * 1
                0b00000000_00000000_00000000_00000000_00000000_10000000_01000000_00100000, // * 2
                0b00000000_00000000_00000000_00000000_10000000_01000000_00100000_00010000, // * 3
                0b00000000_00000000_00000000_10000000_01000000_00100000_00010000_00001000, // * 4
                0b00000000_00000000_10000000_01000000_00100000_00010000_00001000_00000100, // * 5
                0b00000000_10000000_01000000_00100000_00010000_00001000_00000100_00000010, // * 6
                0b10000000_01000000_00100000_00010000_00001000_00000100_00000010_00000001, //h8 -> h8 (7)
                0b01000000_00100000_00010000_00001000_00000100_00000010_00000001_00000000, // * 8
                0b00100000_00010000_00001000_00000100_00000010_00000001_00000000_00000000, // * 9 ***
                0b00010000_00001000_00000100_00000010_00000001_00000000_00000000_00000000, // * 10
                0b00001000_00000100_00000010_00000001_00000000_00000000_00000000_00000000, // * 11
                0b00000100_00000010_00000001_00000000_00000000_00000000_00000000_00000000, // * 12
                0b00000010_00000001_00000000_00000000_00000000_00000000_00000000_00000000, // * 13
                0b00000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000, //h1 -> h1 (14)
            ],
        };
        game.initialize_move_tables();
        game
    }

    /// Private helper function for moving one step in any direction.
    ///
    /// All directions are from the perspective of white.
    ///
    /// Valid directions:
    /// * "north"
    /// * "south"
    /// * "east"
    /// * "west"
    /// * "no_ea" for north east
    /// * "so_ea" for south east
    /// * "no_we" for north west
    /// * "so_we" for south west
    fn one_step(dir: &str, bit: u64) -> u64 {
        let not_a_file: u64 =
            0b11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110;
        let not_h_file: u64 =
            0b01111111_01111111_01111111_01111111_01111111_01111111_01111111_01111111;
        match dir {
            "north" => bit << 8,
            "south" => bit >> 8,
            "east" => (bit << 1) & not_a_file,
            "no_ea" => (bit << 9) & not_a_file,
            "so_ea" => (bit >> 7) & not_a_file,
            "west" => (bit >> 1) & not_h_file,
            "no_we" => (bit << 7) & not_h_file,
            "so_we" => (bit >> 9) & not_h_file,
            _ => panic!("Invalid direction"),
        }
    }

    fn horse_step(dir: &str, bit: u64) -> u64 {
        let not_a_file: u64 =
            0b11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110;
        let not_a_b_file: u64 =
            0b11111100_11111100_11111100_11111100_11111100_11111100_11111100_11111100;
        let not_h_file: u64 =
            0b01111111_01111111_01111111_01111111_01111111_01111111_01111111_01111111;
        let not_g_h_file: u64 =
            0b00111111_00111111_00111111_00111111_00111111_00111111_00111111_00111111;
        match dir {
            "no_no_ea" => (bit << 17) & not_a_file,
            "no_ea_ea" => (bit << 10) & not_a_b_file,
            "so_ea_ea" => (bit >> 6) & not_a_b_file,
            "so_so_ea" => (bit >> 15) & not_a_file,
            "no_no_we" => (bit << 15) & not_h_file,
            "no_we_we" => (bit << 6) & not_g_h_file,
            "so_we_we" => (bit >> 10) & not_g_h_file,
            "so_so_we" => (bit >> 17) & not_h_file,
            _ => panic!("Invalid direction"),
        }
    }

    /// Private helper function to initialize move-lookup tables. Called at the end of Game::new()
    fn initialize_move_tables(&mut self) {
        // White pawn attacks
        let mut iter_bit: u64 =
            0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        for i in 0..64 {
            self.white_pawn_attacks[i] =
                Game::one_step("no_ea", iter_bit) | Game::one_step("no_we", iter_bit);
            iter_bit >>= 1;
        }
        iter_bit = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        // Black pawn attacks
        for i in 0..64 {
            self.black_pawn_attacks[i] =
                Game::one_step("so_ea", iter_bit) | Game::one_step("so_we", iter_bit);
            iter_bit >>= 1;
        }
        iter_bit = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        for i in 0..64 {
            self.knight_moves[i] = Game::horse_step("no_no_we", iter_bit)
                | Game::horse_step("no_we_we", iter_bit)
                | Game::horse_step("so_we_we", iter_bit)
                | Game::horse_step("so_so_we", iter_bit)
                | Game::horse_step("so_so_ea", iter_bit)
                | Game::horse_step("so_ea_ea", iter_bit)
                | Game::horse_step("no_ea_ea", iter_bit)
                | Game::horse_step("no_no_ea", iter_bit)
                | Game::horse_step("no_no_we", iter_bit);
            iter_bit >>= 1;
        }
        iter_bit = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        for i in 0..64 {
            let row: u64 = Game::one_step("east", iter_bit) | Game::one_step("west", iter_bit);
            let king_row: u64 = iter_bit | row;
            self.king_moves[i] =
                Game::one_step("north", king_row) | Game::one_step("south", king_row) ^ king_row;
            iter_bit >>= 1;
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    /// Take the posistion and the move pattern for the piece as two u64. Color of piece that moves?
    /// move = position&target (a u64 with both the position of the piece and the target square)
    /// move^self.white (x or) (a the white piece should be removed)
    pub fn make_move(&mut self, position: u64, target: u64) -> Option<GameState> {
        let move_bit = position | target;
        if position & self.white != 0 {
            // ################
            //    WHITE PAWN
            // ################
            if position & self.pawn != 0 {
                if target & self.black != 0 {
                    //A black piece is captured
                    self.black ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.black == 0 {
                        // If the black king is removed
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.white != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.white ^= move_bit; // Moves the piece to the new square
                self.pawn ^= move_bit;
                self.empty ^= position;
            }
            // ################
            //   WHITE KNIGHT
            // ################
            if position & self.knight != 0 {
                if target & self.black != 0 {
                    //A black piece is captured
                    self.black ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.black == 0 {
                        // If the black king is removed
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.white != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.white ^= move_bit; // Moves the piece to the new square
                self.knight ^= move_bit;
                self.empty ^= position;
            }
            // ################
            //   WHITE ROOK
            // ################
            if position & self.rook != 0 {
                if target & self.black != 0 {
                    //A black piece is captured
                    self.black ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.black == 0 {
                        // If the black king is removed
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.white != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.white ^= move_bit; // Moves the piece to the new square
                self.rook ^= move_bit;
                self.empty ^= position;
            }
            // ################
            //   WHITE BISHOP
            // ################
            if position & self.bishop != 0 {
                if target & self.black != 0 {
                    //A black piece is captured
                    self.black ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.black == 0 {
                        // If the black king is removed
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.white != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.white ^= move_bit; // Moves the piece to the new square
                self.bishop ^= move_bit;
                self.empty ^= position;
            }
            // ################
            //   WHITE QUEEN
            // ################
            if position & self.queen != 0 {
                if target & self.black != 0 {
                    // A white piece is captured
                    self.black ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.white == 0 {
                        // If the black king is removed
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.white != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.white ^= move_bit; // Moves the piece to the new square
                self.queen ^= move_bit;
                self.empty ^= position;
            }
            // ################
            //    WHITE KING
            // ################
            if position & self.king != 0 {
                if target & self.black != 0 {
                    //A black piece is captured
                    self.black ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.black == 0 {
                        // If the black king is removed
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.white != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.white ^= move_bit; // Moves the piece to the new square
                self.king ^= move_bit;
                self.empty ^= position;
            }
        } else if position & self.black != 0 {
            // ################
            //    BLACK PAWN
            // ################
            if position & self.pawn != 0 {
                if target & self.white != 0 {
                    // A white piece is captured
                    self.white ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.white == 0 {
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.black != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.black ^= move_bit; // Moves the piece to the new square
                self.pawn ^= move_bit;
                self.empty ^= position;
            }
            // ################
            //   BLACK KNIGHT
            // ################
            if position & self.knight != 0 {
                if target & self.white != 0 {
                    //A black piece is captured
                    self.white ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.white == 0 {
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.black != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.black ^= move_bit; // Moves the piece to the new square
                self.knight ^= move_bit;
                self.empty ^= position;
            }
            // ################
            //    BLACK ROOK
            // ################
            if position & self.rook != 0 {
                if target & self.white != 0 {
                    // A white piece is captured
                    self.white ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.white == 0 {
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.black != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.black ^= move_bit; // Moves the piece to the new square
                self.rook ^= move_bit;
                self.empty ^= position;
            }
            // ################
            //   BLACK BISHOP
            // ################
            if position & self.bishop != 0 {
                if target & self.white != 0 {
                    // A white piece is captured
                    self.white ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.white == 0 {
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.black != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.black ^= move_bit; // Moves the piece to the new square
                self.bishop ^= move_bit;
                self.empty ^= position;
            }
            // ################
            //   BLACK QUEEN
            // ################
            if position & self.queen != 0 {
                if target & self.white != 0 {
                    // A white piece is captured
                    self.white ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.white == 0 {
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.black != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.black ^= move_bit; // Moves the piece to the new square
                self.queen ^= move_bit;
                self.empty ^= position;
            }
            // ################
            //    BLACK KING
            // ################
            if position & self.king != 0 {
                if target & self.white != 0 {
                    //A black piece is captured
                    self.white ^= target; // Removes the piece
                    let occupied = !self.empty;
                    self.pawn &= occupied;
                    self.knight &= occupied;
                    self.bishop &= occupied;
                    self.rook &= occupied;
                    self.queen &= occupied;
                    self.king &= occupied;
                    if self.king & self.white == 0 {
                        panic!("Can't capture king")
                    }
                } else if target & self.empty != 0 {
                    //An empty square
                    self.empty ^= target; // Makes the square occupied
                } else if target & self.black != 0 {
                    panic!("Attempting to capture a friendly piece")
                } else {
                    panic!("Attempting to move to non existing square!")
                }
                self.black ^= move_bit; // Moves the piece to the new square
                self.king ^= move_bit;
                self.empty ^= position;
            }
        } else {
            panic!("No piece in the given position!");
        }
        Some(self.get_game_state())
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
    pub fn get_possible_moves(&mut self, position: u64) -> Vec<u64> {
        let mut moves = Vec::new();
        let moves_pattern: u64;
        if position & self.pawn != 0 {
            //Pawn
            moves_pattern = self.get_pawn_moves(position);
        } else if position & self.knight != 0 {
            // Knight
            moves_pattern = self.get_knight_moves(position)
        } else if position & self.rook != 0 {
            // Rook
            moves_pattern = self.get_rook_moves(position);
        } else if position & self.bishop != 0 {
            // Bishop
            moves_pattern = self.get_bishop_moves(position);
        } else if position & self.queen != 0 {
            self.rook ^= position;
            self.bishop ^= position;
            moves_pattern = self.get_queen_moves(position); // Simply calls rook and bishop functions.
            self.rook ^= position;
            self.bishop ^= position;
        } else if position & self.king != 0 {
            moves_pattern = self.get_king_moves(position);
        } else {
            panic!("There is no piece in the given position!");
        }
        let mut iter_bit: u64 =
            0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        for _i in 0..64 {
            if iter_bit & moves_pattern != 0 {
                moves.push(iter_bit);
            }
            iter_bit >>= 1
        }

        moves
    }

    fn get_pawn_moves(&self, position: u64) -> u64 {
        let mut moves_bit: u64 = 0;
        if position & self.white != 0 {
            //White
            //Pawn push
            let one_step_push = Game::one_step("north", position);
            if one_step_push & self.empty != 0 {
                moves_bit |= one_step_push;
                let two_step_push = Game::one_step("north", one_step_push);
                let rank_4: u64 =
                    0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000;
                if two_step_push & self.empty & rank_4 != 0 {
                    moves_bit |= two_step_push;
                }
            }
            //Pawn capture.
            let mut iter_bit: u64 =
                0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
            for i in self.white_pawn_attacks {
                if iter_bit == position {
                    if i & self.black != 0 {
                        moves_bit |= i & self.black;
                    }
                }
                iter_bit >>= 1;
            }
        } else if position & self.black != 0 {
            //Black
            //Pawn push
            let one_step_push = Game::one_step("south", position);
            if one_step_push & self.empty != 0 {
                moves_bit |= one_step_push;
                let two_step_push = Game::one_step("south", one_step_push);
                let rank_5: u64 =
                    0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000;
                if two_step_push & self.empty & rank_5 != 0 {
                    moves_bit |= two_step_push;
                }
            }
            //Pawn capture.
            let mut iter_bit: u64 =
                0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
            for i in self.black_pawn_attacks {
                if iter_bit == position {
                    if i & self.white != 0 {
                        moves_bit |= i & self.white;
                    }
                }
                iter_bit >>= 1;
            }
        } else {
            panic!("No pawn in the given position!");
        }
        moves_bit
    }

    fn get_knight_moves(&self, position: u64) -> u64 {
        let mut moves_bit: u64 = 0;
        let mut iter_bit: u64 =
            0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        if position & self.white != 0 {
            for i in self.knight_moves {
                if iter_bit == position {
                    moves_bit |= i & !self.white;
                }
                iter_bit >>= 1;
            }
        } else if position % self.black != 0 {
            for i in self.knight_moves {
                if iter_bit == position {
                    moves_bit |= i & !self.black;
                }
                iter_bit >>= 1;
            }
        } else {
            panic!("No knight in given position!")
        }
        moves_bit
    }

    fn get_rook_moves(&self, position: u64) -> u64 {
        let moves_bit: u64;
        let occupied = !self.empty;
        let file = self.file_mask[Game::get_int_position(position) % 8];
        let rank = self.rank_mask[Game::get_int_position(position) / 8];
        if position & self.white != 0 {
            // White
            let horizontal: u64 = ((occupied - 2 * position)
                ^ (occupied.reverse_bits() - 2 * position.reverse_bits()).reverse_bits())
                & rank
                & !self.white;
            let vertical: u64 = (((occupied & file) - (2 * position))
                ^ ((occupied & file).reverse_bits() - (2 * position.reverse_bits()))
                    .reverse_bits())
                & file
                & !self.white;
            moves_bit = horizontal | vertical;
        } else if position & self.black != 0 {
            //Black
            let horizontal: u64 = ((occupied - 2 * position)
                ^ (occupied.reverse_bits() - 2 * position.reverse_bits()).reverse_bits())
                & rank
                & !self.black;
            let vertical: u64 = (((occupied & file) - (2 * position))
                ^ ((occupied & file).reverse_bits() - (2 * (position).reverse_bits()))
                    .reverse_bits())
                & file
                & !self.black;
            moves_bit = horizontal | vertical;
        } else {
            panic!("No piece in the given position!");
        }
        moves_bit
    }

    fn get_bishop_moves(&self, position: u64) -> u64 {
        let moves_bit: u64;
        let occupied = !self.empty;
        let diagonal_mask = self.diagonal_mask
            [(Game::get_int_position(position) / 8 + Game::get_int_position(position) % 8)];
        let anti_diagonal_mask = self.anti_diagonal_mask
            [(Game::get_int_position(position) / 8 + (7 - Game::get_int_position(position) % 8))];
        if position & self.white != 0 {
            //White
            let random_bullshit: u64 =
                0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
            let diagonal: u64 = (((occupied & diagonal_mask | random_bullshit) - (2 * position))
                ^ ((occupied & diagonal_mask).reverse_bits()
                    | random_bullshit - (2 * position.reverse_bits()))
                .reverse_bits())
                & diagonal_mask
                & !random_bullshit
                & !self.white;
            let anti_diagonal: u64 = (((occupied & anti_diagonal_mask | random_bullshit)
                - (2 * position))
                ^ ((occupied & anti_diagonal_mask).reverse_bits()
                    | random_bullshit - (2 * position.reverse_bits()))
                .reverse_bits())
                & anti_diagonal_mask
                & !random_bullshit
                & !self.white;
            moves_bit = diagonal | anti_diagonal;
        } else if position & self.black != 0 {
            //Black
            let random_bullshit: u64 =
                0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
            let diagonal: u64 = (((occupied & diagonal_mask | random_bullshit) - (2 * position))
                ^ ((occupied & diagonal_mask).reverse_bits()
                    | random_bullshit - (2 * position.reverse_bits()))
                .reverse_bits())
                & !random_bullshit
                & diagonal_mask
                & !self.black;
            let anti_diagonal: u64 = (((occupied & anti_diagonal_mask | random_bullshit)
                - (2 * position))
                ^ ((occupied & anti_diagonal_mask).reverse_bits()
                    | random_bullshit - (2 * position.reverse_bits()))
                .reverse_bits())
                & !random_bullshit
                & anti_diagonal_mask
                & !self.black;
            moves_bit = diagonal | anti_diagonal;
        } else {
            panic!("No piece in the given position!")
        }
        moves_bit
    }

    fn get_queen_moves(&self, position: u64) -> u64 {
        let moves_bit: u64 = self.get_rook_moves(position) | self.get_bishop_moves(position);
        moves_bit
    }
    fn get_king_moves(&self, position: u64) -> u64 {
        let mut moves_bit: u64 = 0;
        let mut iter_bit: u64 =
            0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        if position & self.white != 0 {
            for i in self.king_moves {
                if iter_bit == position {
                    moves_bit |= i & !self.white;
                }
                iter_bit >>= 1;
            }
        } else if position % self.black != 0 {
            for i in self.king_moves {
                if iter_bit == position {
                    moves_bit |= i & !self.black;
                }
                iter_bit >>= 1;
            }
        } else {
            panic!("No king in given position!")
        }
        moves_bit
    }

    fn attacks_to(&mut self, position: u64) -> u64 {
        // Place all pieces here for a super piece
        // Generate moves for super piece and check if there any piece matching the move type
        // if there are any pieces they attack the position.
        // REMEMVER TO REMOVE: super piece.
        // if the given position is a king then you're in check.
        position
    }

    /// Public function that prints the print routine for the board but with the option of including an additional bitboard with an highlight
    ///
    /// This can be used for highliting move patterns during debugging
    pub fn display_board(&self, highlight: u64) {
        // Converts the bitboards to an array to then format that as a string.
        // Initialize the array
        let mut board: [[char; 8]; 8] = [['*'; 8]; 8];
        // Get the intersection for every set of color and piece
        let white_pawn: u64 = self.white & self.pawn;
        let white_knight: u64 = self.white & self.knight;
        let white_bishop: u64 = self.white & self.bishop;
        let white_rook: u64 = self.white & self.rook;
        let white_queen: u64 = self.white & self.queen;
        let white_king: u64 = self.white & self.king;

        let black_pawn: u64 = self.black & self.pawn;
        let black_knight: u64 = self.black & self.knight;
        let black_bishop: u64 = self.black & self.bishop;
        let black_rook: u64 = self.black & self.rook;
        let black_queen: u64 = self.black & self.queen;
        let black_king: u64 = self.black & self.king;
        for i in 0..64 {
            // Shift the bit intersection i spaces and intersect it with 1.
            // This will either yield 0 if the bit in the i:th spot is 0
            // and yields 1 if the bit in the i:th spot is 1
            // If it's 1 then there's a piece there and we add a char symbolizing the specific piece
            // i/8 = rank, i%8 = file
            if ((white_pawn) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'P';
            }
            if ((white_knight) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'N';
            }
            if ((white_bishop) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'B';
            }
            if ((white_rook) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'R';
            }
            if ((white_queen) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'Q';
            }
            if ((white_king) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'K';
            }
            if ((black_pawn) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'p';
            }
            if ((black_knight) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'n';
            }
            if ((black_bishop) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'b';
            }
            if ((black_rook) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'r';
            }
            if (black_queen >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'q';
            }
            if ((black_king) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'k';
            }
            if ((highlight) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'C';
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
                board_string.push(board[7 - file][rank]);
            }
        }
        println!("{}", board_string);
    }

    /// Helper function to transform a position from &str to u64.
    ///
    /// Strings need to be provided in standard chess notation eg. "a6", "e4", or "h8"
    pub fn get_position(position: &str) -> u64 {
        match position {
            "a1" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001,
            "b1" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010,
            "c1" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000100,
            "d1" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            "e1" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            "f1" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100000,
            "g1" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000000,
            "h1" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000000,
            "a2" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000,
            "b2" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00000010_00000000,
            "c2" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00000100_00000000,
            "d2" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00001000_00000000,
            "e2" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00010000_00000000,
            "f2" => 0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_00000000,
            "g2" => 0b00000000_00000000_00000000_00000000_00000000_00000000_01000000_00000000,
            "h2" => 0b00000000_00000000_00000000_00000000_00000000_00000000_10000000_00000000,
            "a3" => 0b00000000_00000000_00000000_00000000_00000000_00000001_00000000_00000000,
            "b3" => 0b00000000_00000000_00000000_00000000_00000000_00000010_00000000_00000000,
            "c3" => 0b00000000_00000000_00000000_00000000_00000000_00000100_00000000_00000000,
            "d3" => 0b00000000_00000000_00000000_00000000_00000000_00001000_00000000_00000000,
            "e3" => 0b00000000_00000000_00000000_00000000_00000000_00010000_00000000_00000000,
            "f3" => 0b00000000_00000000_00000000_00000000_00000000_00100000_00000000_00000000,
            "g3" => 0b00000000_00000000_00000000_00000000_00000000_01000000_00000000_00000000,
            "h3" => 0b00000000_00000000_00000000_00000000_00000000_10000000_00000000_00000000,
            "a4" => 0b00000000_00000000_00000000_00000000_00000001_00000000_00000000_00000000,
            "b4" => 0b00000000_00000000_00000000_00000000_00000010_00000000_00000000_00000000,
            "c4" => 0b00000000_00000000_00000000_00000000_00000100_00000000_00000000_00000000,
            "d4" => 0b00000000_00000000_00000000_00000000_00001000_00000000_00000000_00000000,
            "e4" => 0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000,
            "f4" => 0b00000000_00000000_00000000_00000000_00100000_00000000_00000000_00000000,
            "g4" => 0b00000000_00000000_00000000_00000000_01000000_00000000_00000000_00000000,
            "h4" => 0b00000000_00000000_00000000_00000000_10000000_00000000_00000000_00000000,
            "a5" => 0b00000000_00000000_00000000_00000001_00000000_00000000_00000000_00000000,
            "b5" => 0b00000000_00000000_00000000_00000010_00000000_00000000_00000000_00000000,
            "c5" => 0b00000000_00000000_00000000_00000100_00000000_00000000_00000000_00000000,
            "d5" => 0b00000000_00000000_00000000_00001000_00000000_00000000_00000000_00000000,
            "e5" => 0b00000000_00000000_00000000_00010000_00000000_00000000_00000000_00000000,
            "f5" => 0b00000000_00000000_00000000_00100000_00000000_00000000_00000000_00000000,
            "g5" => 0b00000000_00000000_00000000_01000000_00000000_00000000_00000000_00000000,
            "h5" => 0b00000000_00000000_00000000_10000000_00000000_00000000_00000000_00000000,
            "a6" => 0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000,
            "b6" => 0b00000000_00000000_00000010_00000000_00000000_00000000_00000000_00000000,
            "c6" => 0b00000000_00000000_00000100_00000000_00000000_00000000_00000000_00000000,
            "d6" => 0b00000000_00000000_00001000_00000000_00000000_00000000_00000000_00000000,
            "e6" => 0b00000000_00000000_00010000_00000000_00000000_00000000_00000000_00000000,
            "f6" => 0b00000000_00000000_00100000_00000000_00000000_00000000_00000000_00000000,
            "g6" => 0b00000000_00000000_01000000_00000000_00000000_00000000_00000000_00000000,
            "h6" => 0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000,
            "a7" => 0b00000000_00000001_00000000_00000000_00000000_00000000_00000000_00000000,
            "b7" => 0b00000000_00000010_00000000_00000000_00000000_00000000_00000000_00000000,
            "c7" => 0b00000000_00000100_00000000_00000000_00000000_00000000_00000000_00000000,
            "d7" => 0b00000000_00001000_00000000_00000000_00000000_00000000_00000000_00000000,
            "e7" => 0b00000000_00010000_00000000_00000000_00000000_00000000_00000000_00000000,
            "f7" => 0b00000000_00100000_00000000_00000000_00000000_00000000_00000000_00000000,
            "g7" => 0b00000000_01000000_00000000_00000000_00000000_00000000_00000000_00000000,
            "h7" => 0b00000000_10000000_00000000_00000000_00000000_00000000_00000000_00000000,
            "a8" => 0b00000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            "b8" => 0b00000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            "c8" => 0b00000100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            "d8" => 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            "e8" => 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            "f8" => 0b00100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            "g8" => 0b01000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            "h8" => 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            _ => panic!("Invalid position"),
        }
    }

    /// Helper function to transform a position from u64 to int.
    fn get_int_position(position: u64) -> usize {
        match position {
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001 => 0,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010 => 1,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000100 => 2,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000 => 3,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000 => 4,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100000 => 5,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000000 => 6,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000000 => 7,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000 => 8,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000010_00000000 => 9,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000100_00000000 => 10,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00001000_00000000 => 11,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00010000_00000000 => 12,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_00000000 => 13,
            0b00000000_00000000_00000000_00000000_00000000_00000000_01000000_00000000 => 14,
            0b00000000_00000000_00000000_00000000_00000000_00000000_10000000_00000000 => 15,
            0b00000000_00000000_00000000_00000000_00000000_00000001_00000000_00000000 => 16,
            0b00000000_00000000_00000000_00000000_00000000_00000010_00000000_00000000 => 17,
            0b00000000_00000000_00000000_00000000_00000000_00000100_00000000_00000000 => 18,
            0b00000000_00000000_00000000_00000000_00000000_00001000_00000000_00000000 => 19,
            0b00000000_00000000_00000000_00000000_00000000_00010000_00000000_00000000 => 20,
            0b00000000_00000000_00000000_00000000_00000000_00100000_00000000_00000000 => 21,
            0b00000000_00000000_00000000_00000000_00000000_01000000_00000000_00000000 => 22,
            0b00000000_00000000_00000000_00000000_00000000_10000000_00000000_00000000 => 23,
            0b00000000_00000000_00000000_00000000_00000001_00000000_00000000_00000000 => 24,
            0b00000000_00000000_00000000_00000000_00000010_00000000_00000000_00000000 => 25,
            0b00000000_00000000_00000000_00000000_00000100_00000000_00000000_00000000 => 26,
            0b00000000_00000000_00000000_00000000_00001000_00000000_00000000_00000000 => 27,
            0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000 => 28,
            0b00000000_00000000_00000000_00000000_00100000_00000000_00000000_00000000 => 29,
            0b00000000_00000000_00000000_00000000_01000000_00000000_00000000_00000000 => 30,
            0b00000000_00000000_00000000_00000000_10000000_00000000_00000000_00000000 => 31,
            0b00000000_00000000_00000000_00000001_00000000_00000000_00000000_00000000 => 32,
            0b00000000_00000000_00000000_00000010_00000000_00000000_00000000_00000000 => 33,
            0b00000000_00000000_00000000_00000100_00000000_00000000_00000000_00000000 => 34,
            0b00000000_00000000_00000000_00001000_00000000_00000000_00000000_00000000 => 35,
            0b00000000_00000000_00000000_00010000_00000000_00000000_00000000_00000000 => 36,
            0b00000000_00000000_00000000_00100000_00000000_00000000_00000000_00000000 => 37,
            0b00000000_00000000_00000000_01000000_00000000_00000000_00000000_00000000 => 38,
            0b00000000_00000000_00000000_10000000_00000000_00000000_00000000_00000000 => 39,
            0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000 => 40,
            0b00000000_00000000_00000010_00000000_00000000_00000000_00000000_00000000 => 41,
            0b00000000_00000000_00000100_00000000_00000000_00000000_00000000_00000000 => 42,
            0b00000000_00000000_00001000_00000000_00000000_00000000_00000000_00000000 => 43,
            0b00000000_00000000_00010000_00000000_00000000_00000000_00000000_00000000 => 44,
            0b00000000_00000000_00100000_00000000_00000000_00000000_00000000_00000000 => 45,
            0b00000000_00000000_01000000_00000000_00000000_00000000_00000000_00000000 => 46,
            0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000 => 47,
            0b00000000_00000001_00000000_00000000_00000000_00000000_00000000_00000000 => 48,
            0b00000000_00000010_00000000_00000000_00000000_00000000_00000000_00000000 => 49,
            0b00000000_00000100_00000000_00000000_00000000_00000000_00000000_00000000 => 50,
            0b00000000_00001000_00000000_00000000_00000000_00000000_00000000_00000000 => 51,
            0b00000000_00010000_00000000_00000000_00000000_00000000_00000000_00000000 => 52,
            0b00000000_00100000_00000000_00000000_00000000_00000000_00000000_00000000 => 53,
            0b00000000_01000000_00000000_00000000_00000000_00000000_00000000_00000000 => 54,
            0b00000000_10000000_00000000_00000000_00000000_00000000_00000000_00000000 => 55,
            0b00000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000 => 56,
            0b00000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000 => 57,
            0b00000100_00000000_00000000_00000000_00000000_00000000_00000000_00000000 => 58,
            0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 => 59,
            0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 => 60,
            0b00100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 => 61,
            0b01000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 => 62,
            0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 => 63,
            _ => panic!("Invalid position"),
        }
    }
}

/// Implement print routine for Game.
/// let rook_capture: u64 = self.get_rook_moves(Game::get_position("e4"));
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
        let white_pawn: u64 = self.white & self.pawn;
        let white_knight: u64 = self.white & self.knight;
        let white_bishop: u64 = self.white & self.bishop;
        let white_rook: u64 = self.white & self.rook;
        let white_queen: u64 = self.white & self.queen;
        let white_king: u64 = self.white & self.king;

        let black_pawn: u64 = self.black & self.pawn;
        let black_knight: u64 = self.black & self.knight;
        let black_bishop: u64 = self.black & self.bishop;
        let black_rook: u64 = self.black & self.rook;
        let black_queen: u64 = self.black & self.queen;
        let black_king: u64 = self.black & self.king;
        let rook_capture: u64 = self.get_rook_moves(Game::get_position("e4"));
        for i in 0..64 {
            // Shift the bit intersection i spaces and intersect it with 1.
            // This will either yield 0 if the bit in the i:th spot is 0
            // and yields 1 if the bit in the i:th spot is 1
            // If it's 1 then there's a piece there and we add a char symbolizing the specific piece
            // i/8 = rank, i%8 = file
            if ((white_pawn) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'P';
            }
            if ((white_knight) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'N';
            }
            if ((white_bishop) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'B';
            }
            if ((white_rook) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'R';
            }
            if ((white_queen) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'Q';
            }
            if ((white_king) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'K';
            }

            if ((black_pawn) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'p';
            }
            if ((black_knight) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'n';
            }
            if ((black_bishop) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'b';
            }
            if ((black_rook) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'r';
            }
            if (black_queen >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'q';
            }
            if ((black_king) >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'k';
            }
            if (rook_capture >> i) & 1 == 1 {
                board[i / 8][i % 8] = 'C';
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
                board_string.push(board[7 - file][rank]);
            }
        }
        write!(f, "{}", board_string)
    }
}
