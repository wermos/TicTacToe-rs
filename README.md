# TicTacToe-rs

As the GitHub short description says, this is an implementation of a Tic-Tac-Toe player (an "AI"), written in Rust.

The player can choose whether to go first or go second.

## Features

The AI uses the negamax algorithm to search through the game tree and choose the best possible move.

The program also outputs useful dev statistics like the search speed of the algorithm.

## Implementation Details

There's 4 main files in the project:
1. `main.rs` acts as the orchestrator and runs the entire game. It handles the user input and the turn order, and informing the user about the game state and result (i.e. whether the user won, the computer won, or it was a draw).
2. `board.rs` handles the representation of the board, via the `Board` struct. The `Board` struct also has useful functions that we would want to efficiently run the game, like checking whether the board is full, checking if there is a winner, setting a certain square to an X or an O.
3. `game.rs` uses the `Board` struct and handles the internal representation of the game as whole, like printing the board in a user-friendly fashion, keeping track of whose turn it is, etc.
4. `automated_player.rs` is the AI implementation. It has uses the negamax algorithm with a hand-coded evaluation function to decide the next move. The evaluation function assigns a score of `10 - depth` for a win (to prioritize fast wins over slower, drawn-out ones), `-(10 - depth)` for a loss (to prioritize slow losses over fast ones), and 0 for draws.

## Running the code

To run the code, all you need to do is
```bash
cargo run
```
and Cargo will compile and then run the code.

<details>
    <summary>Ideas for Improvements</summary>

### Engine

1. Turn `empty_squares` in `Board` into a generator/iterator. Basically, on-demand next-move generation.
  - `pub fn legal_moves(&self) -> impl Iterator<Item = Move>`
2. Move ordering

#### Performance

1. Implement alpha-beta pruning for the negamax implementation.
2. Use bitboards/masks instead of win coordinates to check for win?
3. Add `ittapi` performance measuring to the engine stuff and avoid benchmarking the UI/user input stuff.
4. ~Add support for this style of outputs:~
<strike>

```text
Position: midgame
Nodes searched: 549946
Time: 2.1ms
Nodes/sec: 261M
```

</strike>
7. Write a stress test for the engine:

```
stress_test():
    generate a set of boards

    for board in boards:
        ai.choose_move(board)
```

Examples of stress testing positions:

```
Opening:
. . .
. . .
. . .

Midgame:
X O .
. X .
. . O

Forced win:
X X .
O O .
. . .

Near endgame:
X O X
O X .
. O .
```

Or something like:

```
repeat 10000 times:
    generate random legal position
    run search
```

### UI

1. Repeated user input until valid
  - Probably requires a Move struct and also requires some way to communicate with the Board to query it for valid
    moves.
  - Conceptual structure:
    - read input
    - parse input
    - validate move
    - if valid → break
    - else → print error and repeat
2. Show numbers in only the empty cells:

```text
 X | O | X
---+---+---
 4 | O | X
---+---+---
 7 | O | 9
```

3. Better flow:

```text
You are X
Computer is O

Your move (X):

Computer plays: 5
```

3. Redraw the board in-place.

```text
clear screen
print board
prompt move
```

4. Colored input:

```text
X → blue
O → red
winning line → green
```

### Core Design

1. `GameResult` should have a `None` variant so that we don't need to use `Option<GameResult>` in `result` function.
2. Try doing something like this:

```rust
struct Players {
    x: PlayerType,
    o: PlayerType,
}

match players.get(player)
```

3. Implement a `Move` struct that encapsulates a move. (store `usize` or `(usize, usize)`?)

</details>
