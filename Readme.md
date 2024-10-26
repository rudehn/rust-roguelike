Item
- Generation
  - items should have an identity
- Types
  - Potion
  - Scroll
  - Book of magic
  - Lanterns
  - Food
  - Weapons
    - bow
    - dagger
    - sword
    - axe
    - hammer
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
  - Wand
    - invokes certain power which is commonly unknown at first place, the wands have a limited number of charges or uses, and can be recharged using other actions.
  - Rod
    - Rods use energy of wielder to create desired effect or absorb it slowly from environment effectively being usable only once per certain period of time.

Progression


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
- Treasure rooms guarded/locked w/ spawn table for higher level loot
- more prefabs
- rare/unique items in vendors
- item rarity should affect drop chances: common, uncommon, rare, legendary
- Add memory tiles
- Update display coloring
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
hungover stats not coming back
damage over time not giving XP - https://github.com/amethyst/rustrogueliketutorial/issues/164
high level loot spawned on floor 2; might be from magic items
A* pathfinding is incredibly slow
traps not doing damage
vendors can block the exit of buildings
NPCs get stuck going into doors & then block

HOSTING: https://github.com/amethyst/rustrogueliketutorial/issues/215