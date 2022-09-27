// vprytz-chess
// Author: Vilhelm Prytz <vilhelm@prytznet.se> or <vprytz@kth.se>

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    color: Color,
    piece: PieceType,
    untouched: bool,
}

pub struct Game {
    state: GameState,
    board: [[Option<Piece>; 8]; 8],
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            state: GameState::InProgress,
            board: [[None; 8]; 8],
        }
    }

    pub fn setup_initial_board(&mut self) -> () {
        let white_pawn = Some(Piece {
            color: Color::White,
            piece: PieceType::Pawn,
            untouched: true,
        });
        let black_pawn = Some(Piece {
            color: Color::Black,
            piece: PieceType::Pawn,
            untouched: true,
        });
        let white_rook = Some(Piece {
            color: Color::White,
            piece: PieceType::Rook,
            untouched: true,
        });
        let black_rook = Some(Piece {
            color: Color::Black,
            piece: PieceType::Rook,
            untouched: true,
        });
        let white_knight = Some(Piece {
            color: Color::White,
            piece: PieceType::Knight,
            untouched: true,
        });
        let black_knight = Some(Piece {
            color: Color::Black,
            piece: PieceType::Knight,
            untouched: true,
        });
        let white_bishop = Some(Piece {
            color: Color::White,
            piece: PieceType::Bishop,
            untouched: true,
        });
        let black_bishop = Some(Piece {
            color: Color::Black,
            piece: PieceType::Bishop,
            untouched: true,
        });
        let white_queen = Some(Piece {
            color: Color::White,
            piece: PieceType::Queen,
            untouched: true,
        });
        let black_queen = Some(Piece {
            color: Color::Black,
            piece: PieceType::Queen,
            untouched: true,
        });
        let white_king = Some(Piece {
            color: Color::White,
            piece: PieceType::King,
            untouched: true,
        });
        let black_king = Some(Piece {
            color: Color::Black,
            piece: PieceType::King,
            untouched: true,
        });
        self.board[0] = [
            black_rook,
            black_knight,
            black_bishop,
            black_queen,
            black_king,
            black_bishop,
            black_knight,
            black_rook,
        ];
        self.board[1] = [black_pawn; 8];

        self.board[6] = [white_pawn; 8];
        self.board[7] = [
            white_rook,
            white_knight,
            white_bishop,
            white_queen,
            white_king,
            white_bishop,
            white_knight,
            white_rook,
        ];
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        let from = self.pos_to_index(from);
        let to = self.pos_to_index(to);

        let mut vec: Vec<String> = Vec::with_capacity(60);

        return Some(self.get_game_state());
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, piece: String) -> () {
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
    pub fn get_possible_moves(&self, postion: String) -> Option<Vec<String>> {
        let pos = self.pos_to_index(postion);

        // get piece at given position
        let piece = self.board[pos.0][pos.1];

        // print piece
        println!("{:?}", piece);

        // different move sets for different PieceTypes
        match piece {
            Some(Piece {
                piece: PieceType::Pawn,
                ..
            }) => {
                let mut vec: Vec<String> = Vec::with_capacity(5);
                let op: i32 = match piece.unwrap().color {
                    Color::White => 1,
                    Color::Black => -1,
                };

                vec.push(self.index_to_pos(((pos.0 as i32 - 1 * op) as usize, pos.1))); // forward (up/down) one
                vec.push(self.index_to_pos(((pos.0 as i32 - 2 * op) as usize, pos.1))); // forward (up/down) two (only if first move!)
                vec.push(self.index_to_pos(((pos.0 as i32 + 1 * op) as usize, pos.1))); // downward (down/up) one
                vec.push(self.index_to_pos(((pos.0 as i32 - 1 * op) as usize, pos.1 - 1))); // attack left
                vec.push(self.index_to_pos(((pos.0 as i32 - 1 * op) as usize, pos.1 + 1))); // attack left

                return Some(vec);
            }
            None => return None,
            _ => todo!("not yet implemented"),
        }

        None
    }

    // convert two letter position to index in 2d array
    fn pos_to_index(&self, pos: String) -> (usize, usize) {
        // convert pos to lowercase non-borrowed string and then chars
        let pos = pos.to_lowercase();
        let mut chars = pos.chars();

        // when using "as usize", A will be 97, B will be 98 and so on ...
        // meaning if we subtract 97 we will get the correct index
        let y = chars.next().unwrap() as usize - 97;

        // same here, but instead of subtracting 97 we subtract 49
        // since 49 is the ascii value of 1
        // our array increases index from top to bottom, so we need to
        // include "7 -" since chess uses increasing from bottom to top
        let x = 7 - (chars.next().unwrap() as usize - 49);

        (x, y)
    }

    // convert index in 2d array to two letter position
    fn index_to_pos(&self, index: (usize, usize)) -> String {
        // basically the reverse of pos_to_index
        // we convert the index to "ascii value" and then to char
        // we use as u8 since usize cannot be converted to char directly
        let y = (index.1 + 97) as u8 as char;
        let x = (7 - index.0 + 49) as u8 as char;
        format!("{}{}", y.to_uppercase(), x)
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        write!(f, "")
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
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let mut game = Game::new();
        game.setup_initial_board();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    // test converting pos to index
    #[test]
    fn convert_pos_to_index() {
        let mut game = Game::new();
        game.setup_initial_board();

        assert_eq!(game.pos_to_index("a1".to_string()), (7, 0));
        assert_eq!(game.pos_to_index("B1".to_string()), (7, 1));
        assert_eq!(game.pos_to_index("A8".to_string()), (0, 0));
        assert_eq!(game.pos_to_index("H8".to_string()), (0, 7));
        assert_eq!(game.pos_to_index("H1".to_string()), (7, 7));
    }

    // test index to pos
    #[test]
    fn convert_index_to_pos() {
        let mut game = Game::new();
        game.setup_initial_board();

        assert_eq!(game.index_to_pos((0, 0)), "A8");
        assert_eq!(game.index_to_pos((7, 0)), "A1");
        assert_eq!(game.index_to_pos((0, 7)), "H8");
        assert_eq!(game.index_to_pos((7, 7)), "H1");
    }

    // test some moves
    #[test]
    fn test_piece_moves() {
        let mut game = Game::new();
        game.setup_initial_board();

        // test pawn moves
        // try white pawn
        assert_eq!(
            game.get_possible_moves("D2".to_string()),
            Some(vec![
                "D3".to_string(),
                "D4".to_string(),
                "D1".to_string(),
                "C3".to_string(),
                "E3".to_string(),
            ])
        );
        // try black pawn
        assert_eq!(
            game.get_possible_moves("D7".to_string()),
            Some(vec![
                "D6".to_string(),
                "D5".to_string(),
                "D8".to_string(),
                "C6".to_string(),
                "E6".to_string(),
            ])
        );
    }
}
