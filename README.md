# Advent of Code 2023

These are my solutions to puzzles from this year's [Advent of Code](https://adventofcode.com/).

## Usage

1. Fetch your session cookie that can be obtained by [logging in](https:// adventofcode.com/2023/auth/login) to Advent of Code.
2. Build the `get-inputs` executable that will fetch the inputs for the individual days automatically to a `./data/` directory.
    ```sh
    cargo build --release --bin get-inputs --features=inputs
    ```
3. Set the `AOC_SESSION` environment variable to that value and invoke the `get-inputs` executable to download the inputs for the days to `./data/<day>.in`:
    ```sh
    AOC_SESSION="<your-cookie-goes-here>" ./target/release/get-inputs  
    ```
4. Then run the individual executables with `cargo`. For example, `cargo run --release --bin day-05` will print the solutions for day 05 corresponding to the input in `data/05.in`.
