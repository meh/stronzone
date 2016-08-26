use std::io;
use std::net::{UdpSocket, ToSocketAddrs};
use std::io::Cursor;
use byteorder::{LittleEndian, WriteBytesExt};

use Led;

pub struct Grid {
	socket: UdpSocket,

	brightness:  i32,
	coordinates: [[u8; 8]; 8],
	data:        [Led; 8 * 8],
}

impl Grid {
	pub fn connect<T: ToSocketAddrs>(server: T) -> io::Result<Self> {
		let socket = try!(UdpSocket::bind(("0.0.0.0", 0)));
		try!(socket.connect(server));

		Ok(Grid {
			socket: socket,

			brightness:  55,
			coordinates: super::UP,
			data:        [Led::new(0.0, 0.0, 0.0); 8 * 8],
		})
	}

	pub fn coordinates(&mut self, coordinates: [[u8; 8]; 8]) {
		self.coordinates = coordinates;
	}

	pub fn clear(&mut self) {
		for led in &mut self.data[..] {
			*led = Led::new(0.0, 0.0, 0.0);
		}
	}

	pub fn brightness(&mut self, value: i32) {
		self.brightness = value;
	}

	pub fn set(&mut self, (x, y): (u32, u32), led: Led) {
		if x < 8 && y < 8 {
			let index = self.coordinates[y as usize][x as usize];
			self.data[index as usize] = led;
		}
	}

	pub fn get(&self, (x, y): (u32, u32)) -> Option<Led> {
		if x < 8 && y < 8 {
			let index = self.coordinates[y as usize][x as usize];
			Some(self.data[index as usize])
		}
		else {
			None
		}
	}

	pub fn sync(&mut self) -> io::Result<()> {
		let mut buffer = [0; 4 + (8 * 8 * 4)];

		{
			let mut cursor = Cursor::new(&mut buffer[..]);
			try!(cursor.write_i32::<LittleEndian>(self.brightness));

			for led in &self.data[..] {
				try!(cursor.write_u32::<LittleEndian>(
					((led.green * 255.0)   as u32) << 16 |
					((led.red * 255.0) as u32) << 8 |
					((led.blue * 255.0)  as u32)));
			}
		}

		try!(self.socket.send(&buffer[..]));

		Ok(())
	}
}
