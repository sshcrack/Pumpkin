# Pumpkin Repository Custom Instructions

## Repository Overview

**Pumpkin** is a high-performance Minecraft server implementation written entirely in Rust. The project prioritizes performance through multi-threading, compatibility with the latest Java and Bedrock editions, security, and extensibility for plugin development.

**Key Statistics:**
- Language: Rust (stable toolchain)
- Edition: 2024
- Build System: Cargo (workspace with 10 member crates)
- Target: Minecraft 1.21.11
- CI: GitHub Actions (tests run on Linux, Windows, and macOS)

## Project Structure

The workspace consists of the following crates:

- **pumpkin**: Main server binary and core logic
- **pumpkin-protocol**: Minecraft protocol implementation (packets, serialization)
- **pumpkin-world**: World generation, chunk management, and world I/O
- **pumpkin-registry**: Minecraft registries (biomes, blocks, items, entities, effects, etc.)
- **pumpkin-inventory**: Inventory and container management
- **pumpkin-config**: Configuration file parsing (TOML format)
- **pumpkin-util**: Utility functions and helpers
- **pumpkin-macros**: Procedural macros for code generation
- **pumpkin-api-macros**: API-related macro definitions
- **pumpkin-data**: Data file generation and registry data

**Key Directories:**
- `/assets/`: Game data files (JSON registries, NBT chunk samples, translations)
- `/config/`: Configuration files for server
- `/plugins/`: Plugin directory
- `/.github/`: GitHub workflows and issue templates

## Build and Test Instructions

### Prerequisites
- Rust stable toolchain (installed via `rustup`)
- `rustfmt` and `clippy` components

### Build Commands

**Development Build:**
```bash
cargo build
```

**Release Build (optimized with LTO):**
```bash
cargo build --release
```
- Release builds use full LTO and single codegen unit for maximum optimization
- Binaries are placed in `target/release/pumpkin`

### Testing

**Run All Tests:**
```bash
cargo test --verbose
```

**Run Tests for Specific Crate:**
```bash
cargo test -p pumpkin_world --verbose
```

### Code Quality Checks

**Format Check (must pass before PR):**
```bash
cargo fmt --check
```

**Format Code:**
```bash
cargo fmt
```

**Clippy Linting (must have zero warnings):**
```bash
cargo clippy --all-targets --all-features
```

**Check for Typos:**
```bash
typos
```

### Important Notes on Build

1. **Clippy is Strict**: Pumpkin uses strict Clippy settings. All clippy warnings must be resolved before merging.
2. **RUSTFLAGS**: The environment variable `RUSTFLAGS="-Dwarnings"` is set in CI, which treats all warnings as errors.
3. **Build Time**: Release builds can take several minutes due to LTO optimization.
4. **Workspace**: Always run builds and tests from the workspace root where `Cargo.toml` is located.

## Code Quality Standards

### Required for Pull Requests

1. **No Clippy Warnings**: Run `cargo clippy --all-targets --all-features` and fix all issues
2. **Passing Tests**: All tests must pass with `cargo test --verbose`
3. **Code Formatting**: Code must be formatted with `cargo fmt`
4. **Clear Commit Messages**: Use descriptive, concise messages
5. **PR Description**: Include what changed, why, and any impact

### Best Practices

1. **Unit Tests**: Add tests for new features (see Rust Book: https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
2. **Documentation**: Update relevant docs for new functionality
3. **Benchmarking**: For performance-critical changes, add Criterion benchmarks
4. **Tokio/Rayon Usage**:
   - Use Rayon (`rayon::spawn`, parallel iterators) for CPU-intensive tasks
   - **Do not block** the Tokio runtime on Rayon calls
   - Use `tokio::sync::mpsc` to transfer data between runtimes
   - Example: `pumpkin_world::level::Level::fetch_chunks`

## Architecture Guidelines

### Protocol Layer (pumpkin-protocol)
- Handles Minecraft protocol packets (encoding/decoding)
- Manages encryption and compression
- Do not include business logic here

### World Layer (pumpkin-world)
- Chunk generation, loading, and saving
- Block and entity state management
- Dimension/biome handling
- Uses Rayon for parallel chunk operations

### Registry Layer (pumpkin-registry)
- Minecraft registries (blocks, items, entities, biomes, effects, etc.)
- Data is loaded from JSON files in `/assets/`
- **Do not hardcode** registry data; use the registry system

### Core Server (pumpkin)
- Main server loop and initialization
- Player connection management
- Plugin system
- Configuration and RCON/Query servers

## Configuration

- Server configuration: `/config/configuration.toml`
- Feature flags: `/config/features.toml`
- All configuration should use TOML format and be parsed via `pumpkin-config`

## Common Tasks

### Adding a New Registry Entry
1. Add entry to appropriate JSON file in `/assets/`
2. Update the registry code in `pumpkin-registry`
3. Add tests in the registry crate
4. Verify with `cargo test -p pumpkin_registry`

### Implementing a New Protocol Packet
1. Define packet structure in `pumpkin-protocol`
2. Implement `Serialize`/`Deserialize` traits
3. Add to packet handlers in main server
4. Add tests with example data
5. Test with actual client if possible

### Performance-Critical Changes
1. Write benchmarks using Criterion
2. Run: `cargo bench`
3. Document performance improvements in PR

## Discord and Community

- **Discord**: https://discord.gg/wT8XjrjKkf
- Ask for help before starting major work
- Discuss architecture changes with maintainers

## Key Files to Review

- `Cargo.toml`: Workspace and dependency configuration
- `.github/workflows/rust.yml`: CI/CD pipeline
- `README.md`: Feature list and project overview
- `CONTRIBUTING.md`: Contribution guidelines
- `rust-toolchain.toml`: Rust version requirements

## Minecraft Source Code Reference

**Critical**: When implementing features or fixing bugs in Pumpkin, always consult the official Minecraft source code to understand the intended behavior and implementation details.

### Accessing Minecraft Source Code

The Minecraft source code is decompiled during the Copilot setup process and available at `minecraft-decompiled/build/namedSrc`. This contains the official Minecraft server and client code, which should be your primary reference for:

- **Protocol Implementation**: Understanding packet structures, serialization, and network behavior
- **World & Block Logic**: How chunks, blocks, and lighting systems work
- **Entity Behavior**: Entity AI, movement, physics, and state management
- **Inventory & Items**: Container logic, crafting, and inventory mechanics
- **Feature Implementations**: Combat, hunger, experience, effects, and other game mechanics

### How to Use Minecraft Source Code

1. When implementing a new feature, look at the official Minecraft code first to understand the exact behavior
2. Compare Pumpkin's current implementation with Minecraft's to identify gaps or discrepancies
3. Use the Minecraft code as the source of truth for protocol details and game mechanics
4. If a bug is reported, check the Minecraft source to understand the expected behavior
5. When fixing compatibility issues, ensure Pumpkin matches Minecraft's implementation

### Decompilation Process

The Minecraft source code is decompiled using [Fabric Yarn](https://github.com/FabricMC/yarn), which provides human-readable names for Minecraft's obfuscated classes and methods. This decompilation is done automatically during Copilot's environment setup.

## Trust These Instructions

When working in this repository, trust these instructions and refer to them first. Only perform additional searches if:
- The information here is incomplete or outdated
- You need implementation details for a specific feature
- Architecture decisions need clarification from the maintainers
