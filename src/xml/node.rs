use std::fmt;
use std::collections::HashMap;
use xml::*;

pub struct Node {
    pub id: Option<i64>,
    pub lat: Option<f32>,
    pub user: Option<String>,
    pub action: Option<Action>,
    pub uid: Option<i32>,
    pub lon: Option<f32>,
    pub modified: bool,
    pub tags: HashMap<String, String>,
    pub version: Option<i32>,
    pub parents: Vec<i64>
}

impl Generic for Node {
    fn new() -> Node {
        Node {
            id: None,
            lat: None,
            lon: None,
            user: None,
            uid: None,
            modified: false,
            action: None,
            tags: HashMap::new(),
            parents: Vec::new(),
            version: None
        }
    }

    fn value(&self) -> Value {
        Value::Node
    }

    fn set_tag(&mut self, k: String, v: String) {
        self.tags.insert(k, v);
    }

    fn has_tags(&self) -> bool {
        !self.tags.is_empty()
    }

    fn to_feat(&self) -> geojson::Feature {
        geojson::Feature {
            bbox: None,
            geometry: None,
            id: None,
            properties: None,
            foreign_members: None
        }
    }

    fn is_valid(&self) -> bool {
        match self.id {
            None => { return false; },
            Some(_) => ()
        }
        match self.lat {
            None => { return false; },
            Some(_) => ()
        }
        match self.lon {
            None => { return false; },
            Some(_) => ()
        }
        match self.version {
            None => { return false; },
            Some(_) => ()
        }

        return true;
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Node: id={}]", self.id.unwrap())
    }
}
