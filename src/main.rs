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
use service::{Services, Status};

const COLS: u16 = 400;
const ROWS: u16 = 300;

fn main() {
    let services = vec!["sshd", "syncthing@matze", "smb", "mpd", "nfs"];

    let mut black_buffer = [255u8; ROWS as usize * COLS as usize];
    let mut red_buffer = [0u8; ROWS as usize * COLS as usize];
    let mut inky = Inky::new(
        COLS,
        ROWS,
        Rotation::Rotate0,
        &mut black_buffer,
        &mut red_buffer,
    );
    let systemd = Services::new().expect("dbus");
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
        for (i, service) in services.iter().enumerate() {
            let (foreground, background, text) = match systemd.state(service) {
                Status::Running => (Color::Black, Color::White, "  RUNNING  "),
                Status::Stopped => (Color::Red, Color::White, "  STOPPED  "),
                Status::Failed => (Color::White, Color::Red, "  FAILED   "),
                Status::Unavailable => (Color::White, Color::Red, " NOT FOUND "),
                Status::Unknown => (Color::White, Color::Red, "  UNKNOWN  "),
            };
            let y = ROWS as i32 - 1 - (i as i32 + 1) * 11;
            d.draw(
                ProFont9Point::render_str(service)
                    .stroke(Some(Color::Black))
                    .fill(Some(Color::White))
                    .translate(Coord::new(1, y))
                    .into_iter(),
            );
            d.draw(
                ProFont9Point::render_str(text)
                    .stroke(Some(foreground))
                    .fill(Some(background))
                    .translate(Coord::new(150, y))
                    .into_iter(),
            );
        }
    })
    .expect("failed to draw");
}
