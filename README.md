# Conway's Game of Life!

### Abstract
Conway's game of life is a basic experiment in emergent behavior, but like emergent bavhior itself it can become exceedingly complex. 
It's based upon 3 simple rules on a grid of cells that are either "dead" or "alive":
- If a cell is alive, but has less then 2 living cells in the surrounding 8, it dies on the next 'tick'
- If a cell is alive, and has 2 or 3 living cells in the surrounding 8, it survives on the next 'tick'
- If a cell is alive, and has more then 3 living cells in the surrounding 8, it dies on the next 'tick'
- If a cell is dead, but has exactly 3 living cells in the surrounding 8, it comes back to life on the next 'tick'
From these simple rules, incredible complexity can emerge from little gliders, to oscillators, to even turing machines, culminating in perhaps the peak of Conway's game of life: it is possible to run Conway's Game of Life in Conway's Game of Life

### How it was made
I used the language Rust as a backbone, implimenting not much more then the standard library and the Rustline crate for user input. The complex shapes are from the Conway's Game of Life wiki.

### How to use it
Once you've compiled for your machine with `Cargo build` or `Cargo run`, you can then run the generated executable! You will be greeted by `Timestep: ` which is how many seconds until it updates, and you can provide a float to this. Next, the x and y dimensions. I suggest having your terminal maximized and using no more then 50 by 50 as your grid although you might be able to scrape by with more. From there you get a cycling list of options:
- g: glider: the simplist glider 
- p: pulsar: a simple 3 state oscillator
- c: copperhead: a more complex glider
- t: place an individual tile
- lh/lv: 3 cell line, either horizontal or vertical 
- b: barrel: a more complex oscillator
- r: random. This will ignore the x and y location you provide to it
Once you've selected your desired number, press ctrl-d to start and ctrl-c to stop it
