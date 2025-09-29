# d30-printer

## Description

d30-printer is a simple command-line interface (CLI) for the Phomemo D30 label maker. This project is inspired by [crabdancing/phomemo-d30](https://github.com/crabdancing/phomemo-d30) and is designed to provide a cross-platform solution for users. 

## Features

-  **Cross-Platform Support**: Tested on macOS and Linux.
-  **Automatic Device Discovery**: Search for the D30 label maker without needing to provide the MAC address.
-  **Font Flexibility**: Supports system fonts and allows users to provide custom font files.

## Usage
```
Usage: d30-printer [OPTIONS] <TEXT>

Arguments:
  <TEXT>  The text to be printed on the label

Options:
  -a, --addr <ADDR>  The MAC address of the D30 label maker. (Optional)
  -f, --font <FONT>  The font name or the path to the font file. If not set, 'Hack' font will be used by default. (Optional)
  -h, --help         Print help
  -V, --version      Print version
```