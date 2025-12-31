# labelwin

A tiny, movable label window for screen recordings. Built with Rust and [egui](https://github.com/emilk/egui).

## Features

- Customizable text, colors, and font size
- Optional window decorations (use `--undecorated` for a clean overlay)
- Always-on-top mode for screen recordings
- Draggable window when undecorated

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
```

## Usage

```bash
labelwin [OPTIONS]
```

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `--text <TEXT>` | `Claude Code` | The label text to display |
| `--bg <COLOR>` | `#cc0000` | Background color (hex: `#RRGGBB` or `#RRGGBBAA`) |
| `--fg <COLOR>` | `#ffffff` | Foreground/text color (hex format) |
| `--font-size <SIZE>` | `28` | Font size in pixels |
| `--width <WIDTH>` | `260` | Window width in pixels |
| `--height <HEIGHT>` | `80` | Window height in pixels |
| `--undecorated` | `false` | Remove window decorations (title bar, borders) |
| `--always-on-top` | `false` | Keep window above all other windows |
| `--title <TITLE>` | `Label` | Window title (when decorated) |

### Examples

Default label:
```bash
labelwin
```

Custom text with blue background:
```bash
labelwin --text "Recording" --bg "#0066cc"
```

Minimal overlay for screen recordings:
```bash
labelwin --text "LIVE" --bg "#ff0000" --undecorated --always-on-top
```

Custom dimensions:
```bash
labelwin --text "Demo" --width 400 --height 100 --font-size 48
```

## License

See [LICENSE](LICENSE) for details.
