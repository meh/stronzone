use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

extern crate stronzone;

fn main() {
	let mut canvas = stronzone::canvas()
		.add((0, 0), stronzone::grid::new()
			.brightness(55)
			.connect(("192.168.0.145", 9001)).unwrap());


	let mut rainbow = VecDeque::new();
	rainbow.push_back(stronzone::Led::new(1.00, 0.00, 0.00));
	rainbow.push_back(stronzone::Led::new(1.00, 0.50, 0.00));
	rainbow.push_back(stronzone::Led::new(1.00, 1.00, 0.00));
	rainbow.push_back(stronzone::Led::new(0.00, 1.00, 0.00));
	rainbow.push_back(stronzone::Led::new(0.00, 0.00, 1.00));
	rainbow.push_back(stronzone::Led::new(0.30, 0.00, 0.51));
	rainbow.push_back(stronzone::Led::new(0.54, 0.00, 1.00));

	loop {
		let gradient = stronzone::color::Gradient::new(rainbow.iter().cloned());

		for (x, led) in (0 .. canvas.width()).zip(gradient.take(canvas.width() as usize)) {
			for y in 0 .. canvas.height() {
				canvas.set((x, y), led);
			}
		}

		canvas.sync().unwrap();

		thread::sleep(Duration::from_millis(100));
		let color = rainbow.pop_front().unwrap();
		rainbow.push_back(color);
	}
}
