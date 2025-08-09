use std::iter::Zip;

use sfml::{cpp::FBox, graphics::{Color, Drawable, RcTexture, Rect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable}, system::Vector2i, window::{ContextSettings, Event, Key, Style}, SfResult};

fn process_window_events(window: &mut FBox<RenderWindow>) -> () {
    while let Some(ev) = window.poll_event() {
        match ev {
            Event::Closed => window.close(),
            _ => {}
        }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Air,
    Grass,
    Plant(bool),
}

impl Tile {
    pub const WIDTH_PX: u32 = 32;
    pub const HEIGHT_PX: u32 = 32;
    pub const ATLAS_LINE_TILE_COUNT: u32 = 16;
    pub fn tile_id(&self) -> i32 {
        match &self {
            Self::Air => 0,
            Self::Grass => 1,
            Self::Plant(false) => 2,
            Self::Plant(true) => 3
        }
    }
    pub fn texture_rect(&self) -> Option<Rect<i32>> {
        match &self {
            Self::Air => None,
            _ => {
                let a = self.tile_id();
                Some(Rect::new(
                    Self::WIDTH_PX as i32 * (a % Self::ATLAS_LINE_TILE_COUNT as i32),
                    Self::HEIGHT_PX as i32 * (a / Self::ATLAS_LINE_TILE_COUNT as i32),
                    Self::WIDTH_PX as i32,
                    Self::HEIGHT_PX as i32
                ))
            }
        }
    }
}

struct World {
    tiles: Vec<Tile>,
    size_x: usize,
    size_y: usize,
    size_z: usize,
}

impl World {
    fn get_index(&self, x: i64, y: i64, z: i64) -> Option<usize> {
        if x < 0 || x >= self.size_x as i64 || y < 0 || y >= self.size_y as i64 || z < 0 || z >= self.size_z  as i64 {
            return None;
        }
        Some(x as usize + (y as usize * self.size_x) + (z as usize * self.size_x * self.size_y))
    }

    pub fn set_tile(&mut self, x: i64, y: i64, z: i64, tile: Tile) {
        if let Some(index) = self.get_index(x, y, z) {
            self.tiles[index] = tile;
        }
    }

    pub fn get_tile(&self, x: i64, y: i64, z: i64) -> Option<Tile> {
        self.get_index(x, y, z).map(|index| self.tiles[index])
    }
    
    pub fn new_flat(size_x: usize, size_y: usize, size_z: usize) -> Self {
        let tiles = vec![Tile::Air; size_x * size_y * size_z];
        
        let mut world = Self {tiles, size_x, size_y, size_z};
        for x in 0..size_x as i64 {
            for z in 0..size_z as i64{
                world.set_tile(x, 0, z, Tile::Grass);
            }
        }
        world
    }
}

fn tile_coords_to_px(x: i64, y: i64, z: i64) -> Vector2i {
    const TILE_SIZE: i64 = 32;
    const QUARTER_UNIT: i64 = TILE_SIZE / 4;
    Vector2i::new(
        ((x - z) * (QUARTER_UNIT * 2)) as i32,
        ((x + z - y - y) * QUARTER_UNIT) as i32
    )
}

fn draw_tile_at_px(tile: Tile, x: i64, y: i64, window: &mut FBox<RenderWindow>, tex_terrain: &RcTexture) {
    if let Some(texture_rect) = tile.texture_rect() {
        let mut r = RectangleShape::new();
        r.set_position((x as f32, y as f32));
        r.set_size((32.0, 32.0));
        r.set_fill_color(Color::WHITE);
        // println!("texture_rect: {texture_rect:#?}");
        r.set_texture_rect(texture_rect);
        r.set_texture(tex_terrain.raw_texture(), false);
        window.draw(&r);
    }
}

fn draw_tile_at_grid(tile: Tile, x: i64, y: i64, z: i64, window: &mut FBox<RenderWindow>, tex_terrain: &RcTexture) {
    let px = tile_coords_to_px(x, y, z);
    draw_tile_at_px(tile, px.x as i64, px.y as i64, window, tex_terrain);
}

fn draw_window(window: &mut FBox<RenderWindow>, world: &World, tex_terrain: &RcTexture) -> () {
    window.clear(Color::BLACK);

    for y in 0..world.size_y as i64 {
        for x in 0..world.size_x as i64 {
            for z in 0..world.size_z as i64 {
                let tile = world.get_tile(x, y, z).unwrap();
                draw_tile_at_grid(tile, x, y, z, window, tex_terrain);
            }
        }
    }

    // draw_tile_at_grid(Tile::Grass, 0, 0, 0, window, tex_terrain);
    // draw_tile_at_grid(Tile::Grass, 1, 0, 0, window, tex_terrain);
    // draw_tile_at_grid(Tile::Grass, 2, 0, 0, window, tex_terrain);
    // draw_tile_at_grid(Tile::Grass, 3, 0, 0, window, tex_terrain);

    // draw_tile_at_grid(Tile::Grass, 4, 0, 0, window, tex_terrain);
    // draw_tile_at_grid(Tile::Grass, 4, 0, 1, window, tex_terrain);
    // draw_tile_at_grid(Tile::Grass, 4, 0,2, window, tex_terrain);
    // draw_tile_at_grid(Tile::Grass, 4, 0,3, window, tex_terrain);

    // draw_tile_at_grid(Tile::Grass, 4, 1, 0, window, tex_terrain);
    // draw_tile_at_grid(Tile::Grass, 4,-2, 1, window, tex_terrain);
    // draw_tile_at_grid(Tile::Grass, 4, -3,2, window, tex_terrain);
    // draw_tile_at_grid(Tile::Grass, 4, -4,3, window, tex_terrain);
    
    window.display();
}

fn main() -> SfResult<()> {
    let tex_terrain = RcTexture::from_file("res/terrain.png")?;
    let mut window = RenderWindow::new((640, 480), "gardening", Style::DEFAULT, 
        &ContextSettings::default())?;
    window.set_vertical_sync_enabled(true);
    let mut world = World::new_flat(5, 2, 10);
    world.set_tile(4, 1, 2, Tile::Plant(true));

    while window.is_open() {
        process_window_events(&mut window);
        draw_window(&mut window, &world, &tex_terrain);
    }

    Ok(())
}
