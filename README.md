# game-of-life
A game of life🔬 simulator on an infinite♾️ plane

<p float="right">
  <img src="./assets/random.gif" width="65%"/>
  <img src="./assets/halfmax.gif" width="33.5%"/>
</p>

***NOTE: This is a toy project! I did this just for fun, not as a packaged product.***

## About the Author

I'm a *bored* sophomore in college working on projects to fill the time. If you enjoy my work, consider supporting me by buying me a coffee! 

<a href="https://www.buymeacoffee.com/adam.mcdaniel" target="_blank">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-blue.png" alt="Buy Me A Coffee" height="60px" width="217px"/>
</a>

## What possessed me to write yet another Game of Life🦠 simulator?

<div align="left">
<img align='right' src="assets/glider-gun.gif" width="30%"/>
Everyone who has learned to program at this point <a href="https://github.com/search?q=game+of+life&type=code">has probably written a Game of Life simulator</a>; it's a bit of a rite of passage, like a more sophisticated "Hello World!" program. For those of you who haven't seen the Game of Life yet, it's a <a href="https://en.wikipedia.org/wiki/Cellular_automaton">cellular automata</a> performed on a 2D plane with the following rules:

<h4>Rules</h4>
<ol>
<li>Any live cell with fewer than two live neighbours dies, as if by starvation.</li>
<li>Any live cell with two or three live neighbours lives on to the next generation.</li>
<li>Any live cell with more than three live neighbours dies, as if by overpopulation.</li>
<li>Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.</li>
</ol>

When making a Game of Life simulator, though, most people just get the minimal, barebones simulation working on a glider or a few other patterns before finishing up the project, and more importantly, we only ever learn how to implement the Game on a <i>statically sized grid</i>. How do the more complicated simulators on the internet support <i><b>theoretically infinitely sized worlds🧫???</i></b>

Thinking about this got me curious, so I took a stab at it myself. I think I did an okay job!
</div>

## How does it work?

The key to the solution is expressed in the wording of the problem itself: to make an implementation agnostic of infinitely sized worlds, the implementation *must not depend on the world's size*.

<img align='left' src="assets/unbounded.gif" width="30%"/>

So what does that mean in practice? Well, it means that any implementation using statically allocated arrays for the world is already out of the running. But what about dynamically sized arrays? Although, yes, in theory these are possible, they would be **incredibly** inefficient. Every time a glider moved beyond a corner of the world, the program would have to allocate a new row, *and* a new column: which would compound as the glider keeps going! Your program would ***very quickly*** run out of memory, unless it could deallocate parts of the grid it wasn't using, which would take quite a bit of time on its own.

For some, it might be painfully obvious that the solution is to *only* store the live cells mapped from their position. So for a glider traversing the entire world for an infinitely long, your program only every stores data for ***5 cells at once!*** This has the added benefit that this makes it very simple to only ever consider cells which are already neighbors of live cells.


The entire algorithm works as follows for each iteration:
1. For every live cell, if it is overcrowded *(>3 neighbors)* or starving *(<2 neighbors)*, remove it from the next iteration.
2. Additionally, for every neighboring position of that cell *(regardless of whether it was removed)*:
3. If the position has already been considered, skip the following steps. Otherwise, add the position to the set of previously considered positions.
4. If the position is overcrowded or starving, remove the cell at that position from the next iteration *(if it exists)*.
5. If the position has ***exactly three live neighbors, however***, insert a cell at that position in the next iteration.
6. Voila! You have the next iteration of the world!

To see it in action, run some of the [examples](https://github.com/adam-mcdaniel/game-of-life/tree/main/examples)!

## Usage

To run, you must download Rust from [here](https://www.rust-lang.org/).

```bash
# Clone the repo and run from source
git clone https://github.com/adam-mcdaniel/game-of-life
cd game-of-life
cargo run --example random --release
```
