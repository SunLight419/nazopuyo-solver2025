# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based Puyo Puyo solver (謎ぷよソルバー) using bitboard representations for high-performance board state analysis. The project focuses on solving Puyo puzzle challenges by exploring possible moves through efficient board manipulation algorithms.

## Architecture

### Workspace Structure

The project uses a Rust workspace with two main components:
- **Root binary** (`src/main.rs`): CLI application demonstrating board functionality
- **Core library** (`core/`): Reusable Puyo logic designed for future GUI integration (Tauri planned)

### Core Components

#### `core/src/types.rs`
- **`PuyoColor`** enum: 7 puyo types (Empty=0, Garbage=1, Red=2, Blue=3, Green=4, Yellow=5, Purple=6)
- **`Position`** struct: Board coordinates with validation (6×13 grid)
- **`ChainInfo`** struct: Chain execution tracking

#### `core/src/traits.rs`
- **`PuyoBoard`** trait: Core board operations (place_puyo, get_puyo, apply_gravity, execute_chains, display)
- **`PuyoState`** trait: State management for search algorithms (clone_state, hash_state, is_equivalent)
- **`PuyoPair`** trait: Puyo pair handling for game moves
- **`PuyoPlacement`** trait: Advanced placement operations with rotation support

#### `core/src/bitboard.rs`
- **`SimpleBitBoardPuyoBoard`**: Bitboard implementation using column-major layout
  - Each column stored as u64 (3 bits per puyo × 13 rows = 39 bits max)
  - Optimized column height tracking using leading_zeros()
  - Flood-fill algorithm for connected group detection
  - Gravity simulation and chain execution

### Key Design Patterns

- **Column-major bit layout**: Each column stored in u64, 3 bits per puyo position
- **Trait-based abstraction**: Allows multiple board implementations for performance comparison
- **Bitwise operations**: Efficient puyo manipulation using bit shifting and masking
- **Height caching**: O(1) column height calculation using leading_zeros()

## Game Rules Implementation

- **Board size**: 6 columns × 13 rows
- **Chain detection**: 4+ connected same-color puyos disappear
- **Game over condition**: Puyo placed at position (2, 11) - left from 3rd column, 2nd from top
- **Top row exclusion**: Puyos in row 12 don't count for chain connections
- **Gravity**: Automatic puyo falling after placements and chain executions

## Development Commands

### Build and Run
```bash
cargo build                    # Build entire workspace
cargo run                      # Run main demo application
cargo build -p core           # Build only core library
cargo run -p nazopuyo-solver2025  # Run specific binary
```

### Testing
```bash
cargo test                     # Run all tests (workspace + core)
cargo test -p core            # Run only core library tests
cargo test test_name          # Run specific test
cargo test -- --nocapture    # Show println! output in tests
```

### Code Quality
```bash
cargo clippy                  # Lint checking
cargo clippy -p core         # Lint only core library
cargo fmt                    # Code formatting
cargo check                  # Fast syntax/type checking
```

## Dependencies

- **`colored` (2.2.0)**: Terminal color output for board visualization
  - Used in `SimpleBitBoardPuyoBoard::display()` method
  - Colors: Red(red), Blue(blue), Green(green), Yellow(yellow), Purple(magenta), Garbage(bright_black)

## Performance Notes

- **Bitboard representation**: Each column stored in u64 with 3-bit puyo encoding
- **Column height optimization**: Uses `leading_zeros()` for O(1) height calculation
- **Flood-fill implementation**: Efficient connected group detection for chain calculation
- **Memory efficiency**: Entire 6×13 board state fits in 6×u64 = 384 bits

## Future Development Considerations

- GUI support planned using Tauri framework
- Performance optimization opportunities:
  - SIMD instructions for parallel column operations
  - Hash tables for state memoization in search algorithms
  - Parallel processing for move tree exploration
- Additional bitboard implementations for performance comparison