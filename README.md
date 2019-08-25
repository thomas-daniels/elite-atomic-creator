# Elite Atomic creator

A tool that automatically creates the next Elite Atomic tournament on Lichess.

Elite Atomics run each Sunday at 19:00 UTC.

Please note that **you do not need to run this tool yourself**, because if you do,
it will only create unnecessary colliding Elite Atomic arenas. This tool is open-source to serve as an
example of sending a Lichess API request through Rust with the `reqwest` crate.

Tested on Rust stable and Rust nightly (as of August 25, 2019).