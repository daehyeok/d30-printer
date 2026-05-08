# Initial Concept
d30-printer is a simple command-line interface (CLI) for the Phomemo D30 label maker. This project is inspired by [crabdancing/phomemo-d30](https://github.com/crabdancing/phomemo-d30) and is designed to provide a cross-platform solution for users.

# Product Definition - d30-printer

## Vision
A simple, cross-platform command-line interface (CLI) for the Phomemo D30 label maker, enabling users to print high-quality labels directly from their terminal. It aims to provide a robust alternative to mobile apps with a focus on ease of use and flexibility.

## Target Audience
- **CLI Enthusiasts:** Users who prefer terminal-based workflows.
- **D30 Owners:** Individuals looking for a cross-platform (macOS/Linux) desktop solution for their label maker.
- **Developers:** Users who want to automate label printing or integrate it into larger scripts and systems.

## Core Value Proposition
- **Seamless Connectivity:** Automatic discovery of D30 devices via Bluetooth Low Energy (BLE).
- **Flexibility:** Extensive font support, including system fonts and custom font files.
- **Simplicity:** A clean CLI interface that requires minimal configuration to get started.

## Key Features
- **Auto-discovery:** Scans and identifies Phomemo D30 devices without requiring manual MAC address entry.
- **Font Customization:** Leverages `rusttype` and `findfont` to allow users to use any font for their labels.
- **Cross-Platform:** Built with Rust to ensure consistent performance across macOS and Linux.
