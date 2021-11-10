use std::cmp::{max, min};

use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, RGB, Rltk};
use serde::{Deserialize, Serialize};
use specs::prelude::*;

use {crate::HEIGHT, crate::WIDTH};

use super::Rect;
use std::collections::HashSet;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub bloodstains : HashSet<usize>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - w) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + w) + 1, 1.45));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut x = 0;
    let mut y = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            let mut bg = RGB::from_f32(0., 0., 0.);
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.5, 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = wall_glyph(&*map, x, y);
                    fg = RGB::from_f32(0., 1.0, 0.);
                }
                TileType::DownStairs => {
                    glyph = rltk::to_cp437('>');
                    fg = RGB::from_f32(0., 1.0, 1.0);
                }
            }
            if map.bloodstains.contains(&idx) { bg = RGB::from_f32(0.75, 0., 0.); }
            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale();
                bg = RGB::from_f32(0., 0., 0.); // Don't show stains out of visual range
            }
            ctx.set(x, y, fg, bg, glyph);
        }

        x += 1;
        if x > (WIDTH - 1) as i32 {
            x = 0;
            y += 1;
        }
    }
}

fn wall_glyph(map: &Map, x: i32, y: i32) -> rltk::FontCharType {
    if x < 1 || x > map.width - 2 || y < 1 || y > map.height - 2 as i32 {
        return 35;
    }
    let mut mask: u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) {
        mask += 1;
    }
    if is_revealed_and_wall(map, x, y + 1) {
        mask += 2;
    }
    if is_revealed_and_wall(map, x - 1, y) {
        mask += 4;
    }
    if is_revealed_and_wall(map, x + 1, y) {
        mask += 8;
    }

    match mask {
        0 => 9,    // Pillar because we can't see neighbors
        1 => 186,  // Wall only to the north
        2 => 186,  // Wall only to the south
        3 => 186,  // Wall to the north and south
        4 => 205,  // Wall only to the west
        5 => 188,  // Wall to the north and west
        6 => 187,  // Wall to the south and west
        7 => 185,  // Wall to the north, south and west
        8 => 205,  // Wall only to the east
        9 => 200,  // Wall to the north and east
        10 => 201, // Wall to the south and east
        11 => 204, // Wall to the north, south and east
        12 => 205, // Wall to the east and west
        13 => 202, // Wall to the east, west, and south
        14 => 203, // Wall to the east, west, and north
        15 => 206, // ╬ Wall on all sides
        _ => 35,   // We missed one?
    }
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idxi32(x, y);
    map.tiles[idx] == TileType::Wall && map.revealed_tiles[idx]
}

impl Map {
    pub fn idx_to_xy(idx: usize) -> (usize, usize) {
        let x = idx % WIDTH;
        let y = idx / WIDTH;
        (x, y)
    }
    pub fn xy_idx_impl(x: usize, y: usize) -> usize {
        (y * WIDTH) + x
    }

    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        Map::xy_idx_impl(x, y)
    }

    pub fn xy_idxi32(&self, x: i32, y: i32) -> usize {
        self::Map::xy_idx(self, x as usize, y as usize)
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self::Map::xy_idxi32(self, x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idxi32(x, y);
        !self.blocked[idx]
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self::Map::xy_idxi32(self, x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self::Map::xy_idxi32(self, x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    /// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
    /// This gives a handful of random rooms and corridors joining them together.
    pub fn new_map_rooms_and_corridors(depth: i32) -> Map {
        let mut map = Map::new_empty_map(depth);

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        let stairs_position = map.rooms[map.rooms.len() - 1].center();
        let stairs_idx = map.xy_idxi32(stairs_position.0, stairs_position.1);
        map.tiles[stairs_idx] = TileType::DownStairs;

        map
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    fn new_empty_map(depth: i32) -> Map {
        Map {
            tiles: vec![TileType::Wall; HEIGHT * WIDTH],
            rooms: Vec::new(),
            revealed_tiles: vec![false; HEIGHT * WIDTH],
            visible_tiles: vec![false; HEIGHT * WIDTH],
            blocked: vec![false; HEIGHT * WIDTH],
            tile_content: vec![Vec::new(); HEIGHT * WIDTH],
            width: WIDTH as i32,
            height: HEIGHT as i32,
            bloodstains: HashSet::new(),
            depth,
        }
    }

    pub fn new_rand_map(&mut self, depth: i32) -> Map {
        let mut map = Map::new_empty_map(depth);
        for x in 0..WIDTH {
            map.tiles[self.xy_idx(x, 0)] = TileType::Wall;
            map.tiles[self.xy_idx(x, HEIGHT - 1)] = TileType::Wall;
        }
        for y in 0..HEIGHT {
            map.tiles[self.xy_idx(0, y)] = TileType::Wall;
            map.tiles[self.xy_idx(WIDTH - 1, y)] = TileType::Wall;
        }

        let mut rng = rltk::RandomNumberGenerator::new();
        for _i in 0..400 {
            let x = rng.roll_dice(1, 79) as usize;
            let y = rng.roll_dice(1, 49) as usize;
            let idx = self.xy_idx(x, y);
            if idx != self.xy_idx(40, 25) {
                map.tiles[idx] = TileType::Wall;
            }
        }

        map
    }
}
