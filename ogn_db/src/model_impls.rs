use serde::{Deserialize, Serialize};

use crate::{DocumentId, IdeaId};

impl<'de> Deserialize<'de> for DocumentId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let id = s.parse::<i32>().map_err(|_|
            serde::de::Error::custom("couldn't parse document id from payload")
        )?;
        Ok(DocumentId(id))
    }
}

impl Serialize for DocumentId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for IdeaId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let id = s.parse::<i32>().map_err(|_|
            serde::de::Error::custom("couldn't parse idea id from payload")
        )?;
        Ok(IdeaId(id))
    }
}

impl Serialize for IdeaId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

