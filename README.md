# Project Euler Solutions
solving [project euler](https://projecteuler.net) problems in various languages

Each project folder has its own `readme.md` file with the execution time for every language measured on an r7 5800x. Each project should be written as close to the same as possible in each language to make it a fairer comparison, but this is really just for fun and mistakes are possible.

## Other info

- c++ projects:
  - compiled with `g++ main.cpp -o main.out` for debug
  - compiled with `g++ -O3 main.cpp -o main.out` for release
- rust projects:
  - compiled with `rustc main.rs` for debug
  - compiled with `rustc -O main.rs` for release
- python times will be the same on both as it does not have different modes
- execution time is measured with [hyperfine](https://github.com/sharkdp/hyperfine)