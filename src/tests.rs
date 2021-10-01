#[cfg(test)]
mod tests {
    use crate::Game;
    use crate::GameState;

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
        //println!("{:?}", game);
        assert_eq!(game.get_game_state(), GameState::InProgress);
        println!("ok");

        print!("2... ");
        game.initialize_move_tables();
        assert_eq!(game.white_pawn_attacks[0], 0);
        println!("ok");
        print!("3... ");
        assert_eq!(
            game.white_pawn_attacks[9],
            0b10100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
        );
        println!("ok");
        print!("4... ");
        assert_eq!(
            game.knight_moves[0],
            0b00000000_00100000_01000000_00000000_00000000_00000000_00000000_00000000
        );
        println!("ok");
    }

    #[test]
    fn shift_test() {
        println!();
        print!("1... ");
        let bit: u64 = 0b00000000_00000000_00000000_10000000_00000000_00000000_00000000_00000000;
        assert_eq!(
            Game::one_step("north", bit),
            0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000
        );
        println!("ok");
        print!("2... ");
        let bit: u64 = 0b00000000_00000000_00000000_10000000_00000000_00000000_00000000_00000000;
        assert_eq!(
            Game::one_step("south", bit),
            0b00000000_00000000_00000000_00000000_10000000_00000000_00000000_00000000
        );
        println!("ok");
        print!("3... ");
        let bit: u64 = 0b00000000_00000000_00000000_10000000_00000000_00000000_00000000_00000000;
        assert_eq!(
            Game::one_step("east", bit),
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
        );
        println!("ok");
        print!("4... ");
        let bit: u64 = 0b00000000_00000000_00000000_01000000_00000000_00000000_00000000_00000000;
        assert_eq!(
            Game::one_step("west", bit),
            0b00000000_00000000_00000000_00100000_00000000_00000000_00000000_00000000
        );
        println!("ok");
        print!("5... ");
        let bit: u64 = 0b00000000_00000000_00000000_01000000_00000000_00000000_00000000_00000000;
        assert_eq!(
            Game::one_step("no_ea", bit),
            0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000
        );
        println!("ok");
        print!("6... ");
        let bit: u64 = 0b00000000_00000000_00100000_00000000_00000000_00000000_00000000_00000000;
        assert_eq!(
            Game::one_step("no_we", bit),
            0b00000000_00010000_00000000_00000000_00000000_00000000_00000000_00000000
        );
        println!("ok");
        print!("7... ");
        let bit: u64 = 0b00000000_00000000_01000000_00000000_00000000_00000000_00000000_00000000;
        assert_eq!(
            Game::one_step("so_ea", bit),
            0b00000000_00000000_00000000_10000000_00000000_00000000_00000000_00000000
        );
        println!("ok");
        print!("8... ");
        let bit: u64 = 0b00000000_00000000_00000000_00000000_00100000_00000000_00000000_00000000;
        assert_eq!(
            Game::one_step("so_we", bit),
            0b00000000_00000000_00000000_00000000_00000000_00010000_00000000_00000000
        );
        println!("ok");
    }

    #[test]
    fn a_story_in_3_moves() {
        println!();
        let mut game = Game::new();
        game.make_move(Game::get_position("h8"), Game::get_position("e4"));
        game.display_board(game.get_rook_moves(Game::get_position("e4")));
        game.make_move(Game::get_position("c2"), Game::get_position("c4"));
        game.display_board(game.get_rook_moves(Game::get_position("e4")));
        game.make_move(Game::get_position("e4"), Game::get_position("c4"));
        game.display_board(game.get_rook_moves(Game::get_position("c4")));
        let n: u64 = 0b01101001_00000100_00000000_00000000_00000000_00000000_00000000_00000000;
        let m: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_10010110;
        assert_eq!(n, m.reverse_bits());
    }
    #[test]
    fn bishop_test() {
        println!();
        let mut game = Game::new();
        let mut iter_bit: u64 =
            0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000;
        game.make_move(Game::get_position("c8"), iter_bit);
        game.display_board(game.get_bishop_moves(iter_bit));
        for _i in 0..32 {
            game.make_move(iter_bit, iter_bit >> 1);
            game.display_board(game.get_bishop_moves(iter_bit >> 1));
            iter_bit >>= 1;
        }
        let n: u64 = 0b01101001_00000100_00000000_00000000_00000000_00000000_00000000_00000000;
        let m: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_10010110;
        assert_eq!(n, m.reverse_bits());
    }
    #[test]
    fn rook_test() {
        println!();
        let mut game = Game::new();
        let mut iter_bit: u64 =
            0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000;
        game.make_move(Game::get_position("a8"), iter_bit);
        game.display_board(game.get_rook_moves(iter_bit));
        for _i in 0..32 {
            game.make_move(iter_bit, iter_bit >> 1);
            game.display_board(game.get_rook_moves(iter_bit >> 1));
            iter_bit >>= 1;
        }
        let n: u64 = 0b01101001_00000100_00000000_00000000_00000000_00000000_00000000_00000000;
        let m: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_10010110;
        assert_eq!(n, m.reverse_bits());
    }
    #[test]
    fn queen_test() {
        //Further testing required. Seems like bishop works fine for white's side.
        println!();
        let mut game = Game::new();
        let mut iter_bit: u64 =
            0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000;
        game.make_move(Game::get_position("d1"), iter_bit);
        game.display_board(game.get_queen_moves(iter_bit));
        for _i in 0..31 {
            game.make_move(iter_bit, iter_bit >> 1);
            game.display_board(game.get_queen_moves(iter_bit >> 1));
            iter_bit >>= 1;
        }
        let n: u64 = 0b01101001_00000100_00000000_00000000_00000000_00000000_00000000_00000000;
        let m: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_10010110;
        assert_eq!(n, m.reverse_bits());
    }
    #[test]
    fn knight_test() {
        println!();
        let mut game = Game::new();

        let mut iter_bit: u64 =
            0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000;
        game.make_move(Game::get_position("b1"), iter_bit);
        game.display_board(game.get_knight_moves(iter_bit));
        for _i in 0..31 {
            game.make_move(iter_bit, iter_bit >> 1);
            game.display_board(game.get_knight_moves(iter_bit >> 1));
            iter_bit >>= 1;
        }
        let n: u64 = 0b01101001_00000100_00000000_00000000_00000000_00000000_00000000_00000000;
        let m: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_10010110;
        assert_eq!(n, m.reverse_bits());
    }
    #[test]
    fn king_test() {
        println!();
        let mut game = Game::new();

        let mut iter_bit: u64 =
            0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000;
        game.make_move(Game::get_position("e1"), iter_bit);
        game.display_board(game.get_king_moves(iter_bit));
        for _i in 0..31 {
            game.make_move(iter_bit, iter_bit >> 1);
            game.display_board(game.get_king_moves(iter_bit >> 1));
            iter_bit >>= 1;
        }
        let n: u64 = 0b01101001_00000100_00000000_00000000_00000000_00000000_00000000_00000000;
        let m: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_10010110;
        assert_eq!(n, m.reverse_bits());
    }
}
