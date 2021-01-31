extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::{from_str};
use metalink::Metalink4;

#[test]
fn simple_rfc() {
    let s = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <metalink xmlns="urn:ietf:params:xml:ns:metalink">
        <file name="example.ext">
            <size>14471447</size>
            <url>ftp://ftp.example.com/example.ext</url>
            <url>http://example.com/example.ext</url>
            <metaurl mediatype="torrent">
                http://example.com/example.ext.torrent</metaurl>
        </file>
    </metalink>
    "##;
    let metalink: Metalink4 = from_str(s).unwrap();
    println!("{:#?}", metalink);
}

#[test]
fn extended_rfc() {
    let s = r##"
    <?xml version="1.0" encoding="UTF-8"?>
 <metalink xmlns="urn:ietf:params:xml:ns:metalink">
   <published>2009-05-15T12:23:23Z</published>
   <file name="example.ext">
     <size>14471447</size>
     <identity>Example</identity>
     <version>1.0</version>
     <language>en</language>
     <description>
     A description of the example file for download.
     </description>
     <hash type="sha-256">f0ad929cd259957e160ea442eb80986b5f01...</hash>
     <url location="de"
          priority="1">ftp://ftp.example.com/example.ext</url>
     <url location="fr"
          priority="1">http://example.com/example.ext</url>
     <metaurl mediatype="torrent"
          priority="2">http://example.com/example.ext.torrent</metaurl>
   </file>
   <file name="example2.ext">
     <size>14471447</size>
     <identity>Example2</identity>
     <version>1.0</version>
     <language>en</language>
     <description>
     Another description for a second file.
     </description>
     <hash type="sha-256">2f548ce50c459a0270e85a7d63b2383c5523...</hash>
     <url location="de"
          priority="1">ftp://ftp.example.com/example2.ext</url>
     <url location="fr"
          priority="1">http://example.com/example2.ext</url>
     <metaurl mediatype="torrent"
          priority="2">http://example.com/example2.ext.torrent</metaurl>
   </file>
 </metalink>
    "##;
    let metalink: Metalink4 = from_str(s).unwrap();
    println!("{:#?}", metalink);
}