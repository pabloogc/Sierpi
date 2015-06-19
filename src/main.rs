extern crate piston_window;

use piston_window::*;

const WINDOW_W : u32 = 800;
const WINDOW_H : u32 = (0.866 * WINDOW_W as f64) as u32;
const DEPTH  : u32 = 7;

fn main() {
	let window: PistonWindow = WindowSettings::new("Hello Piston!", [WINDOW_W, WINDOW_H])
	.exit_on_esc(true).into();
	for e in window {
			e.draw_2d(|_, g| {
			clear([1.0; 4], g);
			});
			sierpi(&e, 0.0, 0.0, WINDOW_W as f64, WINDOW_H as f64, 0);
	}
}

fn sierpi(window:&PistonWindow, x:f64, y:f64, w:f64, h:f64, d:u32){
	if d == DEPTH {
			t2(window, x, y, w, h);
	} else {
			let wn = w / 2.0;
			let hn = h / 2.0;
			sierpi(window, x, y, wn, hn, d + 1);
			sierpi(window, x + wn, y, wn, hn, d + 1);
			sierpi(window, x + (wn / 2.0), y + hn, wn, hn, d + 1);
	}
}

fn t2(window:&PistonWindow, x:f64, y:f64, w:f64, h:f64){
		window.draw_2d(|c, g| {
			polygon([0.0, 0.0, 0.0, 1.0],
			&[
				[x, (WINDOW_H as f64) - y],
				[x + w, (WINDOW_H as f64) - y],
				[x + (w / 2.0), (WINDOW_H as f64) - y - h]
			], c.transform, g)
	})  ;
}

