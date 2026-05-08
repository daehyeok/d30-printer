# Track Specification - Refactor Bluetooth discovery and error handling

## Overview
This track focuses on improving the robustness of the Bluetooth Low Energy (BLE) discovery process and enhancing error handling across the application. The goal is to ensure a more reliable connection experience and provide clearer feedback to the user when issues occur.

## Objectives
- **Robust Discovery:** Improve the logic for scanning and identifying the Phomemo D30 device.
- **Graceful Error Handling:** Implement better error types and user-facing messages for common Bluetooth failures (e.g., adapter not found, permission denied, connection timeout).
- **Code Quality:** Refactor existing Bluetooth logic in `src/main.rs` and `src/btl.rs` for better modularity and readability.

## Requirements
- Maintain existing CLI arguments and functionality.
- Ensure the app doesn't crash on recoverable Bluetooth errors.
- Provide actionable hints in error messages (as per Product Guidelines).
