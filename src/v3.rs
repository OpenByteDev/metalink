use chrono::NaiveDateTime;
use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};
use url::Url;

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "metalink")]
pub struct Metalink<'a> {
    #[xml(attr = "version")]
    pub metalink_version: Cow<'a, str>, // version
    #[xml(flatten_text = "version")]
    pub version: Option<Cow<'a, str>>, // version
    #[xml(child = "files")]
    pub files: Files<'a>,
    #[xml(attr = "origin")]
    pub origin: Option<Cow<'a, str>>,
    #[xml(attr = "type")]
    pub r#type: Option<Cow<'a, str>>, // Option<Type>
    #[xml(attr = "pubdate")]
    pub pubdate: Option<Cow<'a, str>>, // datetime
    #[xml(attr = "refreshdate")]
    pub refreshdate: Option<Cow<'a, str>>, // datetime
    #[xml(flatten_text = "generator")]
    pub generator: Option<Cow<'a, str>>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Type {
    Dynamic,
    Static,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "files")]
pub struct Files<'a> {
    #[xml(child = "file")]
    pub files: Vec<File<'a>>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "file")]
pub struct File<'a> {
    #[xml(child = "resources")]
    pub resources: Resources<'a>,
    #[xml(flatten_text = "identity")]
    pub identity: Option<Cow<'a, str>>,
    #[xml(flatten_text = "version")]
    pub version: Option<Cow<'a, str>>, // version
    #[xml(child = "verification")]
    pub verification: Option<Verification<'a>>,
    #[xml(flatten_text = "size")]
    pub size: Option<u64>,
    #[xml(flatten_text = "description")]
    pub description: Option<Cow<'a, str>>,
    #[xml(flatten_text = "logo")]
    pub logo: Option<Url>,
    #[xml(flatten_text = "tags")]
    pub tags: Option<Cow<'a, str>>, // comma seperated list of tags
    #[xml(flatten_text = "language")]
    pub language: Option<Cow<'a, str>>,
    #[xml(flatten_text = "os")]
    pub os: Option<Cow<'a, str>>,
    #[xml(flatten_text = "mimetype")]
    pub mimetype: Option<Cow<'a, str>>,
    #[xml(flatten_text = "relation")]
    pub relations: Vec<Cow<'a, str>>,
    #[xml(flatten_text = "releasedate")]
    pub releasedate: Option<NaiveDateTime>,
    #[xml(flatten_text = "changelog")]
    pub changelog: Option<Cow<'a, str>>,
    #[xml(child = "publisher")]
    pub publisher: Option<Publisher<'a>>,
    #[xml(flatten_text = "copyright")]
    pub copyright: Option<Cow<'a, str>>,
    #[xml(child = "license")]
    pub license: Option<License<'a>>,
    #[xml(child = "multimedia")]
    pub multimedia: Option<Multimedia<'a>>,
    #[xml(flatten_text = "screenshots")]
    pub screenshots: Vec<Url>,
    #[xml(flatten_text = "upgrade")]
    pub upgrade: Option<Cow<'a, str>>, // list of comma seperated upgrade actions: install, uninstall, reboot
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "verification")]
pub struct Verification<'a> {
    #[xml(child = "hash")]
    pub hashes: Vec<FileHash<'a>>,
    #[xml(child = "signature")]
    pub signature: Vec<FileSignature<'a>>,
    #[xml(child = "pieces")]
    pub pieces: Vec<PieceHashes<'a>>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "hash")]
pub struct FileHash<'a> {
    #[xml(attr = "type")]
    pub r#type: Cow<'a, str>, // limited to md4, md5, sha1, sha256, sha384, sha512, rmdl160, tiger, crc32
    #[xml(text)]
    pub value: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "signature")]
pub struct FileSignature<'a> {
    #[xml(attr = "type")]
    pub r#type: Cow<'a, str>,
    #[xml(attr = "file")]
    pub file: Cow<'a, str>,
    #[xml(text)]
    pub value: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "pieces")]
pub struct PieceHashes<'a> {
    #[xml(attr = "type")]
    pub r#type: Cow<'a, str>,
    #[xml(attr = "length")]
    pub length: u64,
    #[xml(child = "hash")]
    pub hashes: Vec<PieceHash<'a>>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "piece")]
pub struct PieceHash<'a> {
    #[xml(attr = "piece")]
    pub piece: u32,
    #[xml(text)]
    pub value: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "resources")]
pub struct Resources<'a> {
    #[xml(child = "url")]
    pub urls: Vec<FileUrl<'a>>,
    #[xml(flatten_text = "maxconnections")]
    pub max_connections: Option<u32>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "url")]
pub struct FileUrl<'a> {
    #[xml(attr = "type")]
    pub r#type: Option<Cow<'a, str>>, // limited to ftp, ftps, http, https, rsync, bittorrent, magnet, ed2k
    #[xml(attr = "preference")]
    pub preference: Option<u32>,
    #[xml(attr = "location")]
    pub location: Option<Cow<'a, str>>,
    #[xml(flatten_text = "maxconnections")]
    pub max_connections: Option<u32>,
    #[xml(text)]
    pub value: Url,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "publisher")]
pub struct Publisher<'a> {
    #[xml(flatten_text = "name")]
    pub name: Cow<'a, str>,
    #[xml(flatten_text = "url")]
    pub url: Option<Url>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "license")]
pub struct License<'a> {
    #[xml(flatten_text = "name")]
    pub name: Cow<'a, str>,
    #[xml(flatten_text = "url")]
    pub url: Option<Url>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "multimedia")]
pub struct Multimedia<'a> {
    #[xml(child = "audio")]
    pub audio: Vec<Audio<'a>>,
    #[xml(child = "video")]
    pub video: Vec<Video<'a>>,
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "audio")]
pub struct Audio<'a> {
    #[xml(flatten_text = "bitrate")]
    pub bitrate: Option<u32>,
    #[xml(flatten_text = "codec")]
    pub codec: Option<Cow<'a, str>>,
    #[xml(flatten_text = "album")]
    pub album: Option<Cow<'a, str>>,
    #[xml(flatten_text = "artist")]
    pub artist: Option<Cow<'a, str>>,
    #[xml(flatten_text = "duration")]
    pub duration: Option<Cow<'a, str>>,
    #[xml(flatten_text = "resolution")]
    pub resolution: Option<Cow<'a, str>>,
    // should probably be Map<String, String>
}

#[derive(XmlWrite, XmlRead, Eq, PartialEq, Debug, Clone)]
#[xml(tag = "video")]
pub struct Video<'a> {
    #[xml(flatten_text = "bitrate")]
    pub bitrate: Option<u32>,
    #[xml(flatten_text = "codec")]
    pub codec: Option<Cow<'a, str>>,
    #[xml(flatten_text = "album")]
    pub album: Option<Cow<'a, str>>,
    #[xml(flatten_text = "artist")]
    pub artist: Option<Cow<'a, str>>,
    #[xml(flatten_text = "duration")]
    pub duration: Option<Cow<'a, str>>,
    #[xml(flatten_text = "resolution")]
    pub resolution: Option<Cow<'a, str>>,
    // should probably be Map<String, String>
}

// Note: embedded torrent information is ignored
