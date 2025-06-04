use super::{Map, TileType, Position, spawner, SHOW_MAPGEN_VISUALIZER};
use rltk::{Rect};
use specs::prelude::*;
use crate::constants::AMULET_LEVEL;

mod simple_map;
mod algorithms;
mod utility;
mod levels;
mod common;
mod waveform_collapse;
mod prefab_builder;

use levels::forest::{forest_builder, forest2_builder, forest_castle_transition_builder};
use levels::limestone_cavern::*;
use levels::dwarf_fort_builder::*;
use utility::distant_exit::DistantExit;
use simple_map::SimpleMapBuilder;
use algorithms::bsp_dungeon::BspDungeonBuilder;
use algorithms::cellular_automata::CellularAutomataBuilder;
use algorithms::drunkard::DrunkardsWalkBuilder;
use algorithms::maze::MazeBuilder;
use algorithms::dla::DLABuilder;
use algorithms::voronoi::VoronoiCellBuilder;
use algorithms::voronoi_spawning::VoronoiSpawning;
use waveform_collapse::WaveformCollapseBuilder;
use prefab_builder::PrefabBuilder;
use utility::room_based_spawner::RoomBasedSpawner;
use utility::room_based_stairs::RoomBasedStairs;
use utility::starting_points::{AreaStartingPosition, DungeonEntranceSpawner, RoomBasedStartingPosition, XStart, YStart};
use utility::cull_unreachable::CullUnreachable;
use common::*;
use utility::room_exploder::RoomExploder;
use utility::room_corner_rounding::RoomCornerRounder;
use utility::rooms_corridors_dogleg::DoglegCorridors;
use utility::rooms_corridors_bsp::BspCorridors;
use utility::room_sorter::{RoomSorter, RoomSort};
use utility::room_draw::RoomDrawer;
use utility::rooms_corridors_nearest::NearestCorridors;
use utility::rooms_corridors_lines::StraightLineCorridors;
use utility::room_corridor_spawner::CorridorSpawner;
use utility::door_placement::DoorPlacement;
use utility::amulet_spawner::AmuletSpawner;
use levels::town::town_builder;
use utility::area_ending_point::*;
use levels::mushroom_forest::*;
use levels::dark_elves::*;

pub struct BuilderMap {
    pub spawn_list : Vec<(usize, String)>,
    pub map : Map,
    pub starting_position : Option<Position>,
    pub rooms: Option<Vec<Rect>>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub history : Vec<Map>,
    pub width: i32,
    pub height: i32
}

impl BuilderMap {
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.revealed_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }
}

pub struct BuilderChain {
    starter: Option<Box<dyn InitialMapBuilder>>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    pub build_data : BuilderMap
}

impl BuilderChain {
    pub fn new<S : ToString>(new_depth : i32, width: i32, height: i32, name : S) -> BuilderChain {
        BuilderChain{
            starter: None,
            builders: Vec::new(),
            build_data : BuilderMap {
                spawn_list: Vec::new(),
                map: Map::new(new_depth, width, height, name),
                starting_position: None,
                rooms: None,
                corridors: None,
                history : Vec::new(),
                width,
                height
            }
        }
    }

    pub fn start_with(&mut self, starter : Box<dyn InitialMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder.")
        };
    }

    pub fn with(&mut self, metabuilder : Box<dyn MetaMapBuilder>) {
        self.builders.push(metabuilder);
    }

    pub fn build_map(&mut self) {
        match &mut self.starter {
            None => panic!("Cannot run a map builder chain without a starting build system"),
            Some(starter) => {
                // Build the starting map
                starter.build_map(&mut self.build_data);
            }
        }

        // Build additional layers in turn
        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(&mut self.build_data);
        }
    }

    pub fn spawn_entities(&mut self, ecs : &mut World) {
        for entity in self.build_data.spawn_list.iter() {
            spawner::spawn_entity(ecs, &(&entity.0, &entity.1));
        }
    }
}

pub trait InitialMapBuilder {
    fn build_map(&mut self, build_data : &mut BuilderMap);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, build_data : &mut BuilderMap);
}

fn random_start_position() -> (XStart, YStart) {
    let x;
    let xroll = crate::rng::roll_dice(1, 3);
    match xroll {
        1 => x = XStart::LEFT,
        2 => x = XStart::CENTER,
        _ => x = XStart::RIGHT
    }

    let y;
    let yroll = crate::rng::roll_dice(1, 3);
    match yroll {
        1 => y = YStart::BOTTOM,
        2 => y = YStart::CENTER,
        _ => y = YStart::TOP
    }

    (x, y)
}

// fn random_shape_builder(builder : &mut BuilderChain) {
//     let builder_roll = crate::rng::roll_dice(1, 9);
//     match builder_roll {
//         1 => {println!("CellularAutomataBuilder"); builder.start_with(CellularAutomataBuilder::new())},
//         2 => {println!("DrunkardsWalkBuilder::winding_passages"); builder.start_with(DrunkardsWalkBuilder::winding_passages())},
//         3 => {println!("DrunkardsWalkBuilder::open_halls"); builder.start_with(DrunkardsWalkBuilder::open_halls())},
//         4 => {println!("DrunkardsWalkBuilder::fat_passages"); builder.start_with(DrunkardsWalkBuilder::fat_passages())},
//         5 => {println!("DrunkardsWalkBuilder::fearful_symmetry"); builder.start_with(DrunkardsWalkBuilder::fearful_symmetry())},
//         6 => {println!("DLABuilder::walk_inwards"); builder.start_with(DLABuilder::walk_inwards())},
//         7 => {println!("DLABuilder::central_attractor"); builder.start_with(DLABuilder::central_attractor())},  // Smaller open area
//         8 => {println!("DLABuilder::insectoid"); builder.start_with(DLABuilder::insectoid())},
//         _ => {println!("VoronoiCellBuilder::pythagoras"); builder.start_with(VoronoiCellBuilder::pythagoras())},
        
//         // _ => builder.start_with(DLABuilder::walk_outwards()),  // Another big open area
//         // _ => builder.start_with(DrunkardsWalkBuilder::open_area()), // TODO - reserve for boss?
        
//         // 7 => builder.start_with(MazeBuilder::new()),
//         // _ => builder.start_with(PrefabBuilder::constant(prefab_builder::prefab_levels::WFC_POPULATED)),
//         // _ => builder.start_with(VoronoiCellBuilder::manhattan()),
//     }

//     // Set the start to the center and cull
//     builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
//     builder.with(CullUnreachable::new());

//     // Now set the start to a random starting area
//     let (start_x, start_y) = random_start_position();
//     builder.with(AreaStartingPosition::new(start_x, start_y));

//     // Setup an exit and spawn mobs
//     builder.with(VoronoiSpawning::new());
//     match builder.build_data.map.depth {
//         AMULET_LEVEL => builder.with(AmuletSpawner::new()),
//         _ => builder.with(DistantExit::new()),
//     }
    
// }

pub fn dungeon_entrance_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let map_name = "Dungeon Entrance".to_owned();
    let mut builder = BuilderChain::new(new_depth, width, height, map_name);
    builder.start_with(BspDungeonBuilder::dungeon());
    // builder.with(RoomSorter::new(RoomSort::CENTRAL));
    builder.with(RoomDrawer::new());
    builder.with(NearestCorridors::new());
    builder.with(RoomExploder::new());

    // Spawn entities in corridors
    // let cspawn_roll = crate::rng::roll_dice(1, 2);
    // if cspawn_roll == 1 {
    //     builder.with(CorridorSpawner::new());
    // }

    builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::BOTTOM));
    builder.with(DungeonEntranceSpawner::new());
    builder.with(DistantExit::new());
    builder.with(DoorPlacement::new());

    
    // let spawn_roll = crate::rng::roll_dice(1, 2);
    // match spawn_roll {
    //     1 => builder.with(RoomBasedSpawner::new()),
    //     _ => builder.with(VoronoiSpawning::new())
    // }
    builder
}

pub fn floor_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let map_name = "Floor ".to_owned() + &new_depth.to_string();
    let mut builder = BuilderChain::new(new_depth, width, height, map_name);
    builder.start_with(BspDungeonBuilder::dungeon());
    // builder.with(RoomSorter::new(RoomSort::CENTRAL));
    builder.with(RoomDrawer::new());
    builder.with(NearestCorridors::new());
    builder.with(RoomExploder::new());

    // Spawn entities in corridors
    // let cspawn_roll = crate::rng::roll_dice(1, 2);
    // if cspawn_roll == 1 {
    //     builder.with(CorridorSpawner::new());
    // }
    
    let (start_x, start_y) = random_start_position();
    builder.with(AreaStartingPosition::new(start_x, start_y));
    builder.with(DoorPlacement::new());

    
    // let spawn_roll = crate::rng::roll_dice(1, 2);
    // match spawn_roll {
    //     1 => builder.with(RoomBasedSpawner::new()),
    //     _ => builder.with(VoronoiSpawning::new())
    // }

    match builder.build_data.map.depth {
        AMULET_LEVEL => builder.with(AmuletSpawner::new()),
        _ => {
            let exit_roll = crate::rng::roll_dice(1, 2);
            match exit_roll {
                // 1 => builder.with(RoomBasedStairs::new()),
                // TODO - better algorithm for generating exit
                _ => builder.with(DistantExit::new())
            }
        }
    }

    builder
}

pub fn level_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    rltk::console::log(format!("Depth: {}", new_depth));
    match new_depth {
        1 => dungeon_entrance_builder(new_depth, width, height),
        _ => floor_builder(new_depth, width, height)
    }
}

pub fn map_dimensions(new_depth: i32) -> (i32, i32) {
    match new_depth {
        // 1 => (40, 25), // First map is smaller to give an introduction to the dungeon
        _ => (80, 50)
    }
}