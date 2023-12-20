extern crate sdl2;
use sdl2::render::{Canvas, TextureCreator, Texture, TextureAccess};
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::video::{Window, WindowContext};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

const HEIGHT: u32 = 50;
const WIDTH: u32 = 50;
const SQ_SIZE: u32 = 16;

enum State {
    Paused,
    Playing
}

struct GameState {
    pub screen: [u32; (HEIGHT * WIDTH) as usize],
    pub state: State
}

impl GameState {
    pub fn init() -> Self {
        Self {
            screen: [0; (HEIGHT * WIDTH) as usize],
            state: State::Paused
        }
    }

    pub fn draw_point(self: &mut Self, x: i32, y: i32) {
        let x = (x as u32 / SQ_SIZE) as u32;
        let y = (y as u32 / SQ_SIZE) as u32;
        if x <= WIDTH && y <= HEIGHT {
            let val = &mut self.screen[(x + (y * WIDTH)) as usize];
            if *val == 0 {
                *val = 1;
            } else {
                *val = 0;
            }
        }
    }

    pub fn update(self: &mut Self) {
        if let State::Paused = self.state { return; }
        // Rules for game
        // any cell with < 2 neighbors dies
        // any cell with 2 or 3 neighbors lives
        // any cell with > 3 neighbors dies
        // any empty cell with 3 neighbors becomes alive
        let mut screen = [0; (HEIGHT * WIDTH) as usize];
        for (i, val) in self.screen.iter().enumerate() {
            let count = self.count_neighbors(i as u32);
            if count < 2 {
                screen[i] = 0;
            }
            if count == 3 {
                screen[i] = 1;
            }
            if count == 2 {
                screen[i] = *val;
            }
            if count > 3 {
                screen[i] = 0;
            }
        }
        self.screen = screen;
    }

    fn count_neighbors(self: &Self, i: u32) -> u32 {
        let mut count = 0;
        // Check row above
        if i >= WIDTH {
            count += self.screen[(i - WIDTH) as usize];
            if i % WIDTH != 0 && i > WIDTH {
                count += self.screen[((i - WIDTH) - 1) as usize];
            }
            if i % WIDTH != WIDTH - 1 {
                count += self.screen[((i - WIDTH) + 1) as usize];
            }
        }
        // Check left and right
        if i % WIDTH != 0 && i > 0 {
            count += self.screen[(i - 1) as usize];
        }
        if i % WIDTH != WIDTH - 1 && i + 1 < (HEIGHT * WIDTH) {
            count += self.screen[(i + 1) as usize];
        }

        // Check row below 
        if i + WIDTH < (HEIGHT * WIDTH) { 
            count += self.screen[(i + WIDTH) as usize];
            if i % WIDTH != WIDTH - 1 && (i + WIDTH + 1) < (HEIGHT * WIDTH) {
                count += self.screen[(i + WIDTH + 1) as usize];
            }
            if i % WIDTH != 0 {
                count += self.screen[(i + WIDTH - 1) as usize];
            }
        }

        count
    }
}

fn create_cell<'a>(creator: &'a TextureCreator<WindowContext>, canvas: &mut Canvas<Window>) -> Result<Texture<'a>, String> {
    if let Ok(mut texture) = creator.create_texture(None, TextureAccess::Target, SQ_SIZE, SQ_SIZE) {
        
        let _ = canvas.with_texture_canvas(&mut texture, |cnv| {

            cnv.set_draw_color(Color::RGB(0, 0, 0));
            for i in 1..SQ_SIZE-1 {
                for j in 1..SQ_SIZE-1 {
                    cnv.draw_point(Point::new(j as i32, i as i32)).expect("Failed to draw");
                }
            }
            cnv.set_draw_color(Color::RGB(128, 128, 128));
            for i in 0..SQ_SIZE {
                cnv.draw_point(Point::new(i as i32, (SQ_SIZE - 1) as i32)).expect("Failed to draw");
                cnv.draw_point(Point::new(i as i32, 0)).expect("Failed to draw");
                cnv.draw_point(Point::new(0, i as i32)).expect("Failed to draw");
                cnv.draw_point(Point::new((SQ_SIZE - 1) as i32, i as i32)).expect("Failed to draw");
            }
        });


        return Ok(texture);
    };

    Err("Unable to create texture".to_string())
}

fn create_grid<'a>(creator: &'a TextureCreator<WindowContext>, canvas: &mut Canvas<Window>) -> Result<Texture<'a>, String> {
    if let Ok(mut texture) = creator.create_texture(None, TextureAccess::Target, SQ_SIZE, SQ_SIZE) {
        
        let _ = canvas.with_texture_canvas(&mut texture, |cnv| {
            cnv.set_draw_color(Color::RGB(255, 255, 255));
            cnv.clear();

            cnv.set_draw_color(Color::RGB(128, 128, 128));
            for i in 0..SQ_SIZE {
                cnv.draw_point(Point::new(i as i32, (SQ_SIZE - 1) as i32)).expect("Failed to draw");
                cnv.draw_point(Point::new(i as i32, 0)).expect("Failed to draw");
                cnv.draw_point(Point::new(0, i as i32)).expect("Failed to draw");
                cnv.draw_point(Point::new((SQ_SIZE - 1) as i32, i as i32)).expect("Failed to draw");
            }
        });


        return Ok(texture);
    };

    Err("Unable to create texture".to_string())
}


fn main() {
    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("Game of Life", WIDTH * SQ_SIZE, HEIGHT * SQ_SIZE)
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let texture = create_grid(&texture_creator, &mut canvas).unwrap();
    let cell = create_cell(&texture_creator, &mut canvas).unwrap();
    let mut game = GameState::init();

    'main: loop {
        let start = Instant::now();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => break 'main,
                Event::MouseButtonDown { x, y, ..} => {
                    match game.state {
                        State::Paused => {game.draw_point(x, y)},
                        _ => {}
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    match game.state {
                        State::Paused => { game.state = State::Playing },
                        State::Playing => { game.state = State::Paused },
                    }
                },
                _ => {},
            }
        }

        for (i, point) in game.screen.iter().enumerate() {
            if *point == 1 {
                canvas.copy(
                    &cell,
                    None,
                    Rect::new(
                        ((i as u32 % WIDTH) * SQ_SIZE) as i32,
                        ((i as u32 / WIDTH) * SQ_SIZE) as i32,
                        SQ_SIZE,
                        SQ_SIZE,
                    ),
                ).unwrap();
            } else {
                canvas.copy(
                    &texture, 
                    None, 
                    Rect::new(
                        ((i as u32 % WIDTH) * SQ_SIZE) as i32,
                        ((i as u32 / WIDTH) * SQ_SIZE) as i32,
                        SQ_SIZE,
                        SQ_SIZE,
                    ),
                ).unwrap();
            }
        }
        if let State::Playing = game.state {
            game.update();
            while start.elapsed().as_millis() < 300 {};
        }
        canvas.present();
    }
}
