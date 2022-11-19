use std::vec;

use serde::{self, Serialize, Deserialize};
use serde_json;


#[derive(Debug, Serialize, Deserialize)]
struct Fixture {
    a: String,
    b: f32,
    c: i32,
    d: Vec<String>,
    e: Option<Box<Fixture>>,
}

impl Fixture {
    fn new() -> Self {
        let garbage: Vec<String> = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "e".to_string(),
        ];

        Fixture {
            a: "hello there".to_string(),
            b: -10.0,
            c: 12312,
            d: garbage.clone(),
            e: Some(Box::new(Fixture {
                a: "hello there".to_string(),
                b: -10.0,
                c: 12312,
                d: garbage.clone(),
                e: None,
            }))
        }
    }
}

#[test]
fn todo() {
    // whelp
}