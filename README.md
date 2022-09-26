
# hectic dungeon

**hectic dungeon** is a turn-based tiled dungeon crawler game. It's a reboot
and bevy rewrite of the original
[Hectic-Dungeon](https://github.com/HalfPixelStudios/Hectic-Dungeon) that was
entered for a game jam.

## DEV SETUP

Some features of the nightly build of `rustfmt` are required. Ensure that you
have ran:
```
$ rustup install nightly
```

`dylint` is also required to add additional bevy related lints, you can install
with:
```
$ cargo install cargo-dylint dylint-link
```

Next install some git hooks:
```
$ just devsetup
```

Finally to run the game:
```
$ just
```

## TODO
- [x] basic player and enemy movement
- [x] enemy pathfinding
- [x] player attacking
- [x] enemy attacking
- [x] map loading
- [x] traps / environemntal hazards
- [ ] item pickup
- [ ] animation / tweening system
- [ ] map generation (room system)
- [ ] ui (might be blocked by 0.8)

