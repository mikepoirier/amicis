use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::links::{Link, LinksObject};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceObject {
    #[serde(rename = "type")]
    kind: String,
    id: String,
    attributes: AttributesObject,
    #[serde(skip_serializing_if = "RelationshipsObject::is_empty")]
    relationships: RelationshipsObject,
    links: LinksObject,
}

impl ResourceObject {
    pub fn new(kind: impl Into<String>, id: impl Into<String>) -> Self {
        let kind = kind.into();
        let id = id.into();
        Self {
            kind,
            id,
            attributes: Default::default(),
            relationships: Default::default(),
            links: Default::default(),
        }
    }

    pub fn attribute(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    pub fn link(mut self, name: impl Into<String>, link: impl Into<Link>) -> Self {
        self.links.insert(name.into(), link.into());
        self
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceIdentifierObject {
    #[serde(rename = "type")]
    kind: String,
    id: String,
}

impl ResourceIdentifierObject {
    pub fn new(kind: impl Into<String>, id: impl Into<String>) -> Self {
        let kind = kind.into();
        let id = id.into();
        Self { kind, id }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AttributesObject(HashMap<String, Value>);

impl Deref for AttributesObject {
    type Target = HashMap<String, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AttributesObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RelationshipsObject(HashMap<String, RelationshipObject>);

impl RelationshipsObject {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// TODO: must contain one of "links" or "data"
#[derive(Debug, Deserialize, Serialize)]
pub struct RelationshipObject {
    // TODO: links must have either "self" or "related" link
    links: Option<LinksObject>,
    data: Option<ResourceLinkage>,
}

#[derive(Debug, Deserialize, Serialize)]
enum ResourceLinkage {
    Single(Option<ResourceIdentifierObject>),
    Many(Vec<ResourceIdentifierObject>),
}
