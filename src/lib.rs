extern crate serde;
extern crate serde_xml_rs;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename = "metalink")]
pub struct Metalink4 {
    pub generator: Option<String>,
    pub origin: Option<String>,
    pub published: Option<String>, // Date
    pub updated: Option<String>, // Date
    #[serde(rename = "file")]
    pub files: Vec<File>
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct File {
    pub name: String, // attr
    pub copyright: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "hash", default)]
    pub hashes: Vec<Hash>,
    pub identity: Option<String>,
    pub language: Option<Vec<String>>,
    pub logo: Option<String>, // uri
    #[serde(rename = "metaurl", default)]
    pub meta_urls: Vec<MetaUrl>,
    #[serde(rename = "os", default)]
    pub os: Vec<String>,
    #[serde(rename = "piece", default)]
    pub pieces: Vec<Piece>, //?
    pub publisher: Option<Publisher>,
    pub signature: Option<String>,
    pub size: Option<u64>,
    #[serde(rename = "url", default)]
    pub urls: Vec<Url>,
    pub version: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Piece {
    pub length: u64, // attr
    pub r#type: String, // attr
    #[serde(rename = "hash", default)]
    pub hashes: Vec<Hash>
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Hash {
    pub r#type: Option<String>, // attr
    #[serde(rename = "$value")]
    pub value: String
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MetaUrl {
    pub priority: Option<u32>, // attr
    pub mediatype: String, // attr
    pub name: Option<String>, // attr
    #[serde(rename = "$value")]
    pub value: String
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Origin {
    pub dynamic: Option<bool>,
    #[serde(rename = "$value")]
    pub value: String
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Publisher {
    pub name: String, //attr
    pub url: Option<String> //uri
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Signature {
    pub mediatype: String, //attr
    #[serde(rename = "$value")]
    pub value: String
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Url {
    pub location: Option<String>, //attr
    pub priority: Option<u32>, //attr
    #[serde(rename = "$value")]
    pub value: String
}
