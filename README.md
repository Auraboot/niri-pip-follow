# niri-pip-follow

A small Rust helper that moves a floating Picture-in-Picture window
to the currently focused niri workspace.

Currently configured for a Chromium-family PiP window titled
`Picture in picture` with an empty app ID.

## Build

```sh
cargo build --release --locked
```

## Run

```sh
./target/release/pip-follow
```

Stop with Ctrl+C. A running niri session is required.

## Autostart with niri

Copy the built binary:

```sh
mkdir -p ~/.local/bin
install -m 755 target/release/pip-follow ~/.local/bin/pip-follow
```

Add the following to your niri configuration, replacing `USERNAME`
with your Linux username:

```kdl
spawn-at-startup "/home/USERNAME/.local/bin/pip-follow"
```

For iNiR, this can go in `~/.config/niri/config.d/90-user-extra.kdl`.
Check the configuration with `niri validate`. The helper starts on
the next niri session.

## Limitations

- The window is moved between workspaces rather than truly pinned.
- PiP may not appear above fullscreen windows.
- Browser playback controls may briefly appear when the window moves.

## License

MIT. See [LICENSE](LICENSE).
