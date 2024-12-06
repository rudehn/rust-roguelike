# Known bugs
* The multi-processing system dispatcher is currently disabled. When it's enabled, system execution order isn't deterministic, so some systems will have bugs, like traps not firing when stepped on
* Some dungeon layouts will not spawn an outer wall. Floor tiles are adjacent to the map out of bounds
* thread 'main' panicked at src\map_builders\utility\starting_points.rs:60:13: No valid floors to start on
* Hovering over a fire entity that spawned this turn will show `Nameless item (bug)`
* Setting fire to fire that already exists will give the fire a "burning" status
* https://github.com/amethyst/rustrogueliketutorial/issues/165
* https://github.com/amethyst/rustrogueliketutorial/issues/188
* https://github.com/amethyst/rustrogueliketutorial/issues/208
* https://github.com/amethyst/rustrogueliketutorial/issues/205
* damage over time not giving XP - https://github.com/amethyst/rustrogueliketutorial/issues/164
* A* pathfinding is incredibly slow
* Targeting with the bow can sometimes cause an exception