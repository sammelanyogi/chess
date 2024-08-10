use bevy::prelude::*;

#[derive(Debug)]
pub struct Position(pub u8, pub u8);

#[derive(Event, Debug)]
pub struct SelectSquareEvent(pub Position);

#[derive(Component, Debug)]
pub struct Square;

#[derive(Component, Debug)]
pub struct Overlay;
