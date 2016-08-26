use std::collections::HashMap;
use std::io;
use std::net::ToSocketAddrs;
use {Led, Grid};

pub struct Builder {
	brightness:  i32,
	coordinates: [[u8; 8]; 8],
	data:        HashMap<(u32, u32), Led>,
}

impl Builder {
	pub fn new() -> Self {
		Builder {
			brightness:  0,
			coordinates: super::UP,
			data:        HashMap::new(),
		}
	}

	pub fn brightness(&mut self, value: i32) -> &mut Self {
		self.brightness = value;
		self
	}

	pub fn coordinates(&mut self, value: [[u8; 8]; 8]) -> &mut Self {
		self.coordinates = value;
		self
	}

	pub fn set(&mut self, (x, y): (u32, u32), led: Led) -> &mut Self {
		self.data.insert((x, y), led);
		self
	}

	pub fn connect<T: ToSocketAddrs>(&mut self, addr: T) -> io::Result<Grid> {
		let mut grid = try!(Grid::connect(addr));
		grid.brightness(self.brightness);
		grid.coordinates(self.coordinates);

		for (&(x, y), &led) in &self.data {
			grid.set((x, y), led);
		}

		Ok(grid)
	}
}
