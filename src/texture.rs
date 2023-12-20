use sdl2::rect::Point;
use sdl2::render::{Canvas, TextureCreator, Texture, TextureAccess};
use sdl2::video::{Window, WindowContext};
use sdl2::pixels::Color;

pub fn create_cell<'a>(
    creator: &'a TextureCreator<WindowContext>, 
    canvas: &mut Canvas<Window>,
    size: u32,
    color: Color
    ) -> Result<Texture<'a>, String> {
    if let Ok(mut texture) = creator
        .create_texture(None, TextureAccess::Target, size, size) {
        
        let _ = canvas.with_texture_canvas(&mut texture, |cnv| {

            cnv.set_draw_color(color);
            for i in 1..size-1 {
                for j in 1..size-1 {
                    cnv.draw_point(Point::new(j as i32, i as i32))
                        .expect("Failed to draw");
                }
            }
            cnv.set_draw_color(Color::RGB(128, 128, 128));
            for i in 0..size {
                cnv.draw_point(Point::new(i as i32, (size - 1) as i32))
                    .expect("Failed to draw");
                cnv.draw_point(Point::new(i as i32, 0))
                    .expect("Failed to draw");
                cnv.draw_point(Point::new(0, i as i32))
                    .expect("Failed to draw");
                cnv.draw_point(Point::new((size - 1) as i32, i as i32))
                    .expect("Failed to draw");
            }
        });


        return Ok(texture);
    };

    Err("Unable to create texture".to_string())
}
