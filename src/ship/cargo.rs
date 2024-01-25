use bevy::prelude::*;
use super::HullType;
use super::ShipData;

#[derive(Component)]
pub struct SpawnCourier;

pub struct Courier { length: i32, width: i32, height: i32, hull_type: HullType, }

impl ShipData for Courier {
    fn new() -> Self {
        Courier { length: 2, width: 1, height: 1, hull_type: HullType::Cargo, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
    fn hull_type(&self) -> HullType { self.hull_type }
}

#[derive(Component)]
pub struct SpawnLightTransport;

pub struct LightTransport { length: i32, width: i32, height: i32, hull_type: HullType, }

impl ShipData for LightTransport {
    fn new() -> Self {
        LightTransport { length: 4, width: 1, height: 1, hull_type: HullType::Cargo, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
    fn hull_type(&self) -> HullType { self.hull_type }
}

#[derive(Component)]
pub struct SpawnTransport;

pub struct Transport { length: i32, width: i32, height: i32, hull_type: HullType, }

impl ShipData for Transport {
    fn new() -> Self {
        Transport { length: 4, width: 1, height: 2, hull_type: HullType::Cargo, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
    fn hull_type(&self) -> HullType { self.hull_type }
}

#[derive(Component)]
pub struct SpawnBarge;

pub struct Barge { length: i32, width: i32, height: i32, hull_type: HullType, }

impl ShipData for Barge {
    fn new() -> Self {
        Barge { length: 8, width: 2, height: 1, hull_type: HullType::Cargo, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
    fn hull_type(&self) -> HullType { self.hull_type }
}

#[derive(Component)]
pub struct SpawnHeavyTransport;

pub struct HeavyTransport { length: i32, width: i32, height: i32, hull_type: HullType, }

impl ShipData for HeavyTransport {
    fn new() -> Self {
        HeavyTransport { length: 4, width: 2, height: 2, hull_type: HullType::Cargo, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
    fn hull_type(&self) -> HullType { self.hull_type }
}

#[derive(Component)]
pub struct SpawnFreighter;

pub struct Freighter { length: i32, width: i32, height: i32, hull_type: HullType, }

impl ShipData for Freighter {
    fn new() -> Self {
        Freighter { length: 8, width: 2, height: 2, hull_type: HullType::Cargo, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
    fn hull_type(&self) -> HullType { self.hull_type }
}

#[derive(Component)]
pub struct SpawnContainerShip;

pub struct ContainerShip { length: i32, width: i32, height: i32, hull_type: HullType, }

impl ShipData for ContainerShip {
    fn new() -> Self {
        ContainerShip { length: 8, width: 2, height: 4, hull_type: HullType::Cargo, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
    fn hull_type(&self) -> HullType { self.hull_type }
}