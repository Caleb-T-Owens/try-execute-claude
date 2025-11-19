# Try claude executable

A small script to try and get more debugging information around why
`std::process::Command` might fail to execute CC in certain setups.

Usage:

```bash
cargo run -- <path-to-CC-executable>
# Or with extra debug info
RUST_BACKTRACE=1 cargo run -- <path-to-CC-executable>
# Example with the claude.sh in the project
RUST_BACKTRACE=1 cargo run -- ./claude.sh
```
