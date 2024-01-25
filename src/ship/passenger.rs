use bevy::prelude::*;
use super::HullType;
use super::ShipData;

#[derive(Component)]
pub struct SpawnShuttle;

pub struct Shuttle { length: i32, width: i32, height: i32, hull_type: HullType, }

impl ShipData for Shuttle {
    fn new() -> Self {
        Shuttle { length: 2, width: 1, height: 1, hull_type: HullType::Passenger, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
    fn hull_type(&self) -> HullType { self.hull_type }
}