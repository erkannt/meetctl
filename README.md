## meetctl

Join Google Meet rooms from the command line.

Uses debug websocket connection to the browser to interact with the Google Meet UI on your behalf.

Sadly can't get it to share the full screen as the associated pop up isn't accessible via the debug websocket.

### Installation

`cargo install --path .`

Or download the binary from the [Releases](https://github.com/erkannt/meetctl/releases).

### Usage

```
meetctl launch <name-of-your-profile-directory>
meetctl join <room-name-or-url>
```

The join command will operate on a tab with _Meet_ in its title and then close all other tabs.

You can run the `join` command if you are already in a room. It will leave the room for you.