# conway_rs

Conway's Game of Life in Rust!

## Use

Requires [Rust](https://www.rust-lang.org/).

Build with:

    cargo build --release

Then to do one iteration:

    echo "000\n111\n000" | ./target/release/conway_rs

which should output:

    010
    010
    010

To do 2 iterations:

    echo "000\n111\n000" | ./target/release/conway_rs 2

which should output:

    000
    111
    000
