mod node;
mod way;
mod rel;
mod tree;

use crate::feature;
use crate::osm::node::Node;
use crate::osm::way::Way;
use crate::osm::rel::Rel;
use crate::osm::tree::OSMTree;

use std::string;
use std::num;
use std::io::Cursor;
use quick_xml::events as XMLEvents;
use quick_xml::{Reader, Writer};
use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum XMLError {
    Unknown,
    Invalid,
    GCNotSupported,
    EncodingFailed,
    InternalError(String),
    ParsingError,
    InvalidNode(String),
    InvalidNodeRef,
    InvalidWay(String),
    InvalidWayRef,
    InvalidRel(String),
    InvalidXML,
    InvalidFeature,
    NotFoundError,
    StringParsing(string::ParseError),
    IntParsing(num::ParseIntError),
    FloatParsing(num::ParseFloatError)
}

impl XMLError {
    pub fn to_string(&self) -> String {
        match *self {
            XMLError::Unknown => String::from("Unknown Error"),
            XMLError::GCNotSupported => String::from("GeometryCollection are not currently supported"),
            XMLError::Invalid => String::from("Could not parse XML - Invalid"),
            XMLError::EncodingFailed => String::from("Encoding Failed"),
            XMLError::InternalError(ref msg) => format!("Internal Error: {}", msg),
            XMLError::ParsingError => String::from("Parsing Error"),
            XMLError::InvalidNode(ref msg) => format!("Invalid Node: {}", msg),
            XMLError::InvalidNodeRef => String::from("Invalid Node Reference"),
            XMLError::InvalidWay(ref msg) => format!("Invalid Way: {}", msg),
            XMLError::InvalidWayRef => String::from("Invalid Way Reference"),
            XMLError::InvalidRel(ref msg) => format!("Invalid Relation: {}", msg),
            XMLError::InvalidXML => String::from("Invalid XML"),
            XMLError::NotFoundError => String::from("Not Found"),
            XMLError::InvalidFeature => String::from("Invalid Feature"),
            XMLError::StringParsing(_) => String::from("Could not parse attribute to string"),
            XMLError::IntParsing(_) => String::from("Could not parse attribute to integer"),
            XMLError::FloatParsing(_) => String::from("Could not parse attribute to float")
        }
    }
}

impl From<string::FromUtf8Error> for XMLError {
    fn from(_error: string::FromUtf8Error) -> XMLError {
        XMLError::ParsingError
    }
}

impl From<quick_xml::Error> for XMLError {
    fn from(_error: quick_xml::Error) -> XMLError {
        XMLError::InvalidXML
    }
}

impl From<string::ParseError> for XMLError {
    fn from(error: string::ParseError) -> XMLError {
        XMLError::StringParsing(error)
    }
}

impl From<num::ParseFloatError> for XMLError {
    fn from(error: num::ParseFloatError) -> XMLError {
        XMLError::FloatParsing(error)
    }
}

impl From<num::ParseIntError> for XMLError {
    fn from(error: num::ParseIntError) -> XMLError {
        XMLError::IntParsing(error)
    }
}

pub fn unescape(prop: String) -> String {
    prop.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&#39;", "'")
}

#[derive(PartialEq, Debug)]
pub enum Value {
    None,
    Node,
    Way,
    Rel
}

#[derive(PartialEq, Debug, Clone)]
pub enum Action {
    None,
    Create,
    Modify,
    Delete
}

pub trait Generic {
    fn new() -> Self;
    fn value(&self) -> Value;
    fn set_tag(&mut self, k: String, v: String);
    fn has_tags(&self) -> bool;
    fn to_feat(&self, tree: &OSMTree) -> Result<geojson::Feature, XMLError>;
    fn is_valid(&self) -> Result<bool, String>;
}

pub struct OSMTypes {
    node_it: i64,
    way_it: i64,
    nodes: String,
    ways: String,
    rels: String
}

impl OSMTypes {
    pub fn new() -> OSMTypes {
        OSMTypes {
            node_it: 7_000_000_000_000_000_000,
            way_it: 8_000_000_000_000_000_000,
            nodes: String::from(""),
            ways: String::from(""),
            rels: String::from("")
        }
    }
}

pub fn to_diffresult(ids: HashMap<i64, feature::Response>, tree: OSMTree) -> Result<String, XMLError> {
    let mut diffres = String::from(r#"<diffResult generator="Hecate Server" version="0.6">"#);

    for n in tree.get_nodes().values() {
        if n.action == Some(Action::Create) {
            if let Some(diffid) = ids.get(&n.id.unwrap()) {
                diffres.push_str(&*format!(r#"<node old_id="{}" new_id="{}" new_version="{}"/>"#, diffid.old.unwrap(), diffid.new.unwrap(), diffid.version.unwrap()));
            }
        } else if n.action == Some(Action::Modify) {
            if let Some(diffid) = ids.get(&n.id.unwrap()) {
                diffres.push_str(&*format!(r#"<node old_id="{}" new_id="{}" new_version="{}"/>"#, n.id.unwrap(), n.id.unwrap(), diffid.version.unwrap()));
            }
        } else if n.action == Some(Action::Delete) && ids.get(&n.id.unwrap()).is_some() {
            diffres.push_str(&*format!(r#"<node old_id="{}"/>"#, n.id.unwrap()));
        }
    }

    for w in tree.get_ways().values() {
        if w.action == Some(Action::Create) {
            if let Some(diffid) = ids.get(&w.id.unwrap()) {
                diffres.push_str(&*format!(r#"<way old_id="{}" new_id="{}" new_version="{}"/>"#, diffid.old.unwrap(), diffid.new.unwrap(), diffid.version.unwrap()));
            }
        } else if w.action == Some(Action::Modify) {
            if let Some(diffid) = ids.get(&w.id.unwrap()) {
                diffres.push_str(&*format!(r#"<way old_id="{}" new_id="{}" new_version="{}"/>"#, w.id.unwrap(), w.id.unwrap(), diffid.version.unwrap()));
            }
        } else if w.action == Some(Action::Delete) && ids.get(&w.id.unwrap()).is_some() {
            diffres.push_str(&*format!(r#"<way old_id="{}"/>"#, w.id.unwrap()));
        }
    }

    for _ in tree.get_rels().values() {

    }

    diffres.push_str(r#"</diffResult>"#);

    Ok(diffres)
}

pub fn to_delta_tag(xml_node: &quick_xml::events::BytesStart, map: &mut HashMap<String, Option<String>>) { let mut kv: (Option<String>, Option<String>) = (None, None);
    for attr in xml_node.attributes() {
        let attr = attr.unwrap();

        match attr.key {
            b"k"  => kv.0 = Some(String::from_utf8(attr.value.into_owned()).unwrap()),
            b"v"  => kv.1 = Some(String::from_utf8(attr.value.into_owned()).unwrap()),
            _ => ()
        }
    }

    map.insert(kv.0.unwrap(), kv.1);
}

pub fn to_delta(body: &str) -> Result<HashMap<String, Option<String>>, XMLError> {
    let mut reader = Reader::from_str(body);
    let mut buf = Vec::new();

    let mut map = HashMap::new();

     loop {
        match reader.read_event(&mut buf) {
            Ok(XMLEvents::Event::Start(ref e)) => {
                if let b"tag" = e.name() {
                    to_delta_tag(&e, &mut map)
                }
            },
            Ok(XMLEvents::Event::Empty(ref e)) => {
                if let b"tag" = e.name() {
                    to_delta_tag(&e, &mut map)
                }
            },
            Ok(XMLEvents::Event::Eof) => { break; },
            Err(_) => { return Err(XMLError::Invalid); },
            _ => ()
        }

        buf.clear()
    }

    Ok(map)
}

pub fn to_features(body: &str) -> Result<(geojson::FeatureCollection, OSMTree), XMLError> {
    let tree = tree_parser(&body)?;

    let mut fc = geojson::FeatureCollection {
        bbox: None,
        features: vec![],
        foreign_members: None
    };

    for rel in tree.get_rels().values() {
        if rel.action != Some(Action::Delete) && !rel.has_tags() { continue; }

        fc.features.push(rel.to_feat(&tree)?);
    }

    for way in tree.get_ways().values() {
        if way.action != Some(Action::Delete) && !way.has_tags() { continue; }

        fc.features.push(way.to_feat(&tree)?);
    }

    for node in tree.get_nodes().values() {
        if node.action != Some(Action::Delete) && !node.has_tags() { continue; }

        let n = node.to_feat(&tree)?;

        fc.features.push(n);
    }

    Ok((fc, tree))
}

pub fn tree_parser(body: &str) -> Result<OSMTree, XMLError> {
    let mut tree: OSMTree = OSMTree::default();

    let mut opening_osm = false;
    let mut node: Node = Node::new();
    let mut way: Way = Way::new();
    let mut rel: Rel = Rel::new();

    let mut current_value = Value::None;
    let mut current_action = Action::None;

    let mut reader = Reader::from_str(body);
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(XMLEvents::Event::Start(ref e)) => {
                match e.name() {
                    b"osmChange" => {
                        parse_osm(&e, &mut tree.meta)?;
                        opening_osm = true;
                    },
                    b"create" => {
                        if current_action != Action::None { return Err(XMLError::InternalError(String::from("Action Already Specialized"))); }
                        current_action = Action::Create;
                    },
                    b"modify" => {
                        if current_action != Action::None { return Err(XMLError::InternalError(String::from("Action Already Specialized"))); }
                        current_action = Action::Modify;
                    },
                    b"delete" => {
                        if current_action != Action::None { return Err(XMLError::InternalError(String::from("Action Already Specialized"))); }
                        current_action = Action::Delete;
                    },
                    b"node" => {
                        if current_action == Action::None { return Err(XMLError::InternalError(String::from("node must be in Action"))); }
                        if current_value != Value::None { return Err(XMLError::InternalError(String::from("node cannot be within another value"))); }

                        node = parse_node(e)?;
                        node.action = Some(current_action.clone());

                        if node.action == Some(Action::Create) {
                            node.version = Some(1);
                        }

                        current_value = Value::Node;
                    },
                    b"way" => {
                        if current_action == Action::None { return Err(XMLError::InternalError(String::from("way must be in Action"))); }
                        if current_value != Value::None { return Err(XMLError::InternalError(String::from("way cannot be within another value"))); }

                        way = parse_way(e)?;
                        way.action = Some(current_action.clone());

                        if way.action == Some(Action::Create) {
                            way.version = Some(1);
                        }

                        current_value = Value::Way;
                    },
                    b"relation" => {
                        if current_action == Action::None { return Err(XMLError::InternalError(String::from("rel must be in Action"))); }
                        if current_value != Value::None { return Err(XMLError::InternalError(String::from("rel cannot be within another value"))); }

                        rel = parse_rel(e)?;
                        rel.action = Some(current_action.clone());

                        if rel.action == Some(Action::Create) {
                            rel.version = Some(1);
                        }

                        current_value = Value::Rel;
                    },
                    b"tag" => {
                        let (k, v) = parse_tag(&e)?;

                        match current_value {
                            Value::None => { return Err(XMLError::InternalError(String::from("tags must be in value"))); },
                            Value::Node => {
                                node.set_tag(k, v);
                            },
                            Value::Way => {
                                way.set_tag(k, v);
                            },
                            Value::Rel => {
                                rel.set_tag(k, v);
                            }
                        };

                    }
                    _ => (),
                }
            },
            Ok(XMLEvents::Event::Empty(ref e)) => {
                match e.name() {
                    b"node" => {
                        if current_action == Action::None { return Err(XMLError::InternalError(String::from("node must be in Action"))); }
                        if current_value != Value::None { return Err(XMLError::InternalError(String::from("node cannot be within another value"))); }

                        node = parse_node(&e)?;
                        node.action = Some(current_action.clone());

                        if node.action == Some(Action::Create) {
                            node.version = Some(1);
                        }

                        tree.add_node(node)?;

                        node = Node::new();
                    },
                    b"way" => {
                        return Err(XMLError::InternalError(String::from("ways cannot be self closing")));
                    },
                    b"relation" => {
                        return Err(XMLError::InternalError(String::from("rels cannot be self closing")));
                    },
                    b"nd" => {
                        if current_value != Value::Way { return Err(XMLError::InternalError(String::from("nd must be in way"))); }

                        let ndref = parse_nd(&e)?;
                        way.nodes.push(ndref);
                    },
                    b"tag" => {
                        let (k, v) = parse_tag(&e)?;

                        match current_value {
                            Value::None => { return Err(XMLError::InternalError(String::from("tags must be in value"))); },
                            Value::Node => {
                                node.set_tag(k, v);
                            },
                            Value::Way => {
                                way.set_tag(k, v);
                            },
                            Value::Rel => {
                                rel.set_tag(k, v);
                            }
                        };
                    },
                    b"member" => {
                        let (rtype, rref, rrole) = parse_member(&e)?;

                        match current_value {
                            Value::Rel => {
                                rel.set_member(rtype, rref, rrole);
                            },
                            _ => { return Err(XMLError::InternalError(String::from("member must be in rel"))); }
                        };

                    }
                    _ => ()
                }
            },
            Ok(XMLEvents::Event::End(ref e)) => {
                match e.name() {
                    b"node" => {
                        if current_value != Value::Node { return Err(XMLError::InternalError(String::from("node close outside of node"))); }
                        tree.add_node(node)?;
                        node = Node::new();
                        current_value = Value::None;
                    },
                    b"way" => {
                        if current_value != Value::Way { return Err(XMLError::InternalError(String::from("way close outside of node"))); }
                        tree.add_way(way)?;
                        way = Way::new();
                        current_value = Value::None;
                    },
                    b"relation" => {
                        if current_value != Value::Rel { return Err(XMLError::InternalError(String::from("rel close outside of node"))); }
                        tree.add_rel(rel)?;
                        rel = Rel::new();
                        current_value = Value::None;
                    },
                    b"create" => {
                        if current_action != Action::Create { return Err(XMLError::InternalError(String::from("create close outside of create"))); }
                        current_action = Action::None;
                    },
                    b"modify" => {
                        if current_action != Action::Modify { return Err(XMLError::InternalError(String::from("modify close outside of create"))); }
                        current_action = Action::None;
                    },
                    b"delete" => {
                        if current_action != Action::Delete { return Err(XMLError::InternalError(String::from("delete close outside of create"))); }
                        current_action = Action::None;
                    },
                    b"osmChange" => {
                        if current_value != Value::None { return Err(XMLError::InternalError(String::from("All values must be finished before osm close"))); }
                        if !opening_osm { return Err(XMLError::InternalError(String::from("osm close outside of osm"))); }

                        return Ok(tree);
                    }
                    _ => ()
                }
            }
            Ok(XMLEvents::Event::Eof) => { return Err(XMLError::Invalid); },
            Err(_) => { return Err(XMLError::Invalid); },
            _ => ()
        }

        buf.clear();
    }
}

pub fn from_features(fc: &geojson::FeatureCollection) -> Result<String, XMLError> {
    let mut xml: String = String::from(r#"<?xml version="1.0" encoding="UTF-8"?><osm version="0.6" generator="ROSM">"#);
    let mut osm = OSMTypes::new();

    for feat in &fc.features {
        match feat.geometry {
            Some(ref geom) => {
                match geom.value {
                    geojson::Value::Point(ref coords) => {
                        point(&feat, &coords, &mut osm)?;
                    },
                    geojson::Value::MultiPoint(ref coords) => {
                        multipoint(&feat, &coords, &mut osm)?;
                    },
                    geojson::Value::LineString(ref coords) => {
                        linestring(&feat, &coords, &mut osm)?;
                    },
                    geojson::Value::MultiLineString(ref coords) => {
                        multilinestring(&feat, &coords, &mut osm)?;
                    },
                    geojson::Value::Polygon(ref coords) => {
                        polygon(&feat, &coords, &mut osm)?;
                    },
                    geojson::Value::MultiPolygon(ref coords) => {
                        multipolygon(&feat, &coords, &mut osm)?;
                    },
                    _ => { return Err(XMLError::GCNotSupported); }
                }
            },
            None => { return Err(XMLError::Unknown); }
        }
    }

    xml.push_str(&*osm.nodes);
    xml.push_str(&*osm.ways);
    xml.push_str(&*osm.rels);
    xml.push_str("</osm>");

    Ok(xml)
}

pub fn point(feat: &geojson::Feature, coords: &geojson::PointType, osm: &mut OSMTypes) -> Result<bool, XMLError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut xml_node = XMLEvents::BytesStart::owned(b"node".to_vec(), 4);

    xml_node.push_attribute(("id", &*feature::get_id(feat).unwrap().to_string()));
    xml_node.push_attribute(("version", &*feature::get_version(feat).unwrap().to_string()));
    xml_node.push_attribute(("lon", &*coords[0].to_string()));
    xml_node.push_attribute(("lat", &*coords[1].to_string()));

    writer.write_event(XMLEvents::Event::Start(xml_node)).unwrap();

    match feat.properties {
        Some(ref props) => {
            for (k, v) in props.iter() {
                let mut xml_tag = XMLEvents::BytesStart::owned(b"tag".to_vec(), 3);
                xml_tag.push_attribute(("k", k.as_str()));

                xml_tag.push_attribute(("v", &*json2str(&v)));

                writer.write_event(XMLEvents::Event::Empty(xml_tag)).unwrap();
            }
        },
        None => { return Err(XMLError::Unknown); }
    };

    writer.write_event(XMLEvents::Event::End(XMLEvents::BytesEnd::borrowed(b"node"))).unwrap();

    osm.nodes.push_str(&*String::from_utf8(writer.into_inner().into_inner()).unwrap());

    Ok(true)
}

pub fn multipoint(feat: &geojson::Feature, coords: &Vec<geojson::PointType>, osm: &mut OSMTypes) -> Result<bool, XMLError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut xml_rel = XMLEvents::BytesStart::owned(b"relation".to_vec(), 8);
    xml_rel.push_attribute(("id", &*feature::get_id(feat).unwrap().to_string()));
    xml_rel.push_attribute(("version", &*feature::get_version(feat).unwrap().to_string()));

    writer.write_event(XMLEvents::Event::Start(xml_rel)).unwrap();

    match feat.properties {
        Some(ref props) => {
            for (k, v) in props.iter() {
                let mut xml_tag = XMLEvents::BytesStart::owned(b"tag".to_vec(), 3);
                xml_tag.push_attribute(("k", k.as_str()));

                xml_tag.push_attribute(("v", &*json2str(&v)));
                writer.write_event(XMLEvents::Event::Empty(xml_tag)).unwrap();
            }
        },
        None => { return Err(XMLError::Unknown); }
    };

    let mut xml_tag = XMLEvents::BytesStart::owned(b"tag".to_vec(), 3);
    xml_tag.push_attribute(("k", "type"));
    xml_tag.push_attribute(("v", "multipoint"));
    writer.write_event(XMLEvents::Event::Empty(xml_tag)).unwrap();

    for nd in coords {
        let node_id = match add_node(&nd, osm) {
            Ok(node) => node,
            Err(_) => { return Err(XMLError::EncodingFailed); }
        };

        let mut xml_mem = XMLEvents::BytesStart::owned(b"member".to_vec(), 6);
        xml_mem.push_attribute(("ref", &*node_id.to_string()));
        xml_mem.push_attribute(("type", "node"));
        xml_mem.push_attribute(("role", "point"));
        writer.write_event(XMLEvents::Event::Empty(xml_mem)).unwrap();
    }

    writer.write_event(XMLEvents::Event::End(XMLEvents::BytesEnd::borrowed(b"relation"))).unwrap();

    osm.rels.push_str(&*String::from_utf8(writer.into_inner().into_inner()).unwrap());

    Ok(true)
}

pub fn linestring(feat: &geojson::Feature, coords: &geojson::LineStringType, osm: &mut OSMTypes) -> Result<bool, XMLError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut xml_way = XMLEvents::BytesStart::owned(b"way".to_vec(), 3);
    xml_way.push_attribute(("id", &*feature::get_id(feat).unwrap().to_string()));
    xml_way.push_attribute(("version", &*feature::get_version(feat).unwrap().to_string()));

    writer.write_event(XMLEvents::Event::Start(xml_way)).unwrap();

    let mut n_refs: Vec<i64> = Vec::new();

    for nd in coords {
        let n_ref: i64;

        if n_refs.len() > 1 && *nd == coords[0] {
            n_ref = n_refs[0];
        } else {
            let node_id = match add_node(&nd, osm) {
                Ok(node) => node,
                Err(_) => { return Err(XMLError::EncodingFailed); }
            };

            n_ref = node_id;
        }

        n_refs.push(n_ref);
    }

    for n_ref in n_refs {
        let mut xml_nd = XMLEvents::BytesStart::owned(b"nd".to_vec(), 2);
        xml_nd.push_attribute(("ref", &*n_ref.to_string()));
        writer.write_event(XMLEvents::Event::Empty(xml_nd)).unwrap();
    }

    match feat.properties {
        Some(ref props) => {
            for (k, v) in props.iter() {
                let mut xml_tag = XMLEvents::BytesStart::owned(b"tag".to_vec(), 3);
                xml_tag.push_attribute(("k", k.as_str()));

                xml_tag.push_attribute(("v", &*json2str(&v)));
                writer.write_event(XMLEvents::Event::Empty(xml_tag)).unwrap();
            }
        },
        None => { return Err(XMLError::Unknown); }
    };

    writer.write_event(XMLEvents::Event::End(XMLEvents::BytesEnd::borrowed(b"way"))).unwrap();

    osm.ways.push_str(&*String::from_utf8(writer.into_inner().into_inner()).unwrap());

    Ok(true)
}

pub fn multilinestring(feat: &geojson::Feature, coords: &Vec<geojson::LineStringType>, osm: &mut OSMTypes) -> Result<bool, XMLError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut xml_rel = XMLEvents::BytesStart::owned(b"relation".to_vec(), 8);
    xml_rel.push_attribute(("id", &*feature::get_id(feat).unwrap().to_string()));
    xml_rel.push_attribute(("version", &*feature::get_version(feat).unwrap().to_string()));

    writer.write_event(XMLEvents::Event::Start(xml_rel)).unwrap();

    match feat.properties {
        Some(ref props) => {
            for (k, v) in props.iter() {
                let mut xml_tag = XMLEvents::BytesStart::owned(b"tag".to_vec(), 3);
                xml_tag.push_attribute(("k", k.as_str()));

                xml_tag.push_attribute(("v", &*json2str(&v)));
                writer.write_event(XMLEvents::Event::Empty(xml_tag)).unwrap();
            }
        },
        None => { return Err(XMLError::Unknown); }
    };

    let mut xml_tag = XMLEvents::BytesStart::owned(b"tag".to_vec(), 3);
    xml_tag.push_attribute(("k", "type"));
    xml_tag.push_attribute(("v", "multilinestring"));
    writer.write_event(XMLEvents::Event::Empty(xml_tag)).unwrap();

    for ln in coords {
        let way_id = match add_way(&ln, osm) {
            Ok(way) => way,
            Err(_) => { return Err(XMLError::EncodingFailed); }
        };

        let mut xml_mem = XMLEvents::BytesStart::owned(b"member".to_vec(), 6);
        xml_mem.push_attribute(("ref", &*way_id.to_string()));

        xml_mem.push_attribute(("role", "line"));
        xml_mem.push_attribute(("type", "way"));

        writer.write_event(XMLEvents::Event::Empty(xml_mem)).unwrap();
    }

    writer.write_event(XMLEvents::Event::End(XMLEvents::BytesEnd::borrowed(b"relation"))).unwrap();

    osm.rels.push_str(&*String::from_utf8(writer.into_inner().into_inner()).unwrap());

    Ok(true)
}

pub fn polygon(feat: &geojson::Feature, coords: &geojson::PolygonType, osm: &mut OSMTypes) -> Result<bool, XMLError> {
    if coords.len() == 1 {
        let coords = vec!(coords[0].clone());

        return Ok(linestring(&feat, &coords[0], osm)?);
    }

    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut xml_rel = XMLEvents::BytesStart::owned(b"relation".to_vec(), 8);
    xml_rel.push_attribute(("id", &*feature::get_id(feat).unwrap().to_string()));
    xml_rel.push_attribute(("version", &*feature::get_version(feat).unwrap().to_string()));

    writer.write_event(XMLEvents::Event::Start(xml_rel)).unwrap();

    match feat.properties {
        Some(ref props) => {
            for (k, v) in props.iter() {
                let mut xml_tag = XMLEvents::BytesStart::owned(b"tag".to_vec(), 3);
                xml_tag.push_attribute(("k", k.as_str()));

                xml_tag.push_attribute(("v", &*json2str(&v)));
                writer.write_event(XMLEvents::Event::Empty(xml_tag)).unwrap();
            }
        },
        None => { return Err(XMLError::Unknown); }
    };

    let mut xml_tag = XMLEvents::BytesStart::owned(b"tag".to_vec(), 3);
    xml_tag.push_attribute(("k", "type"));
    xml_tag.push_attribute(("v", "multipolygon"));
    writer.write_event(XMLEvents::Event::Empty(xml_tag)).unwrap();

    for (poly_it, poly) in coords.iter().enumerate() {
        let way_id = match add_way(&poly, osm) {
            Ok(way) => way,
            Err(_) => { return Err(XMLError::EncodingFailed); }
        };

        let mut xml_mem = XMLEvents::BytesStart::owned(b"member".to_vec(), 6);
        xml_mem.push_attribute(("ref", &*way_id.to_string()));

        if poly_it == 0 {
            xml_mem.push_attribute(("role", "outer"));
        } else {
            xml_mem.push_attribute(("role", "inner"));
        }

        xml_mem.push_attribute(("type", "way"));
        writer.write_event(XMLEvents::Event::Empty(xml_mem)).unwrap();
    }

    writer.write_event(XMLEvents::Event::End(XMLEvents::BytesEnd::borrowed(b"relation"))).unwrap();

    osm.rels.push_str(&*String::from_utf8(writer.into_inner().into_inner()).unwrap());

    Ok(true)
}

pub fn multipolygon(feat: &geojson::Feature, coords: &Vec<geojson::PolygonType>, osm: &mut OSMTypes) -> Result<bool, XMLError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut xml_rel = XMLEvents::BytesStart::owned(b"relation".to_vec(), 8);
    xml_rel.push_attribute(("id", &*feature::get_id(feat).unwrap().to_string()));
    xml_rel.push_attribute(("version", &*feature::get_version(feat).unwrap().to_string()));

    writer.write_event(XMLEvents::Event::Start(xml_rel)).unwrap();

    match feat.properties {
        Some(ref props) => {
            for (k, v) in props.iter() {
                let mut xml_tag = XMLEvents::BytesStart::owned(b"tag".to_vec(), 3);
                xml_tag.push_attribute(("k", k.as_str()));

                xml_tag.push_attribute(("v", &*json2str(&v)));
                writer.write_event(XMLEvents::Event::Empty(xml_tag)).unwrap();
            }
        },
        None => { return Err(XMLError::Unknown); }
    };

    let mut xml_tag = XMLEvents::BytesStart::owned(b"tag".to_vec(), 3);
    xml_tag.push_attribute(("k", "type"));
    xml_tag.push_attribute(("v", "multipolygon"));
    writer.write_event(XMLEvents::Event::Empty(xml_tag)).unwrap();

    for polys in coords {
        for (poly_it, poly) in polys.iter().enumerate() {
            let way_id = match add_way(&poly, osm) {
                Ok(way) => way,
                Err(_) => { return Err(XMLError::EncodingFailed); }
            };

            let mut xml_mem = XMLEvents::BytesStart::owned(b"member".to_vec(), 6);
            xml_mem.push_attribute(("ref", &*way_id.to_string()));

            if poly_it == 0 {
                xml_mem.push_attribute(("role", "outer"));
            } else {
                xml_mem.push_attribute(("role", "inner"));
            }

            xml_mem.push_attribute(("type", "way"));
            writer.write_event(XMLEvents::Event::Empty(xml_mem)).unwrap();
        }
    }

    writer.write_event(XMLEvents::Event::End(XMLEvents::BytesEnd::borrowed(b"relation"))).unwrap();

    osm.rels.push_str(&*String::from_utf8(writer.into_inner().into_inner()).unwrap());

    Ok(true)

}

pub fn json2str(v: &serde_json::value::Value) -> String {
    match *v {
        serde_json::value::Value::Null => String::from(""),
        serde_json::value::Value::Bool(ref json_bool) => {
            if *json_bool {
                String::from("yes")
            } else {
                String::from("no")
            }
        },
        serde_json::value::Value::Number(ref json_num) => format!("{}", json_num),
        serde_json::value::Value::String(ref json_str) => json_str.to_string(),
        _ => v.to_string()
    }
}

pub fn add_way(coords: &geojson::LineStringType, osm: &mut OSMTypes) -> Result<i64, XMLError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut xml_way = XMLEvents::BytesStart::owned(b"way".to_vec(), 3);

    osm.way_it += 1;
    let id = osm.way_it;

    xml_way.push_attribute(("id", &*id.to_string()));
    xml_way.push_attribute(("version", "1"));
    writer.write_event(XMLEvents::Event::Start(xml_way)).unwrap();

    let mut n_refs: Vec<i64> = Vec::new();

    for nd in coords {
        let n_ref: i64;

        if n_refs.len() > 1 && *nd == coords[0] {
            n_ref = n_refs[0];
        } else {
            let node_id = match add_node(&nd, osm) {
                Ok(node) => node,
                Err(_) => { return Err(XMLError::EncodingFailed); }
            };

            n_ref = node_id;
        }

        n_refs.push(n_ref);
    }

    for n_ref in n_refs {
        let mut xml_nd = XMLEvents::BytesStart::owned(b"nd".to_vec(), 2);
        xml_nd.push_attribute(("ref", &*n_ref.to_string()));
        writer.write_event(XMLEvents::Event::Empty(xml_nd)).unwrap();
    }

    writer.write_event(XMLEvents::Event::End(XMLEvents::BytesEnd::borrowed(b"way"))).unwrap();

    let way = String::from_utf8(writer.into_inner().into_inner()).unwrap();
    osm.ways.push_str(&*way);

    Ok(id)
}


pub fn add_node(coords: &geojson::PointType, osm: &mut OSMTypes) -> Result<i64, XMLError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut xml_node = XMLEvents::BytesStart::owned(b"node".to_vec(), 4);

    osm.node_it +=  1;
    let id = osm.node_it;

    xml_node.push_attribute(("id", &*id.to_string()));
    xml_node.push_attribute(("version", "1"));
    xml_node.push_attribute(("lat", &*coords[1].to_string()));
    xml_node.push_attribute(("lon", &*coords[0].to_string()));

    writer.write_event(XMLEvents::Event::Empty(xml_node)).unwrap();

    let node = String::from_utf8(writer.into_inner().into_inner()).unwrap();
    osm.nodes.push_str(&*node);

    Ok(id)
}

pub fn parse_osm(xml_node: &XMLEvents::BytesStart, meta: &mut HashMap<String, String>) -> Result<bool, XMLError> {
    for attr in xml_node.attributes() {
        let attr = attr?;

        let key = String::from_utf8_lossy(attr.key);
        let val = String::from_utf8(attr.value.into_owned()).unwrap();

        meta.insert(key.to_string(), val.to_string());
    }

    if !meta.contains_key("version") { return Err(XMLError::InternalError(String::from("version required"))); }

    let v: f32 = match meta.get("version") {
        Some(ver) => ver.parse()?,
        None => { return Err(XMLError::InternalError(String::from("version required"))); }
    };

    if relative_ne!(v, 0.6) { return Err(XMLError::InternalError(String::from("api only supports 0.6"))); }

    Ok(true)
}

pub fn parse_node(xml_node: &XMLEvents::BytesStart) -> Result<Node, XMLError> {
    let mut node = Node::new();

    for attr in xml_node.attributes() {
        let attr = attr?;

        match attr.key {
            b"id" => node.id = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"lat" => node.lat = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"lon" => node.lon = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"user" => node.user = Some(String::from_utf8(attr.value.into_owned()).unwrap()),
            b"uid" => node.uid = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"version" => node.version = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            _ => ()
        }
    }

    Ok(node)
}

pub fn parse_way(xml_node: &XMLEvents::BytesStart) -> Result<Way, XMLError> {
    let mut way = Way::new();

    for attr in xml_node.attributes() {
        let attr = attr?;

        match attr.key {
            b"id" => way.id = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"version"  => way.version = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"user" => way.user = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"uid" => way.uid = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            _ => ()
        }
    }

    Ok(way)
}

pub fn parse_rel(xml_node: &XMLEvents::BytesStart) -> Result<Rel, XMLError> {
    let mut rel = Rel::new();

    for attr in xml_node.attributes() {
        let attr = attr?;

        match attr.key {
            b"id" => rel.id = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"version"  => rel.version = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"user" => rel.user = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"uid" => rel.uid = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            _ => ()
        }
    }

    Ok(rel)
}

pub fn parse_nd(xml_node: &XMLEvents::BytesStart) -> Result<i64, XMLError> {
    let mut ndref: Option<i64> = None;

    for attr in xml_node.attributes() {
        let attr = attr?;

        if let b"ref" = attr.key {
            ndref = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?)
        }
    }

    match ndref {
        Some(val) => Ok(val),
        None => Err(XMLError::InternalError(String::from("unable to parse ndref")))
    }
}

pub fn parse_tag(xml_node: &XMLEvents::BytesStart) -> Result<(String, String), XMLError> {
    let mut k: Option<String> = None;
    let mut v: Option<String> = None;

    for attr in xml_node.attributes() {
        let attr = attr?;

        match attr.key {
            b"k" => k = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"v" => v = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            _ => ()
        }
    }

    Ok((match k {
        Some(key) => key,
        None => { return Err(XMLError::InternalError(String::from("unable to parse key"))) }
    }, match v {
        Some(val) => val,
        None => { return Err(XMLError::InternalError(String::from("unable to parse value"))) }
    }))
}

pub fn parse_member(xml_node: &XMLEvents::BytesStart) -> Result<(Option<Value>, Option<i64>, Option<String>), XMLError> {
    let mut rtype: Option<Value> = None;
    let mut rref: Option<i64> = None;
    let mut rrole: Option<String> = None;

    for attr in xml_node.attributes() {
        let attr = attr?;

        match attr.key {
            b"type" => rtype = Some(match &*String::from_utf8(attr.value.into_owned()).unwrap() {
                "node" => Value::Node,
                "way" => Value::Way,
                "relation" => Value::Rel,
                _ => { return Err(XMLError::InternalError(String::from("invalid type"))); }
            }),
            b"rref" => rref = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            b"rrole" => rrole = Some(String::from_utf8(attr.value.into_owned()).unwrap().parse()?),
            _ => ()
        }
    }

    Ok((rtype, rref, rrole))
}
