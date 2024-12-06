# Feature Requests
* Bumping into the dungeon exit should try to leave dungeon, rather than standing on the exit and pressing ','
* It would be nice if the dungeon exit was always at the bottom center of the first dungeon floor
* Increase the size of the screen, especially for the web
* Add a view that can be accessed that shows the entire message history log
* Rename stats to STR/CON/DEX/INT
* It would be nice to have an optional? recharge for chargeable items, such as staves. The recharge time would be like 500 steps and could be higher the more potent the item is
* It would be an interesting concept for fire on the map to destroy any items in the same cell
* Revisit fire mechanics - currently not scalable damage. Determine tradeoffs of boosting duration, intensity (damage/turn), fire stacks, etc
    * Adding a stat to boost fire stacks (damage done from fire attacks) would be an interesting stat on items or as a skill
* Add support for spells doing damage in a line, such as the tunneling items
* Update tunneling staff to tunnel more than 1 cell
* Update tunneling staff targeting to only select walls (not floors)
* Determine tradeoff of enemies dropping all of their equipment vs chance to drop 1 or more items, vs random loot drop, or some combination of approaches
* High level loot is spawned in early map levels, likely because they are magic items (IE mace +1 loot). The auto-generated magic items may not be using the spawn table loot distributions
* Hovering over an item should show stats
* Hovering over an enemy should show stats
* It would be nice to have a list of all the items/enemies in line of sight be listed on the side of the screen, like Brogue does
* New title screen
* Add concept for sleeping/guarding/idle AI. So any prefabs we place won't have their mobs just start wandering off
* We need a better speed/initiative system. Ideally one that allocates X number of "energy" per iteration, where "X" is determined by the monsters stats. Then actions would cost "Y" amount of energy. You might have enough energy to perform 2 actions per turn, or it may take you multiple turns to make your action happen
    * The current random initiative system + encumberance penalty just isn't fun
* Mobs should be able to use all of their natural attacks, not just randomly select one. IE - a dragon with a bite and 2 claw attacks should do all 3 attacks, not a randomly selected attack
* Look into combining the melee & ranged combat systems, they are pretty identical
* Determine how to add in damage types, IE expand pass simple physical damage into fire, poison, etc. Does the attack itself inflict that damage type? Does the attack inflict a burning or poison status effect? etc
* How does the vision range feel? Does it need updated?
* Each type of weapon should have a unique stat or effect. Such as a spear hitting 2 tiles in front of character, while an axe does AOE in a circle around character, etc. Giving a unique feel for each type of weapon makes for more interesting gameplay options, rather than just swapping out for a weapon with better stats
    * Possible effects - club=knockback, spear=pierce, sword=more damage, axe=AOE around user
* Add additional types of ground tiles, such as grass & long grass. Long grass would obscure vision
* Update fire system to let fire spread if any burnable tiles, such as grass, are adjacent to a current fire tile