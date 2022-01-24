use std::{time::Instant, sync::mpsc::{channel, Receiver}, process::exit};

use primitives::{Movement, Square, Error};
use raylib::{color::Color, prelude::RaylibDraw};
use tokio::spawn;

mod primitives;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (mut ray_handle, thread) = raylib::init()
        .size(800, 600)
        .title("formata essa desgraça direito porra")
        .build();

    let timing_zero = Instant::now();

    let (tx, rx) = channel();
    spawn(async move {
        let timing_zero = timing_zero;
        timings(timing_zero, rx).await;
    });

    let mut obj = Movement {
        object: Square {
            pos_x: 30,
            pos_y: 30,
            size: 20,
            color: Color::WHITE,
        },
        speed_x: 0,
        speed_y: 0,
    };

    while !ray_handle.window_should_close() {
        let mut draw_handle = ray_handle.begin_drawing(&thread);

        draw_handle.clear_background(Color::BLACK);
        draw_handle.draw_text("heya", 800 / 2 - 20, 600 / 64, 20, Color::WHITE);

        draw_handle.draw_line(800 / 16, 600 / 16, 800 - 800 / 16, 600 / 16, Color::WHITE);
        obj.tick(&mut draw_handle);

        tx.send(()).map_err(Error::TxSend)?;
    }
    exit(0);
}

async fn timings(zero: Instant, rx: Receiver<()>) {
    let mut ticks: u128 = 0;

    loop {
        rx.recv().unwrap_or(());
        ticks += 1;
        print!("\r{} ticks for {:?} time", ticks, Instant::now() - zero)
    }
}
