use sdl2;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(height: u32, width: u32) -> Self {
        let cells = vec![Cell::Dead; (width * height) as usize];
        Self {
            height,
            width,
            cells,
        }
    }
}

pub struct Engine {
    universe: Universe,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl Engine {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let mut window = match video_subsystem
            .window("SDL Game of Life", 1000, 1000)
            .position_centered()
            .build()
        {
            Ok(sub_system) => sub_system,
            Err(e) => return Err(format!("Could not build window: {:?}", e)),
        };

        window.set_fullscreen(sdl2::video::FullscreenType::Desktop)?;
        let canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(e) => return Err(format!("Could not convert window into canvas: {:?}", e)),
        };

        Ok(Self {
            universe: Universe::new(320, 480),
            canvas,
            event_pump,
        })
    }
}
fn main() {
    println!("Hello, world!");
}
