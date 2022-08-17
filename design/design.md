
# hectic dungeon design document

hectic dungeon is a rogue-lite turn based dungeon crawler. enter a variety of
dungeons to collect loot and upgrades. however, if you die, that character and
the items will gone forever.

## gameplay

turn based system. the player gets a set amount of time to make a move (be it
move or attack), and then the enemies will respond with their own move.

a variety of loot can be collected from the dungeon, this can include weapons,
armor, crafting materials (ores, herbs) or consumables (potions, consumable
weapons) and brought back home to your vault. on the next run, you can use
items from your vault.

allowing persistent items gives value to gathering gear from previous dungeons
(encouraging replayability), but the permadeath aspect adds an element of risk
and reward.

you also get a set number of character slots, where you can create characters
of different classes that can equip different gear.

goal of different gear is that they all encourage different playstyles and
allow the player to experiment.

## dungeon generation

dungeon generation algorithm needs to be decided (can even vary based on
dungeon)

potential dungeon win condition
- kill all enemies
- find exit
- find all 'keys'
- wave based survival

