use std::cmp::{max, min};

use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, RGB, Rltk};
use specs::prelude::*;

use {crate::HEIGHT, crate::WIDTH};

use super::{Player, Position, Viewshed};
use super::Rect;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
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
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub width: i32,
    pub height: i32,
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut x = 0;
    let mut y = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {

            let glyph;
            let mut fg;
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.5, 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_u8(87, 134, 112);
                }
            }
            if !map.visible_tiles[idx] { fg = fg.to_greyscale() }
            ctx.set(x, y, fg, RGB::from_f32(0.,0.,0.), glyph);
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

impl Map {
    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        (y * WIDTH) + x
    }

    pub fn xy_idxi32(&self, x: i32, y: i32) -> usize {
        self::Map::xy_idx(self, x as usize, y as usize)
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self::Map::xy_idxi32(self, x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
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
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map::new_empty_map();

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

        map
    }

    fn new_empty_map() -> Map {
        Map {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            revealed_tiles: vec![false; HEIGHT * WIDTH],
            visible_tiles: vec![false; HEIGHT * WIDTH],
            width: 80,
            height: 50,
        }
    }

    pub fn new_rand_map(&mut self) -> Map {
        let mut map = Map::new_empty_map();
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
