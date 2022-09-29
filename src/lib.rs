// vprytz-chess
// Author: Vilhelm Prytz <vilhelm@prytznet.se> or <vprytz@kth.se>

use std::fmt;

const BOARD_SIZE: usize = 8;

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

// copyed example from https://users.rust-lang.org/t/how-can-i-implement-fmt-display-for-enum/24111/2
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PieceType::King => write!(f, "K "),
            PieceType::Queen => write!(f, "Q "),
            PieceType::Rook => write!(f, "R "),
            PieceType::Bishop => write!(f, "B "),
            PieceType::Knight => write!(f, "Kn"),
            PieceType::Pawn => write!(f, "P "),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    color: Color,
    piece: PieceType,
    untouched: bool,
}

// copied base example from https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.piece)
    }
}

pub struct Game {
    state: GameState,
    board: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        let mut game = Game {
            state: GameState::InProgress,
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
        };
        // add pieces
        game.setup_initial_board();

        // return board
        game
    }

    pub fn setup_initial_board(&mut self) -> () {
        // calling this will also "reset board"
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
        self.board[1] = [black_pawn; BOARD_SIZE];

        self.board[6] = [white_pawn; BOARD_SIZE];
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
        // check if move is legal by checking if "to" position is in get_possible_moves()
        let possible_moves = self.get_possible_moves(from.to_string());

        // check that "to" is in possible_moves
        // sourcde https://stackoverflow.com/questions/58368801/how-do-i-check-if-a-thing-is-in-a-vector#58368936
        if possible_moves.unwrap().contains(&to) {
            // move piece
            let from_index = self.pos_to_index(from.to_string());
            let to_index = self.pos_to_index(to.to_string());

            let piece = self.board[from_index.0][from_index.1];

            // set piece as touched
            let mut piece = piece.unwrap();
            piece.untouched = false;

            self.board[from_index.0][from_index.1] = None;
            self.board[to_index.0][to_index.1] = Some(piece);

            // check if game is over
            // if self.is_checkmate() {
            //     self.state = GameState::GameOver;
            // } else if self.is_check() {
            //     self.state = GameState::Check;
            // }

            return Some(self.get_game_state());
        } else {
            return None;
        }
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

        let op: i32 = match piece.unwrap().color {
            Color::White => 1,
            Color::Black => -1,
        };

        // different move sets for different PieceTypes
        match piece {
            //
            // PAWN
            //
            Some(Piece {
                piece: PieceType::Pawn,
                ..
            }) => {
                let mut vec: Vec<String> = Vec::with_capacity(5);

                // add possible moves only if they are empty
                if self.board[(pos.0 as i32 - 1 * op) as usize][pos.1].is_none() {
                    vec.push(self.index_to_pos(((pos.0 as i32 - 1 * op) as usize, pos.1)));
                    // forward (up/down) one
                }

                if self.board[(pos.0 as i32 - 2 * op) as usize][pos.1].is_none()
                    && self.board[(pos.0 as i32 - 1 * op) as usize][pos.1].is_none()
                    && piece.unwrap().untouched
                {
                    vec.push(self.index_to_pos(((pos.0 as i32 - 2 * op) as usize, pos.1)));
                    // forward (up/down) two (only if first move!)
                }

                if self.board[(pos.0 as i32 + 1 * op) as usize][pos.1].is_none() {
                    vec.push(self.index_to_pos(((pos.0 as i32 + 1 * op) as usize, pos.1)));
                    // downward (down/up) one
                }

                // attack moves only if the specified positions is occupied by an enemy piece
                // we check that there is something there and that the piece there actually has a different color
                // than our piece
                if self.board[(pos.0 as i32 - 1 * op) as usize][pos.1 + 1].is_some()
                    && self.board[(pos.0 as i32 - 1 * op) as usize][pos.1 + 1]
                        .unwrap()
                        .color
                        != piece.unwrap().color
                {
                    vec.push(self.index_to_pos(((pos.0 as i32 - 1 * op) as usize, pos.1 + 1)));
                    // forward (up/down) one and right (attack right)
                }

                if self.board[(pos.0 as i32 - 1 * op) as usize][pos.1 - 1].is_some()
                    && self.board[(pos.0 as i32 - 1 * op) as usize][pos.1 - 1]
                        .unwrap()
                        .color
                        != piece.unwrap().color
                {
                    vec.push(self.index_to_pos(((pos.0 as i32 - 1 * op) as usize, pos.1 - 1)));
                    // forward (up/down) one and left (attack left)
                }

                return Some(vec);
            }
            //
            // ROOK
            //
            Some(Piece {
                piece: PieceType::Rook,
                ..
            }) => {
                let vec: Vec<String> = Vec::with_capacity(5);

                // get all possible moves in all directions

                return Some(vec);
            }
            //
            // BISHOP
            //
            Some(Piece {
                piece: PieceType::Bishop,
                ..
            }) => {
                let vec: Vec<String> = Vec::with_capacity(5);

                return Some(vec);
            }
            //
            // KNIGHT
            //
            Some(Piece {
                piece: PieceType::Knight,
                ..
            }) => {
                let mut vec: Vec<String> = Vec::with_capacity(5);

                // make list of positions to check
                let positions = [
                    // forward and left/right
                    ((pos.0 as i32 - 2 * op), (pos.1 as i32 + 1)), // two pieces "forward" and one right
                    ((pos.0 as i32 - 2 * op), (pos.1 as i32 - 1)), // two pieces "forward" and one left
                    // backwards and left/right
                    ((pos.0 as i32 + 2 * op), (pos.1 as i32 + 1)), // two pieces "backward" and one right
                    ((pos.0 as i32 + 2 * op), (pos.1 as i32 - 1)), // two pieces "backward" and one left
                    // left and up/down
                    ((pos.0 as i32 + 1 * op), (pos.1 as i32 - 2)), // two pieces "left" and one "down"
                    ((pos.0 as i32 - 1 * op), (pos.1 as i32 - 2)), // two pieces "left" and one "up"
                    // right and up/down
                    ((pos.0 as i32 + 1 * op), (pos.1 as i32 + 2)), // two pieces "right" and one "down"
                    ((pos.0 as i32 - 1 * op), (pos.1 as i32 + 2)), // two pieces "right" and one "up"
                ];

                // check for each position that it is on the board and that it is either empty or occupied by an enemy piece
                for pos in positions.iter() {
                    if pos.0 < 8
                        && pos.0 >= 0
                        && pos.1 < 8
                        && pos.1 >= 0
                        && (self.board[pos.0 as usize][pos.1 as usize].is_none()
                            || self.board[pos.0 as usize][pos.1 as usize].unwrap().color
                                != piece.unwrap().color)
                    {
                        vec.push(self.index_to_pos((pos.0 as usize, pos.1 as usize)));
                    }
                }

                return Some(vec);
            }
            //
            // QUEEN
            //
            Some(Piece {
                piece: PieceType::Queen,
                ..
            }) => {
                let vec: Vec<String> = Vec::with_capacity(5);

                return Some(vec);
            }
            //
            // KING
            //
            Some(Piece {
                piece: PieceType::King,
                ..
            }) => {
                let mut vec: Vec<String> = Vec::with_capacity(5);

                // make list of positions to check
                let positions = [
                    // one step, each direction (including diagonally)
                    ((pos.0 as i32 - 1 * op), (pos.1 as i32 + 1)), // one square "forward" and one right
                    ((pos.0 as i32 - 1 * op), (pos.1 as i32 - 1)), // one square "forward" and one left
                    ((pos.0 as i32 + 1 * op), (pos.1 as i32 + 1)), // one square "backward" and one right
                    ((pos.0 as i32 + 1 * op), (pos.1 as i32 - 1)), // one square "backward" and one left
                    ((pos.0 as i32 + 1 * op), (pos.1 as i32)),     // one square "backward"
                    ((pos.0 as i32 - 1 * op), (pos.1 as i32)),     // one square "forward"
                    ((pos.0 as i32), (pos.1 as i32 + 1)),          // one square "right"
                    ((pos.0 as i32), (pos.1 as i32 - 1)),          // one square "left"
                ];

                // TODO: check that any of the speicifed moves causes an enemy piece to be able to attack the king

                // check for each position that it is on the board and that it is either empty or occupied by an enemy piece
                for pos in positions.iter() {
                    if pos.0 < 8
                        && pos.0 >= 0
                        && pos.1 < 8
                        && pos.1 >= 0
                        && (self.board[pos.0 as usize][pos.1 as usize].is_none()
                            || self.board[pos.0 as usize][pos.1 as usize].unwrap().color
                                != piece.unwrap().color)
                    {
                        vec.push(self.index_to_pos((pos.0 as usize, pos.1 as usize)));
                    }
                }

                return Some(vec);
            }
            None => return None,
        }
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
        let mut board = String::new();

        // add top border
        board.push_str("\n|:----------------------:|");

        // iterate over board and print each piece as letter representation
        for row in self.board.iter() {
            board.push_str("\n|");
            for piece in row.iter() {
                match piece {
                    Some(p) => board.push_str(&format!(" {}", p)),
                    None => board.push_str(" * "),
                }
            }
            board.push_str("|");
        }

        // add bottom border
        board.push_str("\n|:----------------------:|");

        write!(f, "{}", board)
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
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    // test converting pos to index
    #[test]
    fn convert_pos_to_index() {
        let game = Game::new();

        assert_eq!(game.pos_to_index("a1".to_string()), (7, 0));
        assert_eq!(game.pos_to_index("B1".to_string()), (7, 1));
        assert_eq!(game.pos_to_index("A8".to_string()), (0, 0));
        assert_eq!(game.pos_to_index("H8".to_string()), (0, 7));
        assert_eq!(game.pos_to_index("H1".to_string()), (7, 7));
    }

    // test index to pos
    #[test]
    fn convert_index_to_pos() {
        let game = Game::new();

        assert_eq!(game.index_to_pos((0, 0)), "A8");
        assert_eq!(game.index_to_pos((7, 0)), "A1");
        assert_eq!(game.index_to_pos((0, 7)), "H8");
        assert_eq!(game.index_to_pos((7, 7)), "H1");
    }

    // test some pawn
    #[test]
    fn test_pawn_moves() {
        let mut game = Game::new();

        // test pawn moves
        // try white pawn
        assert_eq!(
            game.get_possible_moves("D2".to_string()).unwrap().sort(),
            vec!["D3".to_string(), "D4".to_string(),].sort()
        );
        // try black pawn
        assert_eq!(
            game.get_possible_moves("D7".to_string()).unwrap().sort(),
            vec!["D6".to_string(), "D5".to_string(),].sort()
        );

        // try white pawn at the very left
        assert_eq!(
            game.get_possible_moves("D2".to_string()).unwrap().sort(),
            vec!["D3".to_string(), "D4".to_string(),].sort()
        );

        // try moving D2 pawn to D4
        assert_eq!(
            game.make_move("D2".to_string(), "D4".to_string()),
            Some(GameState::InProgress)
        );

        println!("{:?}", game);

        // check that we have right moves for this newly moved pawn
        assert_eq!(
            game.get_possible_moves("D4".to_string()).unwrap().sort(),
            vec!["D5".to_string(), "D3".to_string(),].sort()
        );
        // then move a black pawn down, C7 to C5
        assert_eq!(
            game.make_move("C7".to_string(), "C5".to_string()),
            Some(GameState::InProgress)
        );
        println!("{:?}", game);

        // now check that the white pawn has right moves, that it can attack the black pawn
        assert_eq!(
            game.get_possible_moves("D4".to_string()).unwrap().sort(),
            vec!["D5".to_string(), "D3".to_string(), "C5".to_string()].sort()
        );
        // then attack the black pawn
        assert_eq!(
            game.make_move("D4".to_string(), "C5".to_string()),
            Some(GameState::InProgress)
        );
        println!("{:?}", game);
    }
    // test some knight moves
    #[test]
    fn test_knight_moves() {
        let mut game = Game::new();

        println!("{:?}", game);

        assert_eq!(
            game.get_possible_moves("B1".to_string()).unwrap().sort(),
            vec!["A3".to_string(), "C3".to_string(),].sort()
        );
        // move B1 to C3
        assert_eq!(
            game.make_move("B1".to_string(), "C3".to_string()),
            Some(GameState::InProgress)
        );
        assert_eq!(
            game.get_possible_moves("C3".to_string()).unwrap().sort(),
            vec![
                "B5".to_string(),
                "D5".to_string(),
                "A4".to_string(),
                "E4".to_string(),
            ]
            .sort()
        );
        println!("{:?}", game);
        // move B7 to B5
        assert_eq!(
            game.make_move("B7".to_string(), "B5".to_string()),
            Some(GameState::InProgress)
        );
        println!("{:?}", game);
        // get the pawn with the knight by moving it to B5
        assert_eq!(
            game.make_move("C3".to_string(), "B5".to_string()),
            Some(GameState::InProgress)
        );
        println!("{:?}", game);
    }

    // test some king moves
    #[test]
    fn test_king_moves() {
        use super::Color;
        use super::Piece;
        use super::PieceType;

        let mut game = Game::new();

        // assert that king cannot move
        assert_eq!(game.get_possible_moves("E1".to_string()), Some(vec![]));

        // create fake king in middle of board
        game.board[3][3] = Some(Piece {
            piece: PieceType::King,
            color: Color::White,
            untouched: true,
        });

        // assert that this newly created (fake) king can move only one square in any direction
        assert_eq!(
            game.get_possible_moves("D5".to_string()).unwrap().sort(),
            vec![
                "D6".to_string(),
                "D4".to_string(),
                "C6".to_string(),
                "C5".to_string(),
                "C4".to_string(),
                "E6".to_string(),
                "E5".to_string(),
                "E4".to_string(),
            ]
            .sort()
        );

        println!("{:?}", game);
    }
}
