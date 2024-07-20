# 🎄 Advent Of Code 2023

My solutions to AOC2023.

## 🥅 Goals
* Pure rust
* No external libraries
* Modular code
* Error handling (Where nescessary)
* Learn more efficient algorithms/optimizations
* All days combined running in under 1s

## 💻 Solutions

Each solution has it's own _self-contained_ library, where i try to make it more as a **maintainable** piece of code, **rather than just solving the issue** in one function or alike.

Links can be found in the table below:

| Day                                           | Solution                                  |
|-----------------------------------------------|-------------------------------------------|
| [01](https://adventofcode.com/2023/day/1)     | [lib/trebuchet](./lib/trebuchet/)         |
| [02](https://adventofcode.com/2023/day/2)     | [lib/cube_game](./lib/cube_game/)         |
| [03](https://adventofcode.com/2023/day/3)     | [lib/gondola_lift](./lib/gondola_lift)    |
| [04](https://adventofcode.com/2023/day/4)     | [lib/scratchcard](./lib/scratchcard/)     |
| [05](https://adventofcode.com/2023/day/5)     | [lib/almanac](./lib/almanac/)             |
| [06](https://adventofcode.com/2023/day/6)     | [lib/boat_race](./lib/boat_race/)         |
| [07](https://adventofcode.com/2023/day/7)     | [lib/camel_cards](./lib/camel_cards/)     |
| [08](https://adventofcode.com/2023/day/8)     | [lib/network_nodes](./lib/network_nodes/) |
| [09](https://adventofcode.com/2023/day/9)     | [lib/oasis](./lib/oasis/)                 |
| [10](https://adventofcode.com/2023/day/10)    | [lib/pipe_maze](./lib/pipe_maze/)         |
| [11](https://adventofcode.com/2023/day/11)    | 🕙TBD |
| [12](https://adventofcode.com/2023/day/12)    | 🕙TBD |
| [13](https://adventofcode.com/2023/day/13)    | 🕙TBD |
| [14](https://adventofcode.com/2023/day/14)    | 🕙TBD |
| [15](https://adventofcode.com/2023/day/15)    | 🕙TBD |
| [16](https://adventofcode.com/2023/day/16)    | 🕙TBD |
| [17](https://adventofcode.com/2023/day/17)    | 🕙TBD |
| [18](https://adventofcode.com/2023/day/18)    | 🕙TBD |
| [19](https://adventofcode.com/2023/day/19)    | 🕙TBD |
| [20](https://adventofcode.com/2023/day/20)    | 🕙TBD |
| [21](https://adventofcode.com/2023/day/21)    | 🕙TBD |
| [22](https://adventofcode.com/2023/day/22)    | 🕙TBD |
| [23](https://adventofcode.com/2023/day/23)    | 🕙TBD |
| [24](https://adventofcode.com/2023/day/24)    | 🕙TBD |
| [25](https://adventofcode.com/2023/day/25)    | 🕙TBD |

## ☠️ Fails
* **Day 5 (Part 2)** - Completed without any external help. I however had to resort to getting a hint on what algorithm to use in order to optimize, as my initial solution ran for +10 minutes. A small win on implementing an algorithm i didn't know before (Ford–Fulkerson).

* **Day 8 (Part 2)** - Completed but never finished *completely* on my own as my initial solution ran for hours without ever ending (Which i knew would happen..). I had to get a hint on how to optimize this, which lead to implementing LCM.

* **Day 10 (Part 2)** - After 12 hours of hacking away, i couldn't get a solution running that worked. Got a hint about implementing Picks Theorem with the Shoelace algorithm, which i tried (And apparently succeeded), but resorted to posting on reddit as it didn't work. It was however, another bug/mistake that caused it not to work - Not the algorithm itself nessecarily.