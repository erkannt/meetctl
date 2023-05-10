## meetctl

Create and join meetings from the commandline. Also lets you share your entire screen with a single command.

Utilises the debug websocket and some chromium flags to enable these shenanigans.

To allow sharing of the entire screen without needing a human to interact with the screen sharing popup it uses `--auto-select-desktop-capture-source="Entire screen"`. This will affect other screen sharing sites/extensions you use inside the same browser session.

### Installation

`cargo install --path .`

Or download the binary from the [Releases](https://github.com/erkannt/meetctl/releases).

### Usage

Start with `launch` then you can use any of the other commands.

```
Usage: meetctl [COMMAND]

Commands:
  launch  Starts Meet in a chromium instance with debug access
  join    Takes a room name, alias or url
  share   Share you entire screen
  new     Create a new meeting and output its url
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

The join command will operate on a tab with `meet.google.com` in its url and then close all empty tabs.

You can run the `join` and `new` command if you are already in a room. It will leave the room for you.