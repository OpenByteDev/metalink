use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};
use url::Url;

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "metalink")]
pub struct Metalink<'a> {
    #[xml(flatten_text = "generator")]
    pub generator: Option<Cow<'a, str>>,
    #[xml(flatten_text = "origin")]
    pub origin: Option<Cow<'a, str>>,
    #[xml(flatten_text = "published")]
    pub published: Option<Cow<'a, str>>, //datetime
    #[xml(flatten_text = "updated")]
    pub updated: Option<Cow<'a, str>>, //datetime
    #[xml(child = "file")]
    pub files: Vec<File<'a>>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "file")]
pub struct File<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>, // attr
    #[xml(flatten_text = "copyright")]
    pub copyright: Option<Cow<'a, str>>,
    #[xml(flatten_text = "description")]
    pub description: Option<Cow<'a, str>>,
    #[xml(child = "hash")]
    pub hashes: Vec<Hash<'a>>,
    #[xml(flatten_text = "identity")]
    pub identity: Option<Cow<'a, str>>,
    #[xml(flatten_text = "language")]
    pub language: Vec<Cow<'a, str>>,
    #[xml(flatten_text = "logo")]
    pub logo: Option<Url>, // uri
    #[xml(child = "metaurl")]
    pub meta_urls: Vec<FileMetaUrl<'a>>,
    #[xml(flatten_text = "os")]
    pub os: Vec<Cow<'a, str>>,
    #[xml(child = "piece")]
    pub pieces: Vec<Piece<'a>>,
    #[xml(child = "publisher")]
    pub publisher: Option<Publisher<'a>>,
    #[xml(flatten_text = "signature")]
    pub signature: Option<Cow<'a, str>>,
    #[xml(flatten_text = "size")]
    pub size: Option<u64>,
    #[xml(child = "url")]
    pub urls: Vec<FileUrl<'a>>,
    #[xml(flatten_text = "version")]
    pub version: Option<Cow<'a, str>>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "piece")]
pub struct Piece<'a> {
    #[xml(attr = "length")]
    pub length: u64, // attr
    #[xml(attr = "type")]
    pub r#type: Cow<'a, str>, // attr
    #[xml(child = "hash")]
    pub hashes: Vec<Hash<'a>>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "hash")]
pub struct Hash<'a> {
    #[xml(attr = "type")]
    pub r#type: Option<Cow<'a, str>>, // attr
    #[xml(text)]
    pub value: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "metaurl")]
pub struct FileMetaUrl<'a> {
    #[xml(attr = "priority")]
    pub priority: Option<u32>, // attr
    #[xml(attr = "mediatype")]
    pub mediatype: Cow<'a, str>, // attr
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>, // attr
    #[xml(text)]
    pub value: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "origin")]
pub struct Origin<'a> {
    #[xml(flatten_text = "dynamic")]
    pub dynamic: Option<bool>,
    #[xml(text)]
    pub value: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "publisher")]
pub struct Publisher<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>, //attr
    #[xml(flatten_text = "url")]
    pub url: Option<Url>, //url
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "signature")]
pub struct Signature<'a> {
    #[xml(attr = "mediatype")]
    pub mediatype: Cow<'a, str>, //attr
    #[xml(text)]
    pub value: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "url")]
pub struct FileUrl<'a> {
    #[xml(attr = "location")]
    pub location: Option<Cow<'a, str>>, //attr
    #[xml(attr = "priority")]
    pub priority: Option<u32>, //attr
    #[xml(text)]
    pub value: Cow<'a, str>,
}
