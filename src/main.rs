extern crate sdl2;
use sdl2::render::TextureCreator;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
mod texture;

pub const HEIGHT: u32 = 50;
pub const WIDTH: u32 = 50;
pub const SQ_SIZE: u32 = 16;

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

    pub fn clear(self: &mut Self) {
        for data in &mut self.screen {
            *data = 0;
        }
    }

    pub fn save(self: &mut Self) {
        if let Ok(mut file) = File::create("save.dat") {
            for data in self.screen {
                if let Err(_) = file.write(&data.to_be_bytes()) {
                    panic!("Failed to save game state");
                }
            }
        }
    }

    pub fn load(self: &mut Self) {
        if let Ok(mut file) = File::open("save.dat") {
            for i in 0..self.screen.len() {
                let mut buf: [u8; 4] = [0; 4];
                if let Err(_) = file.read_exact(&mut buf) {
                    panic!("Failed to read from file");
                }
                let data = u32::from_be_bytes(buf);
                self.screen[i] = data;
            }
        }
    }

    pub fn draw_point(self: &mut Self, x: i32, y: i32, erase: bool) {
        let x = (x as u32 / SQ_SIZE) as u32;
        let y = (y as u32 / SQ_SIZE) as u32;
        if x <= WIDTH && y <= HEIGHT {
            let val = &mut self.screen[(x + (y * WIDTH)) as usize];
            if !erase {
                if *val == 0 || *val == 2 {
                    *val = 1;
                }
            } else {
                *val = 0;
            }
        }
    }

    pub fn hover(self: &mut Self, x: i32, y: i32) {
        let x = (x as u32 / SQ_SIZE) as u32;
        let y = (y as u32 / SQ_SIZE) as u32;
        if x <= WIDTH && y <= HEIGHT {
            let ind: usize = (x + (y * WIDTH)) as usize;
            for (_, val) in self.screen.iter_mut().enumerate() {
                if *val == 2 {
                    *val = 0;
                }
            }
            let val = self.screen[ind];
            if val == 0 {
                self.screen[ind] = 2;
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
    let grid = texture::create_cell(&texture_creator, &mut canvas, SQ_SIZE, Color::RGB(255, 255, 255)).unwrap();
    let cell = texture::create_cell(&texture_creator, &mut canvas, SQ_SIZE, Color::RGB(0, 0, 0)).unwrap();
    let hover = texture::create_cell(&texture_creator, &mut canvas, SQ_SIZE, Color::RGB(128, 128, 128)).unwrap();
    let mut game = GameState::init();

    'main: loop {
        let start = Instant::now();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => break 'main,
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    match game.state {
                        State::Paused => {
                            match mouse_btn {
                                MouseButton::Left => {game.draw_point(x, y, false)},
                                MouseButton::Right => {game.draw_point(x, y, true)},
                                _ => {},
                            }
                        },
                        _ => {}
                    }
                },
                Event::MouseMotion { x, y, mousestate, .. } => {
                    match game.state {
                        State::Paused => {
                            if mousestate.left() {
                                game.draw_point(x, y, false);
                            } else if mousestate.right() {
                                game.draw_point(x, y, true);
                            } else {
                                game.hover(x, y);
                            }
                        },
                        _ => {}
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    match game.state {
                        State::Paused => { game.state = State::Playing },
                        State::Playing => { game.state = State::Paused },
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    if let State::Paused = game.state {
                        game.save();
                    };
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    if let State::Paused = game.state {
                        game.load();
                    };
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    if let State::Paused = game.state {
                        game.clear();
                    };
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
            } else if *point == 2 {
                canvas.copy(
                    &hover,
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
                    &grid, 
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
