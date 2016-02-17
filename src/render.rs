extern crate rustbox;

use vex_editor::{Editor, Render};
use std::cmp;
use self::rustbox::{Color, RustBox, Key, Event};
use std::io::{BufReader, BufRead};

impl Render for rustbox::RustBox {
    fn render(&self, state: &Editor) {
        self.clear();
        self.present();

  		let h = self.height();
  		let w = self.width();

        let active = state.bufs.get(0).unwrap();
        let r = active.as_read_from(0);

	    let br = BufReader::new(r);

	    let other = String::from("~");
	    let lines : Vec<String> = br.lines().flat_map(|c| c).take(h).collect();

	    for i in 0..h {
	    	let line = lines.get(i).unwrap_or(&other);
	    	let text = line.replace("\t", "    ");
	        let formatted = format!("{: <1$}", text, w);
            self.print(0, i, rustbox::RB_NORMAL, Color::Default, Color::Default, &formatted);
	    }

        // let active = state.active().unwrap();
        // let (x, y) = active.point();
        // let (w, h) = active.window();

        // let offset = active.offset();
        // let other = String::from("~");

        // for i in 0..h {
        //     let line = active.lines.get(i+offset).unwrap_or(&other);
        //     // let num = format!("{:2}", i+offset);
        //     let text = line.replace("\t", "    ");
        //     // self.print(0, i + 1, rustbox::RB_BOLD, Color::Default, Color::Default, &num);
        //     let formatted = format!("{: <1$}", text, w);
        //     self.print(0, i + 1, rustbox::RB_NORMAL, Color::Default, Color::Default, &formatted);
        // }

        // let mut idx = 0;
        // for (i, buffer) in state.buffers.iter().enumerate() {
        //     let name = buffer.name();
        //     let (fg, bg) = if i == state.buf_idx {
        //         (Color::White, Color::Red)
        //     } else {
        //         (Color::Default, Color::Default)
        //     };
        //     self.print(idx, 0, rustbox::RB_NORMAL, fg, bg, &name);
        //     idx += 1 + name.len();
        // }

        self.present();
    }
}