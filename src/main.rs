use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use std::time::Instant;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    running: bool,
    x_offset: i32,
    y_offset: i32,
    scale: f32,
    // Spacing between cells in pixels
    spacing: u32,
    // Leg size of a cell square
    leg_size: u32,

}
impl Universe {
    
    /// create a new universe populated with dead cells that is height x width big
    /// 
    /// # Arguments
    /// 
    /// * `height` - An unsigned 32 bit int representing the height of the universe
    /// * `width` - An unsigned 32 bit int representing the width of the universe
    /// ```
    /// use sdl_game_of_life::Universe;
    /// let universe = Universe::new(64, 64);
    /// ```
    pub fn new(height: u32, width: u32) -> Universe  {
        let cells = vec![Cell::Dead; (width * height) as usize];
        
        Universe{
            height,
            width,
            cells,
            running: false,
            x_offset: 0,
            y_offset: 0,
            scale: 1.0,
            leg_size: 10,
            spacing: 1,
        }
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// get the number of live neighbors
    fn get_live_neighbors(&self, row: u32, col: u32) -> u8 {
        let mut live_count = 0;
        
        for row_modifier in [self.height - 1, 0, 1].iter().cloned() {
            for col_modifier in [ self.width - 1, 0, 1].iter().cloned() {
                if row_modifier == 0 && col_modifier == 0 {
                    continue;
                }

                let neighbor_row = (row + row_modifier) % self.height;
                let neighbor_col = (col + col_modifier) % self.width;
                let index = self.get_index(neighbor_row, neighbor_col);
                live_count += self.cells[index] as u8; // increment if alive, because alive = 1

            }
        }

        live_count
    } 

    /// Moves the state of the game by one tick
    pub fn tick(& mut self) {

        match self.running {
            false => return,
            true => {
                let mut next = self.cells.clone();
        
                for row in 0..self.height {
                    for col in 0..self.width {
                        let index = self.get_index(row, col);
                        let live_neighbors = self.get_live_neighbors(row, col);
        
                        next[index] = match (live_neighbors, self.cells[index]){
                            // if neighbors are less than two, then cell dies
                            (x, Cell::Alive) if x < 2 => Cell::Dead,
                            // if neighbors more than tree, then cell dies
                            (x, Cell::Alive) if x > 3 => Cell::Dead,
                            // if neighbors 2 or 3, then cell stays alive
                            (2, Cell::Alive) | (3, Cell::Alive) => Cell::Alive,
                            // if neighbors exactly 3, then revive
                            (3, Cell::Dead) => Cell::Alive,
                            // stay the same for other states
                            (_, otherwise) => otherwise,
                        };
                    }
                }
                
                self.cells = next;
            }
        }

    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {

        let mut current_y = 0 + self.y_offset;
        // currently the size of the cell is 10x10 pixels with 2 pixel border
        for row in self.cells.as_slice().chunks(self.width as usize) {
            let mut current_x = 0 + self.x_offset;
            for &cell in row {
                if cell == Cell::Alive {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                let leg_size = (self.leg_size as f32 * self.scale).floor() as u32;
                
                canvas.fill_rect(Rect::new(current_x, current_y, leg_size, leg_size)).unwrap();


                current_x += ((self.leg_size + self.spacing * 2) as f32 * self.scale) as i32;
            }

            current_y += ((self.leg_size + self.spacing * 2) as f32 * self.scale) as i32;
        }
    }
    
    pub fn toggle_state(&mut self) {
        self.running ^= true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn run(&mut self) {
        self.running = true;
    }

    pub fn shift(&mut self, x: i32, y: i32) {
        self.x_offset += x;
        self.y_offset += y;
    }

    fn get_by_coordinates(&self, x: i32, y: i32) -> Option<usize> {
        // TODO use dynamic cell size to get coordinates when scaling
        let x_size = ((self.leg_size + self.spacing * 2) as f32 * self.scale) as i32;
        let y_size = ((self.leg_size + self.spacing * 2) as f32 * self.scale) as i32;
        // Take the cell size and spacing, multiply by it's index
        
        let y_index = (((y - self.y_offset) as f32) / (y_size as f32)).floor();
        let x_index = (((x - self.x_offset) as f32) / (x_size as f32)).floor();

        if y_index < 0.0 || x_index < 0.0 || y_index >= self.height as f32 || x_index >= self.width as f32 {
            return None;
        }

        Some((y_index as u32 * self.width + x_index as u32) as usize)

    }

    pub fn kill(&mut self, x: i32, y: i32) {
        let cell_index = match self.get_by_coordinates(x, y) {
            Some(index) => index,
            None => return
        };
        self.cells[cell_index] = Cell::Dead;
    }

    pub fn revive(&mut self, x: i32, y: i32) {
        let cell_index = match self.get_by_coordinates(x, y) {
            Some(index) => index,
            None => return
        };
        self.cells[cell_index] = Cell::Alive
    }

    pub fn increment_scale(&mut self, increment: f32) {
        self.scale += increment;
    }

    pub fn reset(&mut self) {
        self.cells = vec![Cell::Dead; (self.width * self.height) as usize];
    }
}
pub struct Engine {
    universe: Universe,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl Engine {
    pub fn new() -> Result<Engine, String> {
        let universe = Universe::new(64, 115);

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let mut window = match video_subsystem.window("SDL Game of Life", 1000, 1000)
                .position_centered()
                .build() {
            Ok(sub_system) => sub_system,
            Err(e) => return Err(format!("Could not build window: {:?}", e))
        };

        window.set_fullscreen(sdl2::video::FullscreenType::Desktop)?;

        let canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(e) => return Err(format!("Could not convert window into canvas: {:?}", e))
        };

        let event_pump = sdl_context.event_pump().unwrap();

        Ok(Engine {
            canvas,
            universe,
            event_pump
        })
    }

    // Starts the game loop
    pub fn run(&mut self) {
        self.canvas.set_draw_color(Color::RGB(120, 120, 120));
        self.canvas.clear();
        self.canvas.present();

        let mut mouse_dragging = false;
        let mut mouse_setting = false;
        let mut mouse_clearing = false;

        let mut previous_mouse_pos_x: i32 = 0;
        let mut previous_mouse_pos_y: i32 = 0;

        let mut render_timer = Instant::now();
        let mut tick_timer = Instant::now();

        'running: loop {
            self.canvas.set_draw_color(Color::RGB(120, 120, 120));
            self.canvas.clear();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running
                    }
                    Event::KeyDown {keycode, ..} => {
                        match keycode {
                            Some(Keycode::Space) => self.universe.toggle_state(),
                            Some(Keycode::Right) => {
                                self.universe.run();
                                self.universe.tick();
                                self.universe.pause();
                            }
                            Some(Keycode::Escape) => {
                                break 'running
                            }
                            Some(Keycode::R) => {
                                self.universe.reset();
                            }
                            _ => {}
                        }
                    },
                    // Enable dragging, cell revive mode, and cell kill mode.
                    Event::MouseButtonDown {mouse_btn, x, y, ..} => {
                        match mouse_btn {
                            MouseButton::Left => {
                                mouse_setting = true;
                                self.universe.revive(x, y);

                            },
                            MouseButton::Right => {
                                mouse_clearing = true;
                                self.universe.kill(x, y);
                            },
                            MouseButton::Middle => mouse_dragging = true,
                            _ => {}
                        };
                        previous_mouse_pos_x = x;
                        previous_mouse_pos_y = y;
                    },
                    // Disable dragging, cell revive mode, and cell kill mode.
                    Event::MouseButtonUp {mouse_btn, ..} => {
                        match mouse_btn {
                            MouseButton::Left => mouse_setting = false,
                            MouseButton::Right => mouse_clearing = false,
                            MouseButton::Middle => mouse_dragging = false,
                            _ => {}
                        };
                    },
                    // Scale the board with scroll wheel
                    Event::MouseWheel {y, ..} => {
                        match y {
                            1 => self.universe.increment_scale(0.1),
                            -1 => self.universe.increment_scale(-0.1),
                            _ => {}
                        };
                    },

                    // Apply motion event like dragging, cell batch revive, cell batch kill.
                    Event::MouseMotion {x, y, ..} => {
                        if mouse_dragging {
                            let x_dif = x - previous_mouse_pos_x;
                            let y_dif = y - previous_mouse_pos_y;
    
                            self.universe.shift(x_dif, y_dif);

                            previous_mouse_pos_x = x;
                            previous_mouse_pos_y = y;
                        } else if mouse_setting {
                            self.universe.revive(x, y);
                        } else if mouse_clearing {
                            self.universe.kill(x, y);
                        }
                    },
                    _ => {
                    }
                }
            }

            if tick_timer.elapsed().as_millis() >= 100 {
                self.universe.tick();
                tick_timer = Instant::now();
            }

            if render_timer.elapsed().as_millis() >= 8  {
                self.universe.render(&mut self.canvas);
                self.canvas.present();
                render_timer = Instant::now();
            }
            
        }
    }
}

fn main() {
    let mut engine = match Engine::new() {
        Ok(engine) => engine,
        Err(error) => panic!("Engine Failed: {:?}", error)
    };
    engine.run();
}
