# Implementation Plan - Refactor Bluetooth discovery and error handling

## Phase 1: Analysis and Error Definition [checkpoint: d9f1e53]
- [x] Task: Audit current Bluetooth logic in `src/btl.rs` and `src/main.rs` (3c52482)
- [x] Task: Define custom error types using `anyhow` or a dedicated enum (fd48daa)
- [x] Task: Conductor - User Manual Verification 'Phase 1: Analysis and Error Definition' (Protocol in workflow.md) (d9f1e53)

## Phase 2: Refactor Discovery Logic
- [ ] Task: Modularize discovery scan in `src/btl.rs`
- [ ] Task: Implement timeout handling for discovery
- [ ] Task: Update `main.rs` to use new discovery interface
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Refactor Discovery Logic' (Protocol in workflow.md)

## Phase 3: Enhanced Error Handling and UX
- [ ] Task: Replace generic errors with specific user-facing messages
- [ ] Task: Implement "Playful Terminal" styling for error output
- [ ] Task: Final manual verification of all error paths
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Enhanced Error Handling and UX' (Protocol in workflow.md)
