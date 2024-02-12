use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LinksObject(HashMap<String, Link>);

impl Deref for LinksObject {
    type Target = HashMap<String, Link>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LinksObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct NoLinkData;
pub struct StringLinkData(String);
pub struct ObjectLinkData(LinkObject);

pub struct LinkBuilder<T> {
    data: T,
}

impl Default for LinkBuilder<NoLinkData> {
    fn default() -> Self {
        Self { data: NoLinkData }
    }
}

impl LinkBuilder<NoLinkData> {
    pub fn href(self, href: impl Into<String>) -> LinkBuilder<StringLinkData> {
        let href = href.into();
        LinkBuilder {
            data: StringLinkData(href),
        }
    }
}

impl LinkBuilder<StringLinkData> {
    pub fn rel(self, rel: impl Into<String>) -> LinkBuilder<ObjectLinkData> {
        let rel = rel.into();
        LinkBuilder {
            data: ObjectLinkData(LinkObject {
                href: self.data.0,
                rel: Some(rel),
                described_by: None,
                title: None,
                kind: None,
                hreflang: None,
            }),
        }
    }

    pub fn described_by(self, link: impl Into<Link>) -> LinkBuilder<ObjectLinkData> {
        let link = link.into();
        LinkBuilder {
            data: ObjectLinkData(LinkObject {
                href: self.data.0,
                rel: None,
                described_by: Some(Box::new(link)),
                title: None,
                kind: None,
                hreflang: None,
            }),
        }
    }

    pub fn title(self, title: impl Into<String>) -> LinkBuilder<ObjectLinkData> {
        let title = title.into();
        LinkBuilder {
            data: ObjectLinkData(LinkObject {
                href: self.data.0,
                rel: None,
                described_by: None,
                title: Some(title),
                kind: None,
                hreflang: None,
            }),
        }
    }

    pub fn kind(self, kind: impl Into<String>) -> LinkBuilder<ObjectLinkData> {
        let kind = kind.into();
        LinkBuilder {
            data: ObjectLinkData(LinkObject {
                href: self.data.0,
                rel: None,
                described_by: None,
                title: None,
                kind: Some(kind),
                hreflang: None,
            }),
        }
    }

    pub fn hreflang(self, hreflang: impl Into<String>) -> LinkBuilder<ObjectLinkData> {
        let hreflang = hreflang.into();
        LinkBuilder {
            data: ObjectLinkData(LinkObject {
                href: self.data.0,
                rel: None,
                described_by: None,
                title: None,
                kind: None,
                hreflang: Some(hreflang),
            }),
        }
    }

    pub fn build(self) -> Link {
        Link(CoreLink::String(self.data.0))
    }
}

impl LinkBuilder<ObjectLinkData> {
    pub fn rel(self, rel: impl Into<String>) -> Self {
        let rel = rel.into();
        LinkBuilder {
            data: ObjectLinkData(LinkObject {
                rel: Some(rel),
                ..self.data.0
            }),
        }
    }

    pub fn described_by(self, link: impl Into<Link>) -> Self {
        let link = link.into();
        LinkBuilder {
            data: ObjectLinkData(LinkObject {
                described_by: Some(Box::new(link)),
                ..self.data.0
            }),
        }
    }

    pub fn title(self, title: impl Into<String>) -> Self {
        let title = title.into();
        LinkBuilder {
            data: ObjectLinkData(LinkObject {
                title: Some(title),
                ..self.data.0
            }),
        }
    }

    pub fn kind(self, kind: impl Into<String>) -> Self {
        let kind = kind.into();
        LinkBuilder {
            data: ObjectLinkData(LinkObject {
                kind: Some(kind),
                ..self.data.0
            }),
        }
    }

    pub fn hreflang(self, hreflang: impl Into<String>) -> Self {
        let hreflang = hreflang.into();
        LinkBuilder {
            data: ObjectLinkData(LinkObject {
                hreflang: Some(hreflang),
                ..self.data.0
            }),
        }
    }

    pub fn build(self) -> Link {
        Link(CoreLink::Object(self.data.0))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link(CoreLink);

impl Link {
    pub fn builder() -> LinkBuilder<NoLinkData> {
        LinkBuilder::default()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum CoreLink {
    String(String),
    Object(LinkObject),
}

impl From<&str> for Link {
    fn from(value: &str) -> Self {
        Link::builder().href(value).build()
    }
}

impl From<String> for Link {
    fn from(value: String) -> Self {
        Link::builder().href(value).build()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LinkObject {
    href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rel: Option<String>,
    #[serde(rename = "describedby", skip_serializing_if = "Option::is_none")]
    described_by: Option<Box<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hreflang: Option<String>,
}
