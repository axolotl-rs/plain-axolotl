use axolotl_nbt_macros::ListSerialize;
use std::fmt::Formatter;

use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, ListSerialize)]
pub struct RawPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Into<[f64; 3]> for RawPosition {
    fn into(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<[f64; 3]> for RawPosition {
    fn from([x, y, z]: [f64; 3]) -> Self {
        Self { x, y, z }
    }
}

impl Into<(f64, f64, f64)> for RawPosition {
    fn into(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

impl From<(f64, f64, f64)> for RawPosition {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ListSerialize)]
pub struct RawRotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Into<[f32; 2]> for RawRotation {
    fn into(self) -> [f32; 2] {
        [self.yaw, self.pitch]
    }
}

impl From<[f64; 2]> for RawRotation {
    fn from([yaw, pitch]: [f64; 2]) -> Self {
        Self {
            yaw: yaw as f32,
            pitch: pitch as f32,
        }
    }
}

impl Into<(f32, f32)> for RawRotation {
    fn into(self) -> (f32, f32) {
        (self.yaw, self.pitch)
    }
}

impl From<(f32, f32)> for RawRotation {
    fn from((yaw, pitch): (f32, f32)) -> Self {
        Self { yaw, pitch }
    }
}
