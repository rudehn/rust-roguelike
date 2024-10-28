Progression
- Start off in town (level 1)
- A dragon has stolen your son, reach level 15 to defeat the dragon and rescue your son
- You progress through a forest (levels 2-4, 4 is a transition to castle), arrive at an abandoned castle (level 5) with a miniboss
  guarding the entrance down into the sewer (level 6) catacombs (level 7) which transition into a narrow passage
  cave system (8). From there, progress through the cave system (9-10) until you reach reach the miniboss
  guardian guarding the route to the abandoned dwarf mines. The dwarf mines (11-14) show remnants of the dwarfs fight with the dragon faction, relics of dwarf tech remain. The mines show structured buildings that have been weathered into the cave system, a final miniboss guards the entrance to the dragon lair (15). Inside the dragon layer is a vast cavern with a dragon guarding a locked room with your son. Defeat the dragon and rescue your son
  to finish the game!

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



Level Design
- read from files like items

Spells

Skills

Combat
- NO AC, armor mitigates damage
- damage types; resistances, weaknesses

Mobs
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

Damage system:
- Damage types
  - Physical, poison, lightning, fire
  - Hit points, drain mana, drain hunger, stat loss


Updates:
- Make effect duration for spells customizable

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



Quests


https://www.roguebasin.com/index.php/Monster_attacks_in_a_structured_list
https://www.roguebasin.com/index.php/Monster_attacks


Creating a design document: https://bfnightly.bracketproductions.com/rustbook/chapter_44.html

To implement slow with duration, update 'apply_effects' macro to read slow and confusion duration and set the value on the component. Then update `event_trigger` to use the duration value


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
traps not doing damage
vendors can block the exit of buildings
NPCs get stuck going into doors & then block

HOSTING: https://github.com/amethyst/rustrogueliketutorial/issues/215
- https://momori.dev/posts/deploying-a-rust-wasm-app-to-github-pages/


TileSet - https://en.wikipedia.org/wiki/Code_page_437

Hovering over an item should show stats
Update graphics - don't do greyscale - divide color by 8 instead
- tile colors look weird

targeting with bow causes exception
New title screen

Goblin archers should try to keep their distance
Don't drop all of an enemies loot, roll for what drops
prefab AI should stay where they were placed