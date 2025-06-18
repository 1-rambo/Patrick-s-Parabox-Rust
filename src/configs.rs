use std::collections::HashMap;
use std::fs::File;
use std::fmt::Debug;
use serde_json::Value;

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
                            "Target" => {
                                let is_player = square["target_type"].as_str().expect("target_type should be a string") == "Player";
                                new_parabox.add_square(x, y, Square::Target(is_player))
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
            }
            if id == player_pos.0 as usize {
                new_parabox.player_pos = Some((player_pos.1 .0, player_pos.1 .1));
            }
            paraboxes.push(new_parabox);
        }
        LevelConfig {
            level,
            paraboxes,
            player_pos,
        }
    }
}

impl Debug for LevelConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LevelConfig {{ level: {}, player_pos: {:?}, paraboxes: {:?} }}", self.level, self.player_pos, self.paraboxes)
    }
}

pub struct Parabox {
    id: u32,
    outer: Option<Box<Parabox>>,
    map: HashMap<(u32, u32), Square>,
    player_pos: Option<(u32, u32)>, // (x, y)
    size: (u32, u32), // (width, height)
}

impl Parabox {
    fn new(id: u32, size: (u32, u32)) -> Self {
        Parabox {
            id,
            outer: None,
            map: HashMap::new(),
            player_pos: None,
            size,
        }
    }

    fn add_square(&mut self, x: u32, y: u32, square: Square) {
        self.map.insert((x, y), square);
    }
}

impl Debug for Parabox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parabox id: {}, size: {:?}", self.id, self.size)?;
        if let Some(outer) = &self.outer {
            write!(f, ", outer_id: {:?}", outer.id)?;
        } else {
            write!(f, ", outer: None")?;
        }
        writeln!(f, "")?;
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                if let Some(square) = self.map.get(&(x, y)) {
                    write!(f, "{:?}", square)?;
                }
                else if let Some((px, py)) = self.player_pos {
                    if px == x && py == y { write!(f, "p")?; } // Player position
                    else { write!(f, ".")?; } // Empty square
                }
                else {
                    write!(f, ".")?; // Empty square
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

enum Square {
    Wall,
    Block,
    Target(bool), // 0: box, 1: player
    Parabox(u32),
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Wall => write!(f, "#"),
            Square::Block => write!(f, "b"),
            Square::Target(true) => write!(f, "="),
            Square::Target(false) => write!(f, "_"),
            Square::Parabox(id) => write!(f, "{}", id),
        }
    }
}
