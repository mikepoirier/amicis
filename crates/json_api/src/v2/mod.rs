use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct NoResource;

pub struct DocumentBuilder<D> {
    data: D,
    includes: Option<Vec<Resource>>,
}

impl Default for DocumentBuilder<NoResource> {
    fn default() -> Self {
        Self {
            data: NoResource,
            includes: None,
        }
    }
}

impl DocumentBuilder<NoResource> {
    pub fn data(self, resource: impl Into<Resource>) -> DocumentBuilder<Resource> {
        let resource = resource.into();
        DocumentBuilder {
            data: resource,
            includes: self.includes,
        }
    }
}

impl DocumentBuilder<Resource> {
    pub fn includes(mut self, resource: impl Into<Resource>) -> Self {
        let resource = resource.into();
        let includes = self.includes.get_or_insert(vec![]);
        includes.push(resource);
        self
    }

    pub fn includes_all(mut self, resources: Vec<impl Into<Resource>>) -> Self {
        let resources = resources.into_iter().map(Into::into);
        let includes = self.includes.get_or_insert(vec![]);
        includes.extend(resources);
        self
    }

    pub fn build(self) -> Document {
        Document {
            data: self.data,
            includes: self.includes,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Document {
    // TODO: This may need to be refined back to a Resource
    data: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<Vec<Resource>>,
}

impl Document {
    pub fn builder() -> DocumentBuilder<NoResource> {
        DocumentBuilder::default()
    }
}

#[derive(Default)]
pub struct NoKind;
#[derive(Default)]
pub struct Kind(String);
#[derive(Default)]
pub struct NoUri;
#[derive(Default)]
pub struct Uri(String);

#[derive(Default)]
pub struct ResourceBuilder<T, U> {
    kind: T,
    uri: U,
    attributes: AttributesObject,
    relationships: RelationshipsObject,
}

impl ResourceBuilder<NoKind, NoUri> {
    fn new() -> Self {
        Self::default()
    }
}

impl<T, U> ResourceBuilder<T, U> {
    pub fn attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> ResourceBuilder<T, U> {
        self.attributes.insert(key.into(), value.into());
        self
    }

    pub fn relationship(
        mut self,
        key: impl Into<String>,
        value: impl Into<Relationship>,
    ) -> ResourceBuilder<T, U> {
        self.relationships.insert(key.into(), value.into());
        self
    }
}

impl<U> ResourceBuilder<NoKind, U> {
    pub fn kind(self, kind: impl Into<String>) -> ResourceBuilder<Kind, U> {
        ResourceBuilder {
            kind: Kind(kind.into()),
            uri: self.uri,
            attributes: self.attributes,
            relationships: self.relationships,
        }
    }
}

impl<T> ResourceBuilder<T, NoUri> {
    pub fn uri(self, uri: impl Into<String>) -> ResourceBuilder<T, Uri> {
        ResourceBuilder {
            kind: self.kind,
            uri: Uri(uri.into()),
            attributes: self.attributes,
            relationships: self.relationships,
        }
    }
}

impl ResourceBuilder<Kind, Uri> {
    pub fn build(self) -> Resource {
        Resource {
            kind: self.kind.0,
            uri: self.uri.0,
            attributes: self.attributes,
            relationships: self.relationships,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Resource {
    #[serde(rename = "type")]
    kind: String,
    uri: String,
    #[serde(skip_serializing_if = "AttributesObject::is_empty")]
    attributes: AttributesObject,
    #[serde(skip_serializing_if = "RelationshipsObject::is_empty")]
    relationships: RelationshipsObject,
}

impl Resource {
    pub fn builder() -> ResourceBuilder<NoKind, NoUri> {
        ResourceBuilder::new()
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct AttributesObject(HashMap<String, Value>);

impl AttributesObject {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

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
struct RelationshipsObject(HashMap<String, Relationship>);

impl RelationshipsObject {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Deref for RelationshipsObject {
    type Target = HashMap<String, Relationship>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RelationshipsObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Relationship(CoreRelationship);

impl Relationship {
    pub fn from_vec<T: Into<RelationshipObject>>(vec: Vec<T>) -> Self {
        Self(CoreRelationship::Multi(
            vec.into_iter().map(Into::into).collect(),
        ))
    }
}

impl<T: Into<RelationshipObject>> From<T> for Relationship {
    fn from(value: T) -> Self {
        Self(CoreRelationship::Single(value.into()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum CoreRelationship {
    Single(RelationshipObject),
    Multi(Vec<RelationshipObject>),
}

#[derive(Default)]
pub struct NoRel;
#[derive(Default)]
pub struct Rel(String);
#[derive(Default)]
pub struct NoHref;
#[derive(Default)]
pub struct Href(String);

#[derive(Debug, Default)]
pub struct RelationshipObjectBuilder<R, H> {
    rel: R,
    href: H,
    extensions: HashMap<String, Value>,
}

impl RelationshipObjectBuilder<NoRel, NoHref> {
    fn new() -> Self {
        Self::default()
    }
}

impl<H> RelationshipObjectBuilder<NoRel, H> {
    pub fn rel(self, rel: impl Into<String>) -> RelationshipObjectBuilder<Rel, H> {
        RelationshipObjectBuilder {
            rel: Rel(rel.into()),
            href: self.href,
            extensions: self.extensions,
        }
    }
}

impl<R> RelationshipObjectBuilder<R, NoHref> {
    pub fn href(self, href: impl Into<String>) -> RelationshipObjectBuilder<R, Href> {
        RelationshipObjectBuilder {
            rel: self.rel,
            href: Href(href.into()),
            extensions: self.extensions,
        }
    }
}

impl<H, R> RelationshipObjectBuilder<H, R> {
    pub fn extension(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extensions.insert(key.into(), value.into());
        self
    }
}

impl RelationshipObjectBuilder<Rel, Href> {
    pub fn build(self) -> RelationshipObject {
        RelationshipObject {
            rel: self.rel.0,
            href: self.href.0,
            extensions: self.extensions,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelationshipObject {
    rel: String,
    href: String,
    #[serde(flatten)]
    extensions: HashMap<String, Value>,
}

impl RelationshipObject {
    pub fn builder() -> RelationshipObjectBuilder<NoRel, NoHref> {
        RelationshipObjectBuilder::new()
    }
}

// impl<T> From<Vec<T>> for RelationshipObject
// where
//     T: Into<RelationshipObject>,
// {
//     fn from(value: Vec<T>) -> Self {
//         todo!()
//     }
// }

// impl<T: Into<RelationshipObject>> Into<RelationshipObject> for Vec<T> {
//     fn into(self) -> RelationshipObject {
//         todo!()
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    kind: String,
}
