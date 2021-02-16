use metalink::v3::Metalink;
use std::fs;
use strong_xml::XmlRead;
use test_generator::test_resources;

#[test_resources("tests/examples/*.metalink")]
fn parse_file(path: &str) {
    let file_contents = fs::read_to_string(path).unwrap();
    let res = Metalink::from_str(&file_contents);
    match res {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok(_metalink) => {
            // println!("{:#?}", metalink);
        }
    }
}
