use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Line {
    timestamp: String,
    value: f64,
}
