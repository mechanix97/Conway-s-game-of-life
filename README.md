# Conway's game of life

My implementation of Conway's game of life.

![Conway's game of life](conways.gif)


## Rules

- Any live cell with fewer than two live neighbours dies, as if by underpopulation.
- Any live cell with two or three live neighbours lives on to the next generation.
- Any live cell with more than three live neighbours dies, as if by overpopulation.
- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

(Rules extracted from [Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life))

## Dependencies

It's built using Rust with the crate macroquad and uses Make to run

## Controls
- Arrows movement
- \- zoom out
- = zoom in
- P pause
- C center
- R reset
- T randomize
- Left click add/remove a cell
- Right click add/remove multiple cells while pressed
