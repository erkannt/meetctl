## meetctl

Join Google Meet rooms from the command line.

Uses debug websocket connection to the browser to join rooms in an existing Meet tab.

Sadly can't get it to share the screen as the associated pop up isn't accessible via the debug websocket.

### Installation

`cargo install --path .`

Or download the binary from the [Releases](https://github.com/erkannt/meetctl/releases).

### Usage

```
meetctl launch <name-of-your-profile-directory>
meetctl join <room-name-or-url>
```

The join command will operate on a tab with `meet.google.com` in its url and then close all other tabs.

You can run the `join` command if you are already in a room. It will leave the room for you.