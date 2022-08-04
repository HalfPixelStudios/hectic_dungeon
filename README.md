
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
- [ ] enemy attacking
- [x] map loading
- [ ] traps / environemntal hazards
- [ ] item pickup
- [ ] animation / tweening system

