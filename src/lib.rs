extern crate byteorder;

pub extern crate palette;
pub use palette as color;
pub type Led = color::Rgb;

mod canvas;
pub use canvas::Canvas;

pub mod grid;
pub use grid::Grid;

pub fn canvas() -> Canvas {
	Canvas::new()
}
