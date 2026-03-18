# TicTacToe-rs

## Ideas for Improvements

### Engine

1. Turn `empty_squares` in `Board` into a generator/iterator. Basically, on-demand next-move generation.
  - `pub fn legal_moves(&self) -> impl Iterator<Item = Move>`
2. Move ordering

#### Performance

1. Add tracking for number of nodes searched.
2. Use a flat board (`[Cell; 9]`) instead of (`[[Cell; 3]; 3]`)?
3. Implement alpha-beta pruning for the negamax implementation.
4. Use bitboards/masks instead of win coordinates to check for win?
5. Add `ittapi` performance measuring to the engine stuff and avoid benchmarking the UI/user input stuff.
6. Add support for this style of inputs:
```rust
Position: midgame
Nodes searched: 549946
Time: 2.1ms
Nodes/sec: 261M
```
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
repeat 10000 times
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
