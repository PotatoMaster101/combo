# Combo
A small tool to automatically type in any key combo when a trigger key is pressed. This tool is mostly used to play HellDivers.

## Building
Requires [`cargo`](https://www.rust-lang.org/) to build:
```
$ cargo build --release
```

## Usage
```
$ combo --help
Combo keys mapper

USAGE:
    combo [OPTIONS] --url <URL>

OPTIONS:
    -h, --help         Print help information
    -l, --local        Specifies the config file is a local file
    -u, --url <URL>    Config file URL or path
```

## Example
Using a config file on local PC:
```
$ combo -u configs/helldivers/shredder.json -l
```

Using a config file from a URL:
```
$ combo -u https://raw.githubusercontent.com/PotatoMaster101/combo/main/configs/helldivers/shredder.json
```

## Config File
Config files are JSON files containing data of the combo. Example config files can be found under `configs`. Config entries:
- `combo`: list of keys to press in sequence for this combo
- `delay`: delay in milliseconds between each key press
- `description`: description of the combo, for human eyes only
- `modifiers`: list of modifier keys to hold down before the combo can be invoked
- `repeat`: whether to repeat the combo as long as trigger key is pressed
- `trigger`: trigger key for invoking the combo

## Key Codes
Keyboard keys are represented as key codes. Some most used keys:
- `A`: 65
- `B`: 66
- `C`: 67
- `D`: 68
- `E`: 69
- `F`: 70
- `G`: 71
- `H`: 72
- `I`: 73
- `J`: 74
- `K`: 75
- `L`: 76
- `M`: 77
- `N`: 78
- `O`: 79
- `P`: 80
- `Q`: 81
- `R`: 82
- `S`: 83
- `T`: 84
- `U`: 85
- `V`: 86
- `W`: 87
- `X`: 88
- `Y`: 89
- `Z`: 90
- `LCtrl`: 162
- `LShift`: 160
