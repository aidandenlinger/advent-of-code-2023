# aidandenlinger's Advent of Code 2023

Doing Advent of Code 2023 in Rust for practice and fun :)

Solutions prioritize legibility over optimality, although both are limited to
the extent of my knowledge ;). You can find Advent of Code 2023
[here](https://adventofcode.com/2023).

## Install
You need the rust toolchain, which you can install with
[rustup](https://rustup.rs/).

## Usage
Each day has two folders - one for part a and one for part b.

To run, open that day and part's folder and run `cargo run`. These
solutions assume proper input, it will panic on improper inputs. You
can change the input in the `input.txt` file in the problem's folder.

`cargo test` will run basic tests on the examples from the problem prompt, as
well as test that it produces the expected answer from my input. If you change
the inputs, that unit test will now fail since the input is different, this
would be expected.

## Thanks
Thanks to other folks publishing their Rust repos so I can learn and improve
my code afterwards :)

- [timvisee](https://github.com/timvisee/advent-of-code-2023)
- [fspoettel](https://github.com/fspoettel/advent-of-code-2023/tree/main/src/bin)
- [AxlLind](https://github.com/AxlLind/AdventOfCode2023/tree/main/src/bin)
- [believer](https://github.com/believer/advent-of-code/tree/master/rust/2023/src)
- [maneatingape](https://github.com/maneatingape/advent-of-code-rust/tree/main/src/year2023)
- [LowLevelLearning (Youtube, mostly not Rust but fun)](https://youtube.com/playlist?list=PLc7W4b0WHTAWQBQkx8oGrFuiCuQFbhecq)

## Contributing
Not accepting contributions, these are puzzles :)

## License
MIT
