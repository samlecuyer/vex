extern crate rustbox;

use vex_editor::{Editor, Render};
use std::cmp;
use self::rustbox::{Color, RustBox, Key, Event};
use std::io::{BufReader, BufRead};

impl Render for rustbox::RustBox {
    fn render(&self, state: &Editor) {
        self.clear();
        self.present();

        let top = state.top;
  		let h = self.height();
  		let w = self.width();

        let active = state.bufs.get(0).unwrap();
        let r = active.as_read_from(0);

	    let br = BufReader::new(r);

	    let other = String::from("~");
	    let lines : Vec<String> = br.lines().flat_map(|c| c).skip(top).take(h).collect();

	    for i in 0..h {
	    	let line = lines.get(i).unwrap_or(&other);
	    	let text = line.replace("\t", "    ");
	        let formatted = format!("{: <1$}", text, w);
            self.print(0, i, rustbox::RB_NORMAL, Color::Default, Color::Default, &formatted);
	    }

        self.present();
    }
}