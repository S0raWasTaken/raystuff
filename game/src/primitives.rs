#![allow(dead_code, non_camel_case_types)]
use std::sync::mpsc::SendError;

use game_macros::Coordinates;
use raylib::{color::Color, core::drawing::RaylibDrawHandle, prelude::RaylibDraw};

#[derive(Debug)]
pub enum Error {
    Speed(String),
    TxSend(SendError<()>),
}

pub trait Coords {
    fn pos_x(&mut self) -> &mut i32;
    fn pos_y(&mut self) -> &mut i32;
}

pub trait Draw {
    fn draw(&self, handle: &mut RaylibDrawHandle);
}

#[derive(Coordinates, Debug)]
pub struct Square {
    pub pos_x: i32,
    pub pos_y: i32,
    pub size: i8,
    pub color: Color,
}

impl Draw for Square {
    fn draw(&self, handle: &mut RaylibDrawHandle) {
        handle.draw_rectangle(
            self.pos_x,
            self.pos_y,
            self.size.into(),
            self.size.into(),
            self.color,
        );
    }
}

#[derive(Debug)]
pub struct Movement<T> {
    pub object: T,
    pub speed_x: i8,
    pub speed_y: i8,
}

impl<T> Movement<T>
where
    T: Coords + Draw,
{
    fn draw(&self, handle: &mut RaylibDrawHandle) {
        self.object.draw(handle)
    }
    pub fn tick(&mut self, handle: &mut RaylibDrawHandle) {
        *self.object.pos_x() += self.speed_x as i32;
        *self.object.pos_y() += self.speed_y as i32;
        self.draw(handle);
    }
}
