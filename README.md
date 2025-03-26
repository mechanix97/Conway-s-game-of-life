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

## Usage

To run it use
```shell
make
```

and to test it use
```shell
make test
```


## Controls
- Arrows movement
- O zoom out
- I zoom in
- P pause
- C center
- R reset
- T randomize
- Left click add/remove a cell
- Right click add/remove multiple cells while pressed


## Decisions on the implementation

I did a separate thread for the simulation so i can catch the key strokes on the main loop and rapidly update the screen. Using this technique, the game looks smoother and responsive.

In order to prevent the program from quitting when the windows is closed, i had to use the macroquad fn 'prevent_quit' and periodically check if the screen was closed using 'is_quit_requested'. 

I used atomics booleans to inform changes in the game states between the simulation and the UI. This include if the game is paused or if the game is still running.
