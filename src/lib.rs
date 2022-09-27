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
        });
        let black_pawn = Some(Piece {
            color: Color::White,
            piece: PieceType::Pawn,
        });
        let white_rook = Some(Piece {
            color: Color::White,
            piece: PieceType::Rook,
        });
        let black_rook = Some(Piece {
            color: Color::White,
            piece: PieceType::Rook,
        });
        let white_knight = Some(Piece {
            color: Color::White,
            piece: PieceType::Knight,
        });
        let black_knight = Some(Piece {
            color: Color::White,
            piece: PieceType::Knight,
        });
        let white_bishop = Some(Piece {
            color: Color::White,
            piece: PieceType::Bishop,
        });
        let black_bishop = Some(Piece {
            color: Color::White,
            piece: PieceType::Bishop,
        });
        let white_queen = Some(Piece {
            color: Color::White,
            piece: PieceType::Queen,
        });
        let black_queen = Some(Piece {
            color: Color::White,
            piece: PieceType::Queen,
        });
        let white_king = Some(Piece {
            color: Color::White,
            piece: PieceType::King,
        });
        let black_king = Some(Piece {
            color: Color::White,
            piece: PieceType::King,
        });
        self.board[0] = [
            white_rook,
            white_knight,
            white_bishop,
            white_queen,
            white_king,
            white_bishop,
            white_knight,
            white_rook,
        ];
        self.board[1] = [white_pawn; 8];
        self.board[6] = [
            black_rook,
            black_knight,
            black_bishop,
            black_queen,
            black_king,
            black_bishop,
            black_knight,
            black_rook,
        ];
        self.board[7] = [black_pawn; 8];
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
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
        None
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
}
