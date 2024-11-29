use serde::Serialize;

// 道具
#[derive(Clone, Debug, Serialize)]
pub struct Prop {
    pub duration: u64,
    pub attributes: [i8; 8],
}

impl Prop {
    pub fn new(duration: u64, attributes: [i8; 8]) -> Self {
        Prop {
            duration,
            attributes,
        }
    }
}