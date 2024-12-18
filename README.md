# Hash Finder task

## You can see the documentation with notes about how it works in `src/lib.rs` and tests are in `tests/tests.rs`

To start you need to download the source code and run:

`cargo run --release -- -N 3 -F 6`


- Found values are immediately printed to the console.
- For testing, results can be written to a file.
- The sequence of numbers is divided into chunks based on the number of threads on the device.
- Each chunk is processed in its respective thread.
- Output is locked for safe access.
- Atomic counting is used for tracking completion.