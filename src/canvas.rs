use std::io;
use std::collections::HashMap;

use {Led, Grid};

pub struct Canvas {
	grids: HashMap<(u32, u32), Grid>,
}

impl Canvas {
	pub fn new() -> Self {
		Canvas {
			grids: HashMap::new(),
		}
	}

	pub fn add(mut self, (x, y): (u32, u32), grid: Grid) -> Self {
		self.grids.insert((x, y), grid);
		self
	}

	pub fn width(&self) -> u32 {
		(self.grids.keys().map(|&(x, _)| x).max().unwrap_or(0) + 1) * 8
	}

	pub fn height(&self) -> u32 {
		(self.grids.keys().map(|&(_, y)| y).max().unwrap_or(0) + 1) * 8
	}

	pub fn grid(&self, (x, y): (u32, u32)) -> Option<&Grid> {
		self.grids.get(&(x, y))
	}

	pub fn grid_mut(&mut self, (x, y): (u32, u32)) -> Option<&mut Grid> {
		self.grids.get_mut(&(x, y))
	}

	pub fn set(&mut self, (x, y): (u32, u32), led: Led) {
		if let Some(grid) = self.grids.get_mut(&(x / 8, y / 8)) {
			grid.set((x % 8, y % 8), led);
		}
	}

	pub fn get(&mut self, (x, y): (u32, u32)) -> Option<Led> {
		self.grids.get(&(x / 8, y / 8)).and_then(|grid|
			grid.get((x % 8, y % 8)))
	}

	pub fn clear(&mut self) {
		for grid in self.grids.values_mut() {
			grid.clear();
		}
	}

	pub fn brightness(&mut self, value: i32) {
		for grid in self.grids.values_mut() {
			grid.brightness(value);
		}
	}

	pub fn sync(&mut self) -> io::Result<()> {
		for grid in self.grids.values_mut() {
			try!(grid.sync());
		}

		Ok(())
	}
}
