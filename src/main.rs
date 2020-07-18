extern crate profont;
use profont::{ProFont12Point, ProFont14Point, ProFont24Point, ProFont9Point};

extern crate ssd1675;
use ssd1675::{Color, Rotation};

extern crate embedded_graphics;
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;

mod display;
use display::Inky;

mod service;
use service::Services;

const COLS: u16 = 400;
const ROWS: u16 = 300;

fn main() {
    println!("{:?}", Services::new().expect("dbus").state("gdm"));
    println!("{:?}", Services::new().expect("dbus").state("sshd"));
    println!("{:?}", Services::new().expect("dbus").state("syncthing@matze"));
    println!("{:?}", Services::new().expect("dbus").state("foob"));
    println!("{:?}", Services::new().expect("dbus").state("mpd"));
    let mut black_buffer = [255u8; ROWS as usize * COLS as usize];
    let mut red_buffer = [0u8; ROWS as usize * COLS as usize];
    let mut inky = Inky::new(
        COLS,
        ROWS,
        Rotation::Rotate0,
        &mut black_buffer,
        &mut red_buffer,
    );
    inky.display(|d| {
        d.draw(
            ProFont24Point::render_str("TEST")
                .stroke(Some(Color::Black))
                .fill(Some(Color::White))
                .translate(Coord::new(1, 1))
                .into_iter(),
        );
        d.draw(
            ProFont9Point::render_str("We're somewhat useless right now")
                .stroke(Some(Color::Red))
                .fill(Some(Color::White))
                .translate(Coord::new(1, 30))
                .into_iter(),
        );
    })
    .expect("failed to draw");
}
