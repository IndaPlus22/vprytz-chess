# vprytz-chess

Author: Vilhelm Prytz <vilhelm@prytznet.se> or <vprytz@kth.se>

This is a chess library written in Rust.
It is a work in progress and is not yet ready for use.

## Usage

## Functions

| **Function**                                                                  | **Description**                                                                                                                                                                               |
| ----------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `pub fn new() -> Game`                                                        | Initialises a new board with pieces.                                                                                                                                                          |
| `pub fn make_move(&mut self, from: String, to: String) -> Option<GameState>`  | If the current game state is `InProgress` and the move is legal, move a piece and return the resulting state of the game.                                                                     |
| **Not yet implemeted** `pub fn set_promotion(&mut self, piece: String) -> ()`                        | Set the piece type that a peasant becames following a promotion.                                                                                                                              |
| `pub fn get_game_state(&self) -> GameState`                                   | Get the current game state.                                                                                                                                                                   |
| `pub fn get_possible_moves(&self, position: String) -> Optional<Vec<String>>` | If a piece is standing on the given tile, return all possible new positions of that piece. Don't forget to the rules for check. _(optional)_ Don't forget to include en passent and castling. |

## Generate this README

You need [cargo-readme](https://github.com/livioribeiro/cargo-readme) to generate this README.

```bash
cargo readme > README.md
```
