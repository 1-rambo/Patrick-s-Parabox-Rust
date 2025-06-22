use std::collections::HashMap;
use std::fs::File;
use std::fmt::Debug;
use serde_json::Value;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct LevelConfig {
    level: i32,
    pub paraboxes: Vec<Parabox>,
    player_pos: (i32, (i32, i32)), // (box_id, (x, y))
}

impl LevelConfig {
    pub fn new(level: i32, file: &str) -> Self {
        let file = File::open(file).expect("Failed to open level config file");
        let data: Value = serde_json::from_reader(file).expect("Failed to parse level config file");
        let player_pos = (
            data["player_pos"][0].as_u64().expect("player_pos x should be a number") as i32,
            (
                data["player_pos"][1].as_u64().expect("player_pos y should be a number") as i32,
                data["player_pos"][2].as_u64().expect("player_pos z should be a number") as i32,
            ),
        );
        let mut paraboxes: Vec<Parabox> = Vec::new();
        for (id, parabox) in data["paraboxes"].as_array().expect("paraboxes should be an array").iter().enumerate() {
            let (size_x, size_y) = (
                parabox["size"][0].as_u64().expect("parabox size width should be a number") as i32,
                parabox["size"][1].as_u64().expect("parabox size height should be a number") as i32,
            );
            let mut new_parabox = Parabox::new(id as i32, (size_x, size_y));
            if parabox["outer"].is_null() {
                new_parabox.outer = None;
            } else {
                let outer_id = parabox["outer"].as_i64().expect("parabox outer id should be a number") as i32;
                new_parabox.outer = Some(outer_id);
            }
            let map = parabox["map"].as_object().expect("parabox map should be an object");
            for wall_pos in map["walls"].as_array().expect("walls should be an array") {
                
                let x = wall_pos[0].as_u64().expect("wall x should be a number") as i32;
                let y = wall_pos[1].as_u64().expect("wall y should be a number") as i32;
                new_parabox.add_square((x, y), Square::Wall);
            }
            for x in 0..size_x {
                for y in 0..size_y {
                    if let Some(square) = map.get(&format!("({}, {})", x, y)) {
                        println!("pos: ({}, {}), square: {:?}", x, y, square);
                        match square["type"].as_str().expect("square should be a string") {
                            "Block" => new_parabox.add_square((x, y), Square::Block),
                            "Parabox" => {
                                let id = square["id"].as_i64().expect("parabox id should be a number") as i32;
                                new_parabox.add_square((x, y), Square::Parabox(id));
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
            }
            let targets = parabox["targets"].as_array().expect("targets should be an array");
            for target in targets {
                let x = target[0].as_u64().expect("target x should be a number") as i32;
                let y = target[1].as_u64().expect("target y should be a number") as i32;
                new_parabox.add_target(x, y);
            }
            if id == player_pos.0 as usize {
                new_parabox.set_player_pos(Some(player_pos.1));
            }
            if !parabox["player_target"].is_null() {
                let target = parabox["player_target"].as_array().expect("player_target should be an array");
                new_parabox.set_player_target(
                    target[0].as_u64().expect("player_target x should be a number") as i32,
                    target[1].as_u64().expect("player_target y should be a number") as i32,
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

    pub fn load(&mut self, level: i32) {
        let file = format!("assets/levels/{}.json", level);
        let new_level = LevelConfig::new(level, &file);
        self.level = new_level.level;
        self.paraboxes = new_level.paraboxes;
        self.player_pos = new_level.player_pos;
    }

    pub fn shift(&mut self, square: Option<Square>, ori_pos: Option<(i32, (i32, i32))>, dir: (i32, i32)) -> bool {
        // Detemine if shift is valid
        assert!((dir.0 == 0 || dir.1 == 0), "Invalid shift: {:?}", dir);
        assert!((dir.0.abs() <= 1 && dir.1.abs() <= 1), "Shift too large: {:?}", dir);

        // Start from the parabox that contains the player
        let ori_pos = ori_pos.unwrap_or(self.player_pos);
        println!("Attempting to shift square: {:?} from position: {:?}, in direction: {:?}", square, ori_pos, dir);
        if let Some(parabox) = self.paraboxes.get_mut(ori_pos.0 as usize) {
            // Check for wall/empty along the shift direction
            let ori_id = parabox.id;
            let mut new_pos = (
                (ori_pos.1 .0 as i32 + dir.0),
                (ori_pos.1 .1 as i32 + dir.1),
            );
            // check for empty (including outer)
            let mut cur_parabox = if parabox.check_inbounds(new_pos) {
                parabox.clone()
            } else {
                let mut outer_parabox = parabox.clone();
                while !outer_parabox.check_inbounds(new_pos) {
                    let new_outer_parabox = self.paraboxes[outer_parabox.outer.unwrap() as usize].clone();
                    new_pos = new_outer_parabox.find_box(outer_parabox.id);
                    new_pos = (
                        new_pos.0 + dir.0,
                        new_pos.1 + dir.1,
                    );
                    outer_parabox = new_outer_parabox;
                }
                outer_parabox
            };
            // cur_parabox: the new parabox;
            // new_pos: the new position in the parabox
            println!("new_pos: {:?}", new_pos);
            let mut path_blocks = Vec::new();
            while let v @ Some(Square::Block | Square::Parabox(_)) = cur_parabox.find_at(new_pos.0 as i32, new_pos.1 as i32) {
                println!("Found block at new_pos: {:?}, square: {:?}", new_pos, v);
                path_blocks.push((v.unwrap().clone(), cur_parabox.id, new_pos));
                new_pos = (
                    new_pos.0 + dir.0,
                    new_pos.1 + dir.1,
                );
                while !cur_parabox.check_inbounds(new_pos) {
                    let new_outer_parabox = self.paraboxes[cur_parabox.outer.unwrap() as usize].clone();
                    new_pos = new_outer_parabox.find_box(cur_parabox.id);
                    new_pos = (
                        new_pos.0 + dir.0,
                        new_pos.1 + dir.1,
                    );
                    cur_parabox = new_outer_parabox;
                }
            }

            println!("path_blocks: {:?}", path_blocks);
            // println!("Current position: {:?}", self.player_pos.1);
            // println!("Attempting to move to: {:?}", new_pos);
            // if new_pos.0 < 0 || new_pos.1 < 0 ||
            //     new_pos.0 >= parabox.size.0 as i32 || new_pos.1 >= parabox.size.1 as i32 {
            //     unreachable!("New position out of bounds: {:?}", new_pos);
            // }
            if let Some(Square::Wall) = cur_parabox.find_at(new_pos.0 as i32, new_pos.1 as i32) {
                // TODO: try_enter
                println!("New position is a wall: {:?}, trying enter now", new_pos);
                if path_blocks.is_empty() { return false; }
                let mut successful = false;
                for ((block, box_id, pos), (next_block, _, _)) in path_blocks.iter().rev().zip(path_blocks.iter().rev().skip(1)) {
                    if !successful {
                        if let Square::Parabox(id) = block {
                            if self.shift(Some(next_block.clone()), Some((*id, self.paraboxes[*id as usize].enter_from(dir))), dir) { 
                                successful = true;
                            }
                        }
                    } else {
                        self.paraboxes[*box_id as usize].remove_square(*pos);
                        self.paraboxes[*box_id as usize].add_square(*pos, next_block.clone());
                    }
                }
                if !successful {
                    if let Square::Parabox(id) = path_blocks[0].0 {
                        if self.shift(square.clone(), Some((id, self.paraboxes[id as usize].enter_from(dir))), dir) {
                            successful = true;
                            if let Some(_) = square {
                                self.paraboxes[ori_pos.0 as usize].remove_square(ori_pos.1);
                            } else {
                                self.paraboxes[ori_pos.0 as usize].set_player_pos(None);
                            }
                        }
                    }
                }
                else {
                    let (_, box_id, pos) = path_blocks[0];
                    self.paraboxes[box_id as usize].remove_square(pos);
                    if let Some(square) = square {
                        self.paraboxes[box_id as usize].add_square(pos, square);
                    } else {
                        // If no square is provided, just move the player
                        self.player_pos = (box_id, pos);
                        self.paraboxes[ori_id as usize].set_player_pos(None);
                        self.paraboxes[box_id as usize].set_player_pos(Some(pos));
                    }
                }
                return successful;
            }
            else {
                // move backwards through the path_blocks
                let mut dest = (cur_parabox.id, new_pos); 
                println!("Final destination: {:?}", dest);
                for (block, box_id, pos) in path_blocks.iter().rev() {
                    println!("removing block: {:?} from parabox: {} at pos: {:?}", block, box_id, pos);
                    self.paraboxes[*box_id as usize].remove_square(*pos);
                    println!("adding block: {:?} to parabox: {} at pos: {:?}", block, dest.0, dest.1);
                    self.paraboxes[dest.0 as usize].add_square(dest.1, block.clone());
                    dest = (*box_id, *pos);
                }
                if let Some(square) = square {
                    self.paraboxes[dest.0 as usize].add_square(dest.1, square);
                } else {
                    // If no square is provided, just move the player
                    self.player_pos = dest;
                    self.paraboxes[ori_id as usize].set_player_pos(None);
                    self.paraboxes[dest.0 as usize].set_player_pos(Some(dest.1));
                }
            }
        } else {
            panic!("Parabox with id {} not found", self.player_pos.0);
        }

        // self.check_win()
        true
    }

    pub fn check_win(&self) -> bool {
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
    id: i32,
    // outer: Option<Box<Parabox>>,
    outer: Option<i32>,
    map: HashMap<(i32, i32), Square>,
    player_pos: Option<(i32, i32)>, // (x, y)
    player_target: Option<(i32, i32)>, // (x, y)
    size: (i32, i32), // (width, height)
    targets: Vec<(i32, i32)>, // List of target positions
}

impl Parabox {
    fn new(id: i32, size: (i32, i32)) -> Self {
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

    fn add_square(&mut self, pos: (i32, i32), square: Square) {
        self.map.insert(pos, square);
    }

    fn remove_square(&mut self, pos: (i32, i32)) {
        self.map.remove(&pos);
    }

    fn set_player_pos(&mut self, pos: Option<(i32, i32)>) {
        self.player_pos = pos;
    }

    fn set_player_target(&mut self, x: i32, y: i32) {
        self.player_target = Some((x, y));
    }

    fn add_target(&mut self, x: i32, y: i32) {
        self.targets.push((x, y));
    }

    fn find_at(&self, x: i32, y: i32) -> Option<&Square> {
        self.map.get(&(x, y))
    }

    fn find_box(&self, id: i32) -> (i32, i32) {
        for ((x, y), square) in &self.map {
            if let Square::Parabox(box_id) = square {
                if *box_id == id {
                    return (*x, *y);
                }
            }
        }
        panic!("Parabox with id {} not found in Parabox {}", id, self.id);
    }

    fn check_inbounds(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && y >= 0 && x < self.size.0 && y < self.size.1
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

    fn enter_from(&self, dir: (i32, i32)) -> (i32, i32) {
        if let (1, 0) = dir {
            // Enter from the top
            (-1, self.size.1 / 2)
        }
        else if let (-1, 0) = dir {
            // Enter from the bottom
            (self.size.0, self.size.1 / 2)
        }
        else if let (0, 1) = dir {
            // Enter from the left
            (self.size.0 / 2, -1)
        }
        else if let (0, -1) = dir {
            // Enter from the right
            (self.size.0 / 2, self.size.1)
        }
        else {
            panic!("Invalid direction for entering parabox: {:?}", dir);
        }
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
pub enum Square {
    Wall,
    Block,
    Parabox(i32),
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Wall => write!(f, "#"),
            Square::Block => write!(f, "b"),
            // Square::Target(true) => write!(f, "="),
            // Square::Target(false) => write!(f, "_"),
            Square::Parabox(id) => write!(f, "{}", id + 1),
        }
    }
}
