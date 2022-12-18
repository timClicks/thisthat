use std::vec;

use serde::{self, Deserialize, Serialize};
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
        let garbage: Vec<&'static str> = vec!["a", "b", "c", "", "e"];

        Fixture {
            a: "hello there".to_string(),
            b: -10.0,
            c: 12312,
            d: garbage.iter().map(|x| x.to_string()).collect(),
            e: Some(Box::new(Fixture {
                a: "hello there".to_string(),
                b: -10.0,
                c: 12312,
                d: garbage.iter().map(|x| x.to_string()).collect(),
                e: None,
            })),
        }
    }
}

#[test]
fn todo() {
    // whelp
}
