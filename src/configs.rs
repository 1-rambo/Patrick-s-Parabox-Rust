use std::collections::HashMap;
use std::fs::File;
use std::fmt::Debug;
use serde_json::Value;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct LevelConfig {
    level: u32,
    pub paraboxes: Vec<Parabox>,
    player_pos: (u32, (u32, u32)), // (box_id, (x, y))
}

impl LevelConfig {
    pub fn new(level: u32, file: &str) -> Self {
        let file = File::open(file).expect("Failed to open level config file");
        let data: Value = serde_json::from_reader(file).expect("Failed to parse level config file");
        let player_pos = (
            data["player_pos"][0].as_u64().expect("player_pos x should be a number") as u32,
            (
                data["player_pos"][1].as_u64().expect("player_pos y should be a number") as u32,
                data["player_pos"][2].as_u64().expect("player_pos z should be a number") as u32,
            ),
        );
        let mut paraboxes: Vec<Parabox> = Vec::new();
        for (id, parabox) in data["paraboxes"].as_array().expect("paraboxes should be an array").iter().enumerate() {
            let (size_x, size_y) = (
                parabox["size"][0].as_u64().expect("parabox size width should be a number") as u32,
                parabox["size"][1].as_u64().expect("parabox size height should be a number") as u32,
            );
            let mut new_parabox = Parabox::new(id as u32, (size_x, size_y));
            let map = parabox["map"].as_object().expect("parabox map should be an object");
            for wall_pos in map["walls"].as_array().expect("walls should be an array") {
                
                let x = wall_pos[0].as_u64().expect("wall x should be a number") as u32;
                let y = wall_pos[1].as_u64().expect("wall y should be a number") as u32;
                new_parabox.add_square(x, y, Square::Wall);
            }
            for x in 0..size_x {
                for y in 0..size_y {
                    if let Some(square) = map.get(&format!("({}, {})", x, y)) {
                        println!("pos: ({}, {}), square: {:?}", x, y, square);
                        match square["type"].as_str().expect("square should be a string") {
                            "Block" => new_parabox.add_square(x, y, Square::Block),
                            _ => unimplemented!(),
                        }
                    }
                }
            }
            let targets = parabox["targets"].as_array().expect("targets should be an array");
            for target in targets {
                let x = target[0].as_u64().expect("target x should be a number") as u32;
                let y = target[1].as_u64().expect("target y should be a number") as u32;
                new_parabox.add_target(x, y);
            }
            if id == player_pos.0 as usize {
                new_parabox.set_player_pos(player_pos.1 .0, player_pos.1 .1);
            }
            if !parabox["player_target"].is_null() {
                let target = parabox["player_target"].as_array().expect("player_target should be an array");
                new_parabox.set_player_target(
                    target[0].as_u64().expect("player_target x should be a number") as u32,
                    target[1].as_u64().expect("player_target y should be a number") as u32,
                );
            }
            paraboxes.push(new_parabox);
        }
        LevelConfig {
            level,
            paraboxes,
            player_pos,
        }
    }

    pub fn shift(&mut self, pos: (i32, i32)) -> bool {
        assert!((pos.0 == 0 || pos.1 == 0), "Invalid shift: {:?}", pos);
        assert!((pos.0.abs() <= 1 && pos.1.abs() <= 1), "Shift too large: {:?}", pos);
        if let Some(parabox) = self.paraboxes.get_mut(self.player_pos.0 as usize) {
            let mut new_pos = (
                (self.player_pos.1 .0 as i32 + pos.0),
                (self.player_pos.1 .1 as i32 + pos.1),
            );
            while let Some(Square::Block | Square::Parabox(_)) = parabox.find_at(new_pos.0 as u32, new_pos.1 as u32) {
                new_pos = (
                    new_pos.0 + pos.0,
                    new_pos.1 + pos.1,
                );
            }
            println!("Current position: {:?}", self.player_pos.1);
            println!("Attempting to move to: {:?}", new_pos);
            if new_pos.0 < 0 || new_pos.1 < 0 ||
                new_pos.0 >= parabox.size.0 as i32 || new_pos.1 >= parabox.size.1 as i32 {
                unreachable!("New position out of bounds: {:?}", new_pos);
            }
            else if let Some(Square::Wall) = parabox.find_at(new_pos.0 as u32, new_pos.1 as u32) {
                println!("New position is a wall: {:?}", new_pos);
            }
            else {
                println!("New position is valid: {:?}", new_pos);
                if new_pos.0 == self.player_pos.1 .0 as i32 {
                    let mut final_pos = (new_pos.0, new_pos.1 - pos.1);
                    while final_pos.1 != self.player_pos.1 .1 as i32 {
                        let block_origin = parabox.find_at(final_pos.0 as u32, final_pos.1 as u32).unwrap();
                        parabox.add_square(final_pos.0 as u32, (final_pos.1 + pos.1) as u32, block_origin.clone());
                        parabox.remove_square(final_pos.0 as u32, final_pos.1 as u32);
                        final_pos.1 -= pos.1;
                    }
                }
                else if new_pos.1 == self.player_pos.1 .1 as i32 {
                    let mut final_pos = (new_pos.0 - pos.0, new_pos.1);
                    while final_pos.0 != self.player_pos.1 .0 as i32 {
                        let block_origin = parabox.find_at(final_pos.0 as u32, final_pos.1 as u32).unwrap();
                        parabox.add_square((final_pos.0 + pos.0) as u32, final_pos.1 as u32, block_origin.clone());
                        parabox.remove_square(final_pos.0 as u32, final_pos.1 as u32);
                        final_pos.0 -= pos.0;
                    }
                } else {
                    unreachable!("Invalid move: {:?}", new_pos);
                }
                parabox.set_player_pos(
                    (self.player_pos.1 .0 as i32 + pos.0) as u32,
                    (self.player_pos.1 .1 as i32 + pos.1)as u32
                );
                self.player_pos.1 = (
                    (self.player_pos.1 .0 as i32 + pos.0) as u32,
                    (self.player_pos.1 .1 as i32 + pos.1) as u32,
                );
            }
        } else {
            panic!("Parabox with id {} not found", self.player_pos.0);
        }
        self.check_win()
    }

    fn check_win(&self) -> bool {
        for parabox in &self.paraboxes {
            if !parabox.check_win() {
                return false;
            }
        }
        return true;
    }
}

impl Debug for LevelConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LevelConfig {{ level: {}, player_pos: {:?}, paraboxes: {:?} }}", self.level, self.player_pos, self.paraboxes)
    }
}

#[derive(Resource, Clone)]
pub struct Parabox {
    id: u32,
    outer: Option<Box<Parabox>>,
    map: HashMap<(u32, u32), Square>,
    player_pos: Option<(u32, u32)>, // (x, y)
    player_target: Option<(u32, u32)>, // (x, y)
    size: (u32, u32), // (width, height)
    targets: Vec<(u32, u32)>, // List of target positions
}

impl Parabox {
    fn new(id: u32, size: (u32, u32)) -> Self {
        Parabox {
            id,
            outer: None,
            map: HashMap::new(),
            player_pos: None,
            player_target: None,
            size,
            targets: Vec::new(),
        }
    }

    fn add_square(&mut self, x: u32, y: u32, square: Square) {
        self.map.insert((x, y), square);
    }

    fn remove_square(&mut self, x: u32, y: u32) {
        self.map.remove(&(x, y));
    }

    fn set_player_pos(&mut self, x: u32, y: u32) {
        self.player_pos = Some((x, y));
    }

    fn set_player_target(&mut self, x: u32, y: u32) {
        self.player_target = Some((x, y));
    }

    fn add_target(&mut self, x: u32, y: u32) {
        self.targets.push((x, y));
    }

    fn find_at(&self, x: u32, y: u32) -> Option<&Square> {
        self.map.get(&(x, y))
    }

    fn check_win(&self) -> bool {
        if let Some(player_target) = self.player_target {
            if let Some(player_pos) = self.player_pos {
                if player_pos != player_target {
                    return false;
                }
            }
            else {
                return false;
            }
        }
        for (target_x, target_y) in &self.targets {
            if let Some(Square::Block) | Some(Square::Parabox(_)) = self.find_at(*target_x, *target_y) {
                continue;
            }
            else {
                return false;
            }
        }
        return true;
    }
}

impl Debug for Parabox {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "Parabox id: {}, size: {:?}", self.id, self.size)?;
        // if let Some(outer) = &self.outer {
        //     write!(f, ", outer_id: {:?}", outer.id)?;
        // } else {
        //     write!(f, ", outer: None")?;
        // }
        // writeln!(f, "")?;
        let mut map = String::from("");
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                if let Some(square) = self.map.get(&(x, y)) {
                    map += &format!("{:?}", square);
                }
                else {
                    map += "."; // Empty space
                }
            }
            map += "\n";
        }
        if let Some((player_target_x, player_target_y)) = self.player_target {
            let pos = (player_target_x * (self.size.1 + 1) + player_target_y) as usize;
            if map.chars().nth(pos).unwrap() == '.' {
                map.replace_range(pos..pos + 1, "=");
            }
        }
        for (target_x, target_y) in &self.targets {
            let pos = (target_x * (self.size.1 + 1) + target_y) as usize;
            if map.chars().nth(pos).unwrap() == '.' {
                map.replace_range(pos..pos + 1, "_");
            }
        }
        if let Some((player_x, player_y)) = self.player_pos {
            let pos = (player_x * (self.size.1 + 1) + player_y) as usize;
            map.replace_range(pos..pos + 1, "p");
        }
        for ch in map.chars() {
            if ch == '\n' {
                write!(f, "\n")?;
            } else {
                write!(f, "{}", ch)?;
            }
        }
        Ok(())
    }
}

#[derive(Resource, Clone)]
enum Square {
    Wall,
    Block,
    Parabox(u32),
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Wall => write!(f, "#"),
            Square::Block => write!(f, "b"),
            // Square::Target(true) => write!(f, "="),
            // Square::Target(false) => write!(f, "_"),
            Square::Parabox(id) => write!(f, "{}", id),
        }
    }
}
