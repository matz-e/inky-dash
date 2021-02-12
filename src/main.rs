extern crate profont;
use profont::{ProFont12Point, ProFont14Point, ProFont24Point, ProFont9Point};

extern crate ssd1675;
use ssd1675::{Color, Rotation};

extern crate embedded_graphics;
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;

mod display;
use display::{Display, Inky};

mod service;

const COLS: u16 = 400;
const ROWS: u16 = 300;

struct ServiceDisplay<'a> {
    names: &'a [&'a str],
}

impl<'a> ServiceDisplay<'a> {
    fn new(names: &'a [&'a str]) -> Self {
        ServiceDisplay {
            names,
        }
    }

    fn draw(&self, d: &mut Display) {
        for (i, (name, state)) in service::state(self.names).iter().enumerate() {
            let (foreground, background, text) = match state {
                service::Status::Good => (Color::Black, Color::White, "  RUNNING  "),
                service::Status::Bad => (Color::White, Color::Red, "  STOPPED  "),
            };
            let y = ROWS as i32 - 1 - ((self.names.len() - i) as i32) * 11;
            d.draw(
                ProFont9Point::render_str(name)
                    .stroke(Some(Color::Black))
                    .fill(Some(Color::White))
                    .translate(Coord::new(1, y))
                    .into_iter(),
            );
            d.draw(
                ProFont9Point::render_str(text)
                    .stroke(Some(foreground))
                    .fill(Some(background))
                    .translate(Coord::new(100, y))
                    .into_iter(),
            );
        }
    }
}

fn main() {
    let services = vec!["sshd", "syncthing", "smbd", "mpd", "nfsd"];

    let mut black_buffer = [255u8; ROWS as usize * COLS as usize];
    let mut red_buffer = [0u8; ROWS as usize * COLS as usize];
    let mut inky = Inky::new(
        COLS,
        ROWS,
        Rotation::Rotate0,
        &mut black_buffer,
        &mut red_buffer,
    );
    let status = ServiceDisplay::new(&services[..]);
    inky.display(|d| {
        d.draw(
            ProFont24Point::render_str("TEST")
                .stroke(Some(Color::Black))
                .fill(Some(Color::White))
                .translate(Coord::new(1, 1))
                .into_iter(),
        );
        d.draw(
            ProFont12Point::render_str("We're somewhat useless right now")
                .stroke(Some(Color::Red))
                .fill(Some(Color::White))
                .translate(Coord::new(1, 30))
                .into_iter(),
        );
        status.draw(d);
    })
    .expect("failed to draw");
}
