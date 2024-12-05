# Getting Set Up
## Installing Rust
TBD

# Releasing with WASM for Online Play
Following instructions from [here](https://bfnightly.bracketproductions.com/rustbook/webbuild.html)

```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target\wasm32-unknown-unknown\release\rust-roguelike.wasm --out-dir wasm --no-modules --no-typescript


```

















Basic game play:
* Win condition
  - Go down through the dungeon to retrieve the amulet on level 10 and return up the floors and out the dungeon entrance
  - If you die, game over
* Progression
  - Enemies will get stronger over time, so the character will also get stronger through equipment, spells and stat increases

* Map generation
* Mobs
  - AI
    - Sleeping, wandering, chasing, guarding
* Items
  * Generation
* Spells
* Damage System
  * Hit Chance
    - hit_chance = attacker_accuracy * .987 ^ defender_evasion
    - accuracy = 100; later multiply by 1.065^(enchant level)
    - evasion = 10 * dex bonus
  * Damage
    - damage = roll(dmg_min, dmg_max) - (roll(armor/2, armor) - attacker_pierce)
      - Min/max damage is from the attackers weapon, natural attack, or 1d4 if unarmed
      - Armor is from the defenders gear or natural armor
      - Pierce is an attribute on weapons
  * Damage types
    - Physical
    - Fire
    - Poison
  * Resistances/Weakness/Immunities
  * Status Effects
    - Burning: 1d3 for 5 turns. Inflicted by any fire attack. Additional fire damage will reset the counter. Stepping into water will extinguish. TODO - implement fire spreading on flammible terrain
    - Poison: Deal n damage for m turns. Poison stacks, new poison damage is added to the previous amount and the number of turns is increased by the new duration
* Turn system
* UI
  - Items & creatures in view should be displayed on the side
    - Health & status effects should be displayed
    - Hovering over these entities should show a detailed description
  - Hovering over an item or creature on the ground should show a description



TODO
Dungeon:
- BSPDungeon algorithm is broken; Find out why sometimes the map spawns only 1 square
- Bump into the dungeon exit
- increase size of screen
- add a view to print the entire log
- Display AC value
thread 'main' panicked at src\map_builders\utility\starting_points.rs:60:13:
No valid floors to start on
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\rust-roguelike.exe` (exit code: 101)
Rename stats to STR/CON/DEX/INT
do entities that died in current turn still get to attack?
Nameless item bug for fire
Items with charges don't recharge
Burning damage particle is on wrong tile when moving
Should burn items that are on the ground
Setting fire to fire that already exists will give the fire a "burning" status
Spells that affect things in a line
Update tunneling to dig out all paths in the line



https://www.rockpapershotgun.com/how-do-roguelikes-generate-levels
https://brogue.fandom.com/wiki/Level_Generation

  




Mobs
- Each mob should have a unique theme, possible themes include
  - basic melee
  - basic ranged, single target
  - debuff
  - inflict status effect,
  - AOE 
  - poison
  - slow
  - invisible
  - glass cannon
  - cast silence
  - tough/regenerates, low damage
  - stun only, but loses 1 hp every time it attacks
  - parasite, giving positive + negative benefits
  - spawn amalgamation if too many of same entity type die in same place
  - an enemy with knockback
  - enemy with high armor and attack, but very slow
  - stationary props that have some kind of environmental impact
- Monsters gain strength through levels by adding prefixes. Prefixes add stats  

- Monsters using items
- Monsters picking up items
- Monsters coordinating/boosting other monsters
- Mimics (appear as another symbol)
- Demons (appear as &)
- Types
  - goblins
  - orcs
  - dark elves
  - dwarves
  - dragon
  - bandits
  - slimes
  - trolls
- Bosses
- Minibosses

Spawning:
- 

Item
- Generation
  - items should have an identity
- Types
  - Consumables
    - Potion
      - Healing
      - Mana
      - Attr increase (temp vs permanent)
    - Scroll
  - Book of magic
  - Food
  - Weapons
    - bow - ranged
    - dagger - can pierce armor, initiative boost, low damage
    - sword - normal speed
    - axe - chance to bleed, initiative penalty, heavy damage
    - hammer - chance to stun, chance to knockback initiative penalty, heavy damage
  - Armor
    - Slots
      - helmet
      - chest
      - gloves
      - legs
      - boots
      - shield
    - light armor
    - heavy armor - high def; magic penalty; slow initiative
    - Robes
  - Jewelry
    - 2 slots
    - types
      - amulet (rare)
      - ring
    - modify character's attributes, grant various powers, special ability or resistance. Also they can allow magical power or spell to be activated. 

Ideas:
  - Lanterns
  - Wand
    - invokes certain power which is commonly unknown at first place, the wands have a limited number of charges or uses, and can be recharged using other actions.
  - Rod
    - Rods use energy of wielder to create desired effect or absorb it slowly from environment effectively being usable only once per certain period of time.
- Treasure rooms guarded/locked w/ spawn table for higher level loot
- more prefabs
- rare/unique items in vendors
- item rarity should affect drop chances: common, uncommon, rare, legendary
- Add memory tiles
- Update display coloring
- TODO - customizable map size per level
- status immunities
- status effects:
  - berserk
  - blind - vision reduced/removed
  - bleed - lose 2.5% max hp per tick
  - confuse - attack both ally and enemy
  - fear - causes enemies to run away from source of fear
  - stun - prevent all actions for 1 turn
  - curse - drop in stats
  - slow - reduce targets speed by 50%, less for bosses
  - poison - lose a small amount of hp per turn
  - burn
  - charmed - you control the charmed target
  - silence - target is unable to use magic spells
  - sleep - skips turn
  - trapped - can't move
  - invisible - can't be seen
  - Webbed



Bugs:
https://github.com/amethyst/rustrogueliketutorial/issues/165
https://github.com/amethyst/rustrogueliketutorial/issues/188
https://github.com/amethyst/rustrogueliketutorial/issues/208
https://github.com/amethyst/rustrogueliketutorial/issues/205
BSP fix: https://github.com/amethyst/rustrogueliketutorial/issues/156
hungover stats not coming back - encumbrance system is still getting the status, not deleted from initiative system
damage over time not giving XP - https://github.com/amethyst/rustrogueliketutorial/issues/164
high level loot spawned on floor 2; might be from magic items
A* pathfinding is incredibly slow
traps sometimes don't activate

HOSTING: https://github.com/amethyst/rustrogueliketutorial/issues/215
- https://momori.dev/posts/deploying-a-rust-wasm-app-to-github-pages/


TileSet - https://en.wikipedia.org/wiki/Code_page_437

Hovering over an item should show stats

targeting with bow causes exception
New title screen

Goblin archers should try to keep their distance
Don't drop all of an enemies loot, roll for what drops
prefab AI should stay where they were placed


Converting DND armor stats into this game

Leather armor AC = 1
Weight: 10

head: 15%
torso: 40%
legs: 25%
feet: 10%
hands: 10%

TODO
* Refactor out code from ranged & melee combat systems
* Update tooltips to display fraction CR
* Goblins moving 2 spaces / turn
* Only 1 of a mobs natural attacks are used
* Combine melee combat and ranged combat systems?
* Don't drop all of an enemies items, add to loot drop table & roll
* Display AC
* Implement 2 handed weapons
* Add in saving throws for enemy attacks
* Is hit bonus for natural attacks calculated correctly? IE, is it using a stat bonus, and can it use dex stat? Is it adding in the additional damage from bonus?
* TODO - implement saving throws
* TODO - attack damage types
* TODO - implement "Push" weapon feature - https://www.dndbeyond.com/equipment/6-greatclub
* Implement troll regeneration; don't regenertate if just took fire damage; new system for effect
* Implement fire damage
* Anything in vision range should display on side of screen
* Increase vision range??