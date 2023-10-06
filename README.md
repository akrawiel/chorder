# Chorder

A little GTK app to control your system with subsequent keystrokes (or _chords_)

![chorder window](/images/chorder.png "chorder window")

## How?

You basically compile it and run it, as simple as that (there will be packages
in the near future, I promise 🙂)

Watch the code, _there be dragons_

If you have no config file yet, an empty one will be created for you instead.
An empty window will be also a sign the app is working alright for now.
Here are the config locations according to the [`dirs`](https://crates.io/crates/dirs) crate:

| OS      | Config dir location                   | Example config file location                                   |
| ------- | ------------------------------------- | -------------------------------------------------------------- |
| Linux   | `$XDG_CONFIG_HOME` or `$HOME/.config` | `/home/alice/.config/chorder/config.json`                      |
| macOS   | `$HOME/Library/Application Support`   | `/Users/Alice/Library/Application Support/chorder/config.json` |
| Windows | `{FOLDERID_RoamingAppData}`           | `C:\Users\Alice\AppData\Roaming\chorder\config.json`           |

An empty `config.json` file will look like this:

```json
{
  "max_rows": 3,
  "max_columns": 4,
  "margin": 16,
  "spacing": 16,
  "button_width": 150,
  "button_height": 150,
  "shell": "",
  "options": {}
}
```

The things you have to fill in are the `options` key and optionally `shell` key
if you'd like to run scripts instead of just apps. Basic `options` look like
this:

```json
{
  "options": {
    "apps": [
      {
        "run": "pavucontrol",
        "description": "Run pavucontrol",
        "shortcut": "v"
      }
    ],
    "scripts": [
      {
        "scripts": "$HOME/Scripts/reboot.sh",
        "description": "Reboot",
        "shortcut": "s-r"
      }
    ],
    "main": [
      {
        "description": "apps",
        "switch": "apps",
        "shortcut": "a"
      },
      {
        "shortcut": "s",
        "description": "scripts",
        "switch": "scripts"
      }
    ]
  }
}
```

The main portion of the `options` is the array under the `main` key - this will
be an entry point to your `chorder` setup. Only 2 keys are required in each
option from the array: `shortcut` & `description`. These will display inside of
the app and will be used to operate on the chorder keystrokes.

## TODO

- write more in README
- write about what does what
- write about _Emacsified_ key modifiers
- write about `$HOME` case
- package the app up for multiple systems
- do a little bit more testing
- after that: system requirements
- add video recording of app works
