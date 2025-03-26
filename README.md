# Self-Evolving Learning Agent

A Rust-based reinforcement learning agent that autonomously evolves from addition to multiplication, running indefinitely.

## Overview
This project implements a Q-learning agent that:
- Starts by learning addition (e.g., 3 + 5).
- Detects mastery via average reward over 50 episodes.
- Evolves to multiplication (e.g., 4 * 3) or resets to addition if maxed out.
- Runs forever, printing progress every 10 episodes.

## Running
1. Ensure Rust is installed (`cargo` available).
2. Clone this repo.
3. Run `cargo run --release`

## License
MIT License - see [LICENSE](LICENSE) for details.
