# Product Guidelines - d30-printer

## Tone and Voice
- **Technical/Direct:** Documentation and CLI output should be professional, clear, and focused on technical accuracy. Avoid fluff and be precise with terminology.

## UX Principles
- **Error Recovery:** Prioritize helpful error messages. If a Bluetooth connection fails or a font isn't found, suggest specific remedies (e.g., "Check if your D30 is turned on" or "Run with --font to specify a valid path").
- **Speed/Efficiency:** The CLI should minimize latency, especially during device discovery and image processing.
- **Intuitive CLI Design:** Use consistent naming for flags and arguments (following `clap` best practices). Ensure `--help` provides comprehensive yet concise guidance.

## Visual Identity
- **Playful Terminal:** While the tone is technical, the output can use emojis or simple ASCII elements to improve readability and provide a more modern terminal experience. Use colors sparingly to highlight key information (errors in red, success in green).

## Quality Standards
- **Reliability:** Ensure robust handling of Bluetooth connection drops.
- **Accuracy:** The printed output must accurately reflect the user's text and font choices.
