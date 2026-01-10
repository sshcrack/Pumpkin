# Implement 4 Easy TODO Items

Your task is to implement the following 4 TODO items in the Pumpkin Minecraft server codebase. These are relatively straightforward changes that improve data handling, performance, and extensibility.

## TODOs to Implement

### 1. Use Perfect Hash Function (phf) for Data Components
**Location**: `pumpkin-data/build/data_component.rs` around line 38
**Current Issue**: Data components are currently looked up using a non-optimal method. Use the `phf` crate for compile-time perfect hashing.
**Requirements**:
- Add `phf` to dependencies in pumpkin-data `Cargo.toml`
- Replace the current data component lookup with a `phf::Map`
- Update tests to verify the lookup still works correctly
- Document the performance improvement in your PR

### 2. Allow Plugins to Access Config Packets
**Location**: `pumpkin/src/net/java/config.rs` around line 143
**Current Issue**: The config phase packets are not accessible to plugins. Add a hook for plugins to handle or modify config packets.
**Requirements**:
- Define a config packet event/hook that plugins can listen to
- Call the plugin hook during config packet processing
- Ensure the hook allows plugins to cancel or modify packet handling
- Add documentation about the new plugin API

### 3. Validate Operator and Ban Lists on Load
**Locations**: 
- `pumpkin/src/data/op_data.rs` around line 30
- `pumpkin/src/data/banned_player_data.rs` around line 48
- `pumpkin/src/data/banned_ip_data.rs` around line 44

**Current Issue**: The operator and ban lists are loaded but not validated. Add validation to ensure data integrity.
**Requirements**:
- Create validation functions for each list type
- Check for duplicate entries
- Check for malformed UUIDs or IPs
- Log warnings for invalid entries and skip them
- Ensure the server continues to start even if some entries are invalid

### 4. Implement Offline Mode UUID Generation
**Location**: `pumpkin/src/net/java/login.rs` around line 49
**Current Issue**: Offline mode UUID generation is not implemented. Players joining in offline mode need proper UUIDs.
**Requirements**:
- Implement UUID v3 (MD5-based) using the namespace "OfflineMode:PlayerName"
- Use the username as input to generate consistent UUIDs per player
- Ensure the UUID format matches Minecraft's offline mode UUID format
- Add tests to verify UUID generation produces consistent results
- Reference: Minecraft's offline mode uses `UUID.nameUUIDFromBytes("OfflineMode:<username>".getBytes())`

## Implementation Notes

1. **Start with validation**: TODOs #3 and #4 are the most self-contained and have fewer dependencies
2. **Use Minecraft source code**: Reference the decompiled Minecraft code in `minecraft-decompiled/build/namedSrc` for UUID generation and validation patterns
3. **Testing**: Each change should have corresponding tests or validation that confirms the functionality works
4. **Code quality**: Ensure all code passes `cargo clippy` and `cargo fmt`
5. **Documentation**: Update any relevant documentation or comments when adding new functionality

## Acceptance Criteria

- [ ] All 4 TODOs are implemented
- [ ] Code compiles without warnings: `cargo clippy --all-targets --all-features`
- [ ] All tests pass: `cargo test --verbose`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] Changes follow Pumpkin architecture guidelines
- [ ] New code references Minecraft's implementation where applicable
- [ ] Each change includes appropriate error handling and validation
