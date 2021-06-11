use sdl2;
use sdl2::{render::Canvas, video::Window, pixels, rect::Rect};

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_ctx: &sdl2::Sdl) -> Self {
	let video = sdl_ctx.video().unwrap();
	let window = video
	    .window(
		"chip8-rs",
		(crate::SCREEN_WIDTH * crate::SCREEN_SCALE) as u32,
		(crate::SCREEN_HEIGHT * crate::SCREEN_SCALE) as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

	let mut canvas = window
	    .into_canvas()
	    .build()
	    .unwrap();

	canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
	canvas.clear();
	canvas.present();

	Display {
	    canvas,
	}
    } 

    pub fn draw(&mut self, pixel: &[[u8; crate::SCREEN_WIDTH]; crate::SCREEN_HEIGHT]) {
	for (y, &row) in pixel.iter().enumerate() {
	    for (x, &column) in row.iter().enumerate() {
		self.canvas.set_draw_color(
		    if column == 1 {
			pixels::Color::RGB(255, 255, 255)
		    }
		    else {
			pixels::Color::RGB(0, 0, 0)
		    });
		let _ = self.canvas
		    .fill_rect(
			Rect::new((x * crate::SCREEN_SCALE) as i32, (y * crate::SCREEN_SCALE) as i32, crate::SCREEN_SCALE as u32, crate::SCREEN_SCALE as u32));
	    }
	}
	self.canvas.present();
    }
}
