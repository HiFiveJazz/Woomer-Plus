
# Woomer+

![](./demo.gif)

A Blazing fast Zoomer application for Hyprland, heavily inspired by [Woomer](https://github.com/coffeeispower/woomer) and [Boomer](https://github.com/tsoding/boomer)
## Dependencies

### Arch Linux
This project assumes usage of wlr-screencopy and Hyprland (for now!)
```bash
sudo pacman -S hyprland wlr-screencopy
```


If you want to compile it manually, nightly rust is required. 
```bash
sudo pacman -S rustup
rustup default stable
rustup toolchain install nightly
```

## Quick Start



Build and compile it manually or download a pre-compiled binary from [releases](https://github.com/HiFiveJazz/Woomer-Plus/releases/)
```bash
cargo +nightly build --release #Minimize build size
./zoomer --help
./zoomer
```

## Controls

| Control                                   | Description                                                   |
|-------------------------------------------|---------------------------------------------------------------|
| <kbd>q</kbd> or <kbd>ESC</kbd>            | Quit the application                                         |
| <kbd>Ctrl</kbd>                           | Flashlight Effect                  |
| <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>🖱️Scroll Wheel</kbd>                           | Resize Flashlight |
| Drag with <kbd>🖱️Left Mouse Button</kbd>               | Move the image around                                        |
| <kbd>🖱️Scroll Wheel</kbd>| Zoom in/out                                                  |

## Coming Soon

- TOML Config File
- AUR Package
- Drawing on the Image with <kbd>🖱️Right-Click</kbd>
