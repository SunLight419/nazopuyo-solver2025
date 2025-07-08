# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based Puyo Puyo solver using SIMD-friendly bitboard representations. The solver implements high-performance algorithms for analyzing Puyo board states and finding optimal moves.

## Architecture

### Core Components

- **`src/bitboard.rs`**: SIMD-optimized bitboard implementation for 6×13 Puyo boards
  - `BitBoard` struct: Single bitboard stored in u128 with efficient shift operations
  - `Board` struct: Complete board state with 7 bitboards (empty + 6 states) plus column height cache
  - Flood-fill algorithms for chain detection with optional AVX2 optimization
  - Gravity simulation with BMI2 CPU instruction optimization when available

- **`src/main.rs`**: Entry point demonstrating basic board manipulation

### Key Design Patterns

- **Column-major bit layout**: Bits 0-12 = column 0 (bottom to top), bits 13-25 = column 1, etc.
- **State encoding**: 0=empty, 1=garbage, 2-6=five Puyo colors
- **Height caching**: O(1) column height tracking for efficient placement
- **SIMD optimization**: Uses BMI2 and AVX2 CPU features when available

## Development Commands

### Build and Check
```bash
cargo build        # Build the project
cargo check        # Fast syntax/type checking
cargo run          # Build and run
```

### Testing
```bash
cargo test         # Run all tests
cargo test [name]  # Run specific test
```

### Code Quality
```bash
cargo clippy       # Lint checking
cargo fmt          # Code formatting
```

## Dependencies

- `colored` (2.2.0): Terminal color output for board visualization

## Performance Notes

- Board uses u128 for 78-bit representation (6×13 grid)
- Optimized flood-fill with AVX2 SIMD when target supports it
- BMI2 instruction set used for efficient gravity simulation
- Column height cache maintains O(1) placement operations