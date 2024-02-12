use serde::{Deserialize, Serialize};

use self::resource::{ResourceIdentifierObject, ResourceObject};

pub mod links;
pub mod resource;

#[derive(Debug, Deserialize, Serialize)]
pub struct Document(CoreDocument);

impl Document {
    pub fn builder() -> DocumentBuilder<NoData, NoError> {
        DocumentBuilder::default()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum CoreDocument {
    Data { data: CoreData },
    Error { error: ErrorObject },
}

pub struct NoData;
pub struct SomeData(CoreData);
pub struct NoError;
pub struct SomeError(ErrorObject);

pub struct DocumentBuilder<D, E> {
    data: D,
    error: E,
}

impl DocumentBuilder<SomeData, NoError> {
    pub fn build(self) -> Document {
        Document(CoreDocument::Data { data: self.data.0 })
    }
}

impl DocumentBuilder<NoData, SomeError> {
    pub fn build(self) -> Document {
        Document(CoreDocument::Error {
            error: self.error.0,
        })
    }
}

impl DocumentBuilder<NoData, NoError> {
    pub fn data(self, data: Data) -> DocumentBuilder<SomeData, NoError> {
        DocumentBuilder {
            data: SomeData(data.0),
            error: NoError,
        }
    }

    pub fn error(self, error: ErrorObject) -> DocumentBuilder<NoData, SomeError> {
        DocumentBuilder {
            data: NoData,
            error: SomeError(error),
        }
    }
}

impl Default for DocumentBuilder<NoData, NoError> {
    fn default() -> Self {
        Self {
            data: NoData,
            error: NoError,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data(CoreData);

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum CoreData {
    Single(Single),
    Collection(Collection),
}

impl Data {
    pub fn builder() -> DataBuilder<NoSingle, NoCollection> {
        DataBuilder::default()
    }
}

pub struct NoSingle;
pub struct SomeSingle(Single);
pub struct NoCollection;
pub struct SomeCollection(Collection);

pub struct DataBuilder<S, C> {
    single: S,
    collection: C,
}

impl DataBuilder<NoSingle, NoCollection> {
    pub fn resource(
        self,
        resource: impl Into<ResourceObject>,
    ) -> DataBuilder<SomeSingle, NoCollection> {
        let resource = resource.into();
        DataBuilder {
            single: SomeSingle(Single::Resource(resource)),
            collection: NoCollection,
        }
    }

    pub fn resources(
        self,
        resources: Vec<impl Into<ResourceObject>>,
    ) -> DataBuilder<NoSingle, SomeCollection> {
        let resources = resources.into_iter().map(Into::into).collect();
        DataBuilder {
            single: NoSingle,
            collection: SomeCollection(Collection::Resource(resources)),
        }
    }

    pub fn resource_identifier(
        self,
        resource_identifier: impl Into<ResourceIdentifierObject>,
    ) -> DataBuilder<SomeSingle, NoCollection> {
        let resource = resource_identifier.into();
        DataBuilder {
            single: SomeSingle(Single::Identifier(resource)),
            collection: NoCollection,
        }
    }

    pub fn resource_identifiers(
        self,
        resource_identifiers: Vec<impl Into<ResourceIdentifierObject>>,
    ) -> DataBuilder<NoSingle, SomeCollection> {
        let resource_identifiers = resource_identifiers.into_iter().map(Into::into).collect();
        DataBuilder {
            single: NoSingle,
            collection: SomeCollection(Collection::Identifier(resource_identifiers)),
        }
    }
}

impl DataBuilder<SomeSingle, NoCollection> {
    pub fn build(self) -> Data {
        Data(CoreData::Single(self.single.0))
    }
}

impl DataBuilder<NoSingle, SomeCollection> {
    pub fn build(self) -> Data {
        Data(CoreData::Collection(self.collection.0))
    }
}

impl Default for DataBuilder<NoSingle, NoCollection> {
    fn default() -> Self {
        Self {
            single: NoSingle,
            collection: NoCollection,
        }
    }
}

// TODO: Fill this out
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorObject;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Single {
    Resource(ResourceObject),
    Identifier(ResourceIdentifierObject),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Collection {
    Resource(Vec<ResourceObject>),
    Identifier(Vec<ResourceIdentifierObject>),
}
