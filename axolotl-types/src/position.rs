use std::fmt::Formatter;

use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq)]
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

pub struct RawPositionVisitor;

impl<'de> Visitor<'de> for RawPositionVisitor {
    type Value = RawPosition;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("An array of 3 doubles(f64)")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        if seq.size_hint() != Some(3) {
            return Err(A::Error::invalid_length(3, &self));
        }
        let raw_position = RawPosition {
            x: seq
                .next_element()?
                .ok_or_else(|| A::Error::invalid_length(3, &self))?,
            y: seq
                .next_element()?
                .ok_or_else(|| A::Error::invalid_length(3, &self))?,
            z: seq
                .next_element()?
                .ok_or_else(|| A::Error::invalid_length(3, &self))?,
        };
        Ok(raw_position)
    }
}

impl<'de> Deserialize<'de> for RawPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RawPositionVisitor)
    }
}

impl Serialize for RawPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.x)?;
        seq.serialize_element(&self.y)?;
        seq.serialize_element(&self.z)?;
        seq.end()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

pub struct RawRotationVisitor;

impl<'de> Visitor<'de> for RawRotationVisitor {
    type Value = RawRotation;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("An array of 3 doubles(f64)")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        if seq.size_hint() != Some(2) {
            return Err(A::Error::invalid_length(3, &self));
        }
        let raw_rotation = RawRotation {
            yaw: seq
                .next_element()?
                .ok_or_else(|| A::Error::invalid_length(3, &self))?,
            pitch: seq
                .next_element()?
                .ok_or_else(|| A::Error::invalid_length(3, &self))?,
        };
        Ok(raw_rotation)
    }
}

impl<'de> Deserialize<'de> for RawRotation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RawRotationVisitor)
    }
}

impl Serialize for RawRotation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.yaw)?;
        seq.serialize_element(&self.pitch)?;
        seq.end()
    }
}
