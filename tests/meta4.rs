use std::fs;
use serde_xml_rs::{from_str};
use metalink::Metalink4;

#[test]
fn simple_from_rfc() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let s = fs::read_to_string("tests/examples/simple.meta4")?;
    let metalink: Metalink4 = from_str(&s)?;
    println!("{:#?}", metalink);
    Ok(())
}

#[test]
fn extended_from_rfc() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let s = fs::read_to_string("tests/examples/extended.meta4")?;
    let metalink: Metalink4 = from_str(&s)?;
    println!("{:#?}", metalink);
    Ok(())
}