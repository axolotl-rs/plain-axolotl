use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

use serde::de::{Error, Visitor};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub trait NamespacedKey: Display + Hash + Into<(String, String)> {
    fn get_key(&self) -> &str;
    fn get_namespace(&self) -> &str;

    fn as_tuple(&self) -> (&str, &str);
}

#[derive(Debug)]
pub struct BadNamespacedKeyError;

impl Display for BadNamespacedKeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Bad Namespaced Key")
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct OwnedNameSpaceKey {
    namespace: String,
    key: String,
}

impl Into<(String, String)> for OwnedNameSpaceKey {
    fn into(self) -> (String, String) {
        (self.namespace, self.key)
    }
}

impl Into<(String, String)> for &'_ OwnedNameSpaceKey {
    fn into(self) -> (String, String) {
        (self.namespace.clone(), self.key.clone())
    }
}

impl NamespacedKey for &'_ OwnedNameSpaceKey {
    fn get_key(&self) -> &str {
        &self.key
    }
    fn get_namespace(&self) -> &str {
        &self.namespace
    }

    fn as_tuple(&self) -> (&str, &str) {
        (&self.namespace, &self.key)
    }
}

impl FromStr for OwnedNameSpaceKey {
    type Err = BadNamespacedKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let namespace = split.next().ok_or(BadNamespacedKeyError)?;
        let key = split.next().ok_or(BadNamespacedKeyError)?;
        Ok(Self {
            namespace: namespace.to_string(),
            key: key.to_string(),
        })
    }
}

impl TryFrom<String> for OwnedNameSpaceKey {
    type Error = BadNamespacedKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Serialize for OwnedNameSpaceKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct NameSpaceKeyVisitor;

impl<'de> Visitor<'de> for NameSpaceKeyVisitor {
    type Value = OwnedNameSpaceKey;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
    {
        OwnedNameSpaceKey::from_str(v).map_err(Error::custom)
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
    {
        OwnedNameSpaceKey::from_str(&v).map_err(Error::custom)
    }
}

impl<'de> Deserialize<'de> for OwnedNameSpaceKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NameSpaceKeyVisitor)
    }
}

impl OwnedNameSpaceKey {
    pub fn new(namespace: String, key: String) -> Self {
        Self { namespace, key }
    }
}

impl Display for OwnedNameSpaceKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.key)
    }
}

impl NamespacedKey for OwnedNameSpaceKey {
    fn get_key(&self) -> &str {
        &self.key
    }
    fn get_namespace(&self) -> &str {
        &self.namespace
    }

    fn as_tuple(&self) -> (&str, &str) {
        (&self.namespace, &self.key)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct NameSpaceRef<'a> {
    namespace: &'a str,
    key: &'a str,
}

impl Serialize for NameSpaceRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'a> NameSpaceRef<'a> {
    pub fn new(namespace: &'a str, key: &'a str) -> Self {
        Self { namespace, key }
    }
}

impl<'a> Display for NameSpaceRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.key)
    }
}

impl Into<(String, String)> for NameSpaceRef<'_> {
    fn into(self) -> (String, String) {
        (self.namespace.to_string(), self.key.to_string())
    }
}

impl<'a> NamespacedKey for NameSpaceRef<'a> {
    fn get_key(&self) -> &str {
        self.key
    }
    fn get_namespace(&self) -> &str {
        self.namespace
    }

    fn as_tuple(&self) -> (&str, &str) {
        (self.namespace, self.key)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum NameSpaceKey<'key> {
    Owned(OwnedNameSpaceKey),
    RefOwned(&'key OwnedNameSpaceKey),
    Ref(NameSpaceRef<'key>),
}


impl Display for NameSpaceKey<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NameSpaceKey::Owned(key) => write!(f, "{}", key),
            NameSpaceKey::RefOwned(key) => write!(f, "{}", key),
            NameSpaceKey::Ref(key) => write!(f, "{}", key),
        }
    }
}

impl Into<(String, String)> for NameSpaceKey<'_> {
    fn into(self) -> (String, String) {
        match self {
            NameSpaceKey::Owned(owned) => owned.into(),
            NameSpaceKey::RefOwned(owned) => owned.into(),
            NameSpaceKey::Ref(r) => r.into(),
        }
    }
}

impl NamespacedKey for NameSpaceKey<'_> {
    fn get_key(&self) -> &str {
        match self {
            NameSpaceKey::Owned(owned) => owned.get_key(),
            NameSpaceKey::RefOwned(owned) => owned.get_key(),
            NameSpaceKey::Ref(r) => r.get_key(),
        }
    }
    fn get_namespace(&self) -> &str {
        match self {
            NameSpaceKey::Owned(owned) => owned.get_namespace(),
            NameSpaceKey::RefOwned(owned) => owned.get_namespace(),
            NameSpaceKey::Ref(r) => r.get_namespace(),
        }
    }

    fn as_tuple(&self) -> (&str, &str) {
        match self {
            NameSpaceKey::Owned(owned) => owned.as_tuple(),
            NameSpaceKey::RefOwned(owned) => owned.as_tuple(),
            NameSpaceKey::Ref(r) => r.as_tuple(),
        }
    }
}

impl Serialize for NameSpaceKey<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            NameSpaceKey::Owned(owned) => owned.serialize(serializer),
            NameSpaceKey::RefOwned(owned) => owned.serialize(serializer),
            NameSpaceKey::Ref(name_space_ref) => name_space_ref.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for NameSpaceKey<'_> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        OwnedNameSpaceKey::deserialize(deserializer).map(NameSpaceKey::Owned)
    }
}