# Tech Stack - d30-printer

## Core Technologies
- **Rust (2024 Edition):** The primary programming language, chosen for its safety, performance, and excellent cross-platform support.
- **Tokio:** The asynchronous runtime used for non-blocking I/O, particularly for Bluetooth communication.
- **Btleplug:** A cross-platform Bluetooth Low Energy library for Rust.
- **Clap (v4):** A command-line argument parser for building user-friendly CLI interfaces.

## Supporting Libraries
- **Image & Imageproc:** Used for generating and manipulating the label images before they are sent to the printer.
- **Rusttype & Findfont:** Handles font loading and glyph rendering into the label images.
- **Anyhow:** Provides easy error management.
- **Env_logger & Log:** Standard logging framework for diagnostics.

## Development Environment
- **Cargo:** Rust's package manager and build system.
- **Devenv:** Used for managing the development environment (as indicated by `devenv.yaml`).
- **GitHub Actions:** Configured for automated testing (as indicated by `.github/workflows/test.yml`).
