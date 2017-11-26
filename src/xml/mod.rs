extern crate geojson;
extern crate quick_xml;
extern crate serde_json;

mod node;
mod way;
mod rel;
mod tree;

use xml::node::Node;
use xml::way::Way;
use xml::rel::Rel;
use xml::tree::OSMTree;

use std::string;
use std::num;
use std::io::Cursor;
use self::quick_xml::writer::Writer;
use self::quick_xml::events as XMLEvents;
use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum XMLError {
    Unknown,
    Invalid,
    GCNotSupported,
    EncodingFailed,
    InternalError,
    ParsingError,
    InvalidNode,
    InvalidNodeRef,
    InvalidWay,
    InvalidWayRef,
    InvalidRel,
    InvalidXML,
    NotFoundError,
    StringParsing(string::ParseError),
    IntParsing(num::ParseIntError),
    FloatParsing(num::ParseFloatError)
}

impl XMLError {
    pub fn to_string(&self) -> &str {
        match *self {
            XMLError::Unknown => "Unknown Error",
            XMLError::GCNotSupported => "GeometryCollection are not currently supported",
            XMLError::Invalid => "Could not parse XML - Invalid",
            XMLError::EncodingFailed => "Encoding Failed",
            XMLError::InternalError => "Internal Error",
            XMLError::ParsingError => "Parsing Error",
            XMLError::InvalidNode => "Invalid Node",
            XMLError::InvalidNodeRef => "Invalid Node Reference",
            XMLError::InvalidWay => "Invalid Way",
            XMLError::InvalidWayRef => "Invalid Way Reference",
            XMLError::InvalidRel => "Invalid Relation",
            XMLError::InvalidXML => "Invalid XML",
            XMLError::NotFoundError => "Not Found",
            XMLError::StringParsing(_) => "Could not parse attribute to string",
            XMLError::IntParsing(_) => "Could not parse attribute to integer",
            XMLError::FloatParsing(_) => "Could not parse attribute to float"
        }
    }
}

impl From<string::FromUtf8Error> for XMLError {
    fn from(error: string::FromUtf8Error) -> XMLError {
        XMLError::ParsingError
    }   
}

impl From<quick_xml::errors::Error> for XMLError {
    fn from(error: quick_xml::errors::Error) -> XMLError {
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
    fn to_feat(&self) -> geojson::Feature;
    fn is_valid(&self) -> bool;
}

pub struct OSMTypes {
    node_it: i64,
    nodes: String,
    ways: String,
    rels: String
}

impl OSMTypes {
    pub fn new() -> OSMTypes {
        OSMTypes {
            node_it: 0,
            nodes: String::from(""),
            ways: String::from(""),
            rels: String::from("")
        }
    }
}

pub fn to_changeset_tag(xml_node: &quick_xml::events::BytesStart, map: &mut HashMap<String, Option<String>>) {
    let mut kv: (Option<String>, Option<String>) = (None, None);

    for attr in xml_node.attributes() {
        let attr = attr.unwrap();

        match attr.key {
            b"k"  => kv.0 = Some(String::from_utf8_lossy(attr.value).parse().unwrap()),
            b"v"  => kv.1 = Some(String::from_utf8_lossy(attr.value).parse().unwrap()),
            _ => ()
        }
    }

    map.insert(kv.0.unwrap(), kv.1);
}

pub fn to_changeset(body: &String) -> Result<HashMap<String, Option<String>>, XMLError> {
    let mut reader = quick_xml::reader::Reader::from_str(body);
    let mut buf = Vec::new();

    let mut map = HashMap::new();

     loop {
        match reader.read_event(&mut buf) {
            Ok(XMLEvents::Event::Start(ref e)) => {
                match e.name() {
                    b"tag" => { to_changeset_tag(&e, &mut map) },
                    _ => (),
                }
            },
            Ok(XMLEvents::Event::Empty(ref e)) => {
                match e.name() {
                    b"tag" => { to_changeset_tag(&e, &mut map) },
                    _ => (),
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

pub fn to_features(body: &String) -> Result<geojson::FeatureCollection, XMLError> {
    let mut tree = tree_parser(&body)?;

    let mut fc = geojson::FeatureCollection {
        bbox: None,
        features: vec![],
        foreign_members: None
    };

    for (i, rel) in tree.get_rels() {
        if !rel.has_tags() { continue; }

        fc.features.push(rel.to_feat());        
    }

    for (i, way) in tree.get_ways() {
        if !way.has_tags() { continue; }

        fc.features.push(way.to_feat());        
    }

    for (i, node) in tree.get_nodes() {
        if !node.has_tags() { continue; }

        fc.features.push(node.to_feat());        
    }

    Err(XMLError::InternalError)
}

pub fn tree_parser(body: &String) -> Result<OSMTree, XMLError> {
    let mut tree: OSMTree = OSMTree::new();

    let mut opening_osm = false;
    let mut n: Node = Node::new();
    let mut w: Way = Way::new();
    let mut r: Rel = Rel::new();

    let mut currentValue = Value::None;
    let mut currentAction = Action::None;

    let mut reader = quick_xml::reader::Reader::from_str(body);
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
                        if currentAction != Action::None { return Err(XMLError::InternalError); }
                        currentAction = Action::Create;
                    },
                    b"modify" => {
                        if currentAction != Action::None { return Err(XMLError::InternalError); }
                        currentAction = Action::Modify;
                    },
                    b"delete" => {
                        if currentAction != Action::None { return Err(XMLError::InternalError); }
                        currentAction = Action::Delete;
                    },
                    b"node" => {
                        if currentAction == Action::None { return Err(XMLError::InternalError) }
                        if currentValue != Value::None { return Err(XMLError::InternalError) }

                        n = parse_node(e)?;
                        n.action = Some(currentAction.clone());

                        if n.action == Some(Action::Create) {
                            n.version = Some(1);
                        }

                        currentValue = Value::Node;
                    },
                    b"way" => {
                        if currentAction == Action::None { return Err(XMLError::InternalError) }
                        if currentValue != Value::None { return Err(XMLError::InternalError) }

                        w = parse_way(e)?;
                        w.action = Some(currentAction.clone());

                        if w.action == Some(Action::Create) {
                            w.version = Some(1);
                        }

                        currentValue = Value::Way;
                    },
                    b"relation" => {
                        if currentAction == Action::None { return Err(XMLError::InternalError) }
                        if currentValue != Value::None { return Err(XMLError::InternalError) }

                        r = parse_rel(e)?;
                        r.action = Some(currentAction.clone());

                        if r.action == Some(Action::Create) {
                            r.version = Some(1);
                        }

                        currentValue = Value::Rel;
                    },
                    b"tag" => {
                        let (k, v) = parse_tag(&e)?;

                        match currentValue {
                            Value::None => { return Err(XMLError::InternalError) },
                            Value::Node => {
                                n.set_tag(k, v);
                            },
                            Value::Way => {
                                w.set_tag(k, v);
                            },
                            Value::Rel => {
                                r.set_tag(k, v);
                            }
                        };

                    }
                    _ => (),
                }
            },
            Ok(XMLEvents::Event::Empty(ref e)) => {
                match e.name() {
                    b"node" => {
                        if currentAction == Action::None { return Err(XMLError::InternalError) }
                        if currentValue != Value::None { return Err(XMLError::InternalError) }

                        n = parse_node(&e)?;
                        n.action = Some(currentAction.clone());

                        if n.action == Some(Action::Create) {
                            n.version = Some(1);
                        }

                        tree.add_node(n)?;

                        n = Node::new();
                    },
                    b"way" => {
                        return Err(XMLError::InternalError);
                    },
                    b"relation" => {
                        return Err(XMLError::InternalError);
                    },
                    b"nd" => {
                        if currentValue != Value::Way { return Err(XMLError::InternalError) }

                        let ndref = parse_nd(&e)?;
                        w.nodes.push(ndref);
                    },
                    b"tag" => {
                        let (k, v) = parse_tag(&e)?;

                        match currentValue {
                            Value::None => { return Err(XMLError::InternalError) },
                            Value::Node => {
                                n.set_tag(k, v);
                            },
                            Value::Way => {
                                w.set_tag(k, v);
                            },
                            Value::Rel => {
                                r.set_tag(k, v);
                            }
                        };
                    },
                    b"member" => {
                        let (rtype, rref, rrole) = parse_member(&e)?;

                        match currentValue {
                            Value::Rel => {
                                r.set_member(rtype, rref, rrole);
                            },
                            _ => {
                                return Err(XMLError::InternalError);
                            }
                        };

                    }
                    _ => ()
                }
            },
            Ok(XMLEvents::Event::End(ref e)) => {
                match e.name() {
                    b"node" => {
                        if currentValue != Value::Node { return Err(XMLError::InternalError); }

                        tree.add_node(n)?;
                        n = Node::new();
                        currentValue = Value::None;
                    },
                    b"way" => {
                        if currentValue != Value::Way { return Err(XMLError::InternalError); }
                        tree.add_way(w)?;
                        w = Way::new();
                        currentValue = Value::None;
                    },
                    b"relation" => {
                        if currentValue != Value::Rel { return Err(XMLError::InternalError); }
                        tree.add_rel(r)?;
                        r = Rel::new();
                        currentValue = Value::None;
                    },
                    b"create" => {
                        if currentAction != Action::Create { return Err(XMLError::InternalError); }
                        currentAction = Action::None;
                    },
                    b"modify" => {
                        if currentAction != Action::Modify { return Err(XMLError::InternalError); }
                        currentAction = Action::None;
                    },
                    b"delete" => {
                        if currentAction != Action::Delete { return Err(XMLError::InternalError); }
                        currentAction = Action::None;
                    },
                    b"osmChange" => {
                        if currentValue != Value::None { return Err(XMLError::InternalError); }
                        if !opening_osm { return Err(XMLError::InternalError); }

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

    Err(XMLError::InvalidXML)
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
                        multipoint(&feat, &coords, &mut osm);
                    },
                    geojson::Value::LineString(ref coords) => {
                        linestring(&feat, &coords, &mut osm)?;
                    },
                    geojson::Value::MultiLineString(ref coords) => {
                        multilinestring(&feat, &coords, &mut osm);
                    },
                    geojson::Value::Polygon(ref coords) => {
                        polygon(&feat, &coords, &mut osm);
                    },
                    geojson::Value::MultiPolygon(ref coords) => {
                        multipolygon(&feat, &coords, &mut osm);
                    },
                    _ => {
                        return Err(XMLError::GCNotSupported);
                    }
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
    xml_node.push_attribute(("id", "1"));
    xml_node.push_attribute(("version", "1"));
    xml_node.push_attribute(("lat", &*coords[0].to_string()));
    xml_node.push_attribute(("lon", &*coords[1].to_string()));

    writer.write_event(XMLEvents::Event::Start(xml_node)).unwrap();

    match *&feat.properties {
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

pub fn multipoint(_feat: &geojson::Feature, _coords: &Vec<geojson::PointType>, _osm: &mut OSMTypes) {

}

pub fn linestring(feat: &geojson::Feature, coords: &geojson::LineStringType, osm: &mut OSMTypes) -> Result<bool, XMLError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut xml_way = XMLEvents::BytesStart::owned(b"way".to_vec(), 3);
    xml_way.push_attribute(("id", "1"));
    xml_way.push_attribute(("version", "1"));

    writer.write_event(XMLEvents::Event::Start(xml_way)).unwrap();

    for nd in coords {
        let node = match add_node(&nd, osm) {
            Ok(node) => node,
            Err(_) => { return Err(XMLError::EncodingFailed); }
        };

        osm.nodes.push_str(&*node.0);

        let mut xml_nd = XMLEvents::BytesStart::owned(b"nd".to_vec(), 2);
        xml_nd.push_attribute(("ref", &*node.1.to_string()));
        writer.write_event(XMLEvents::Event::Empty(xml_nd)).unwrap();
    }

    match *&feat.properties {
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
pub fn multilinestring(_feat: &geojson::Feature, _coords: &Vec<geojson::LineStringType>, _osm: &mut OSMTypes) {

}

pub fn polygon(_feat: &geojson::Feature, _coords: &geojson::PolygonType, _osm: &mut OSMTypes) {

}

pub fn multipolygon(_feat: &geojson::Feature, _coords: &Vec<geojson::PolygonType>, _osm: &mut OSMTypes) {

}

pub fn json2str(v: &serde_json::value::Value) -> String {
    match *v {
        serde_json::value::Value::Null => String::from(""),
        serde_json::value::Value::Bool(ref json_bool) => match *json_bool {
            true => String::from("yes"),
            false => String::from("no")
        },
        serde_json::value::Value::Number(ref json_num) => String::from(format!("{}", json_num)),
        serde_json::value::Value::String(ref json_str) => json_str.to_string(),
        _ => v.to_string()
    }
}

pub fn add_node(coords: &geojson::PointType, osm: &mut OSMTypes) -> Result<(String, i64), XMLError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut xml_node = XMLEvents::BytesStart::owned(b"node".to_vec(), 4);

    osm.node_it = osm.node_it - 1;

    xml_node.push_attribute(("id", &*osm.node_it.to_string()));
    xml_node.push_attribute(("version", "1"));
    xml_node.push_attribute(("lat", &*coords[0].to_string()));
    xml_node.push_attribute(("lon", &*coords[1].to_string()));

    writer.write_event(XMLEvents::Event::Empty(xml_node)).unwrap();

    Ok((String::from_utf8(writer.into_inner().into_inner()).unwrap(), osm.node_it))
}

pub fn parse_osm(xml_node: &XMLEvents::BytesStart, meta: &mut HashMap<String, String>) -> Result<bool, XMLError> {
    for attr in xml_node.attributes() {
        let attr = attr?;

        let key = String::from_utf8_lossy(attr.key);
        let val = String::from_utf8_lossy(attr.value);

        meta.insert(key.to_string(), val.to_string());
    }

    if !meta.contains_key("version") {
        return Err(XMLError::InternalError);
    }

    let v: f32 = match meta.get("version") {
        Some(ver) => ver.parse()?,
        None => { return Err(XMLError::InternalError); }
    };

    if v != 0.6 {
        return Err(XMLError::InternalError);
    }

    return Ok(true);
}

pub fn parse_node(xml_node: &XMLEvents::BytesStart) -> Result<Node, XMLError> {
    let mut node = Node::new();

    for attr in xml_node.attributes() {
        let attr = attr?;

        match attr.key {
            b"id" => node.id = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"lat" => node.lat = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"lon" => node.lon = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"user" => node.user = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"uid" => node.uid = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"version" => node.version = Some(String::from_utf8_lossy(attr.value).parse()?),
            _ => ()
        }
    }

    return Ok(node);
}

pub fn parse_way(xml_node: &XMLEvents::BytesStart) -> Result<Way, XMLError> {
    let mut way = Way::new();

    for attr in xml_node.attributes() {
        let attr = attr?;

        match attr.key {
            b"id" => way.id = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"version"  => way.version = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"user" => way.user = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"uid" => way.uid = Some(String::from_utf8_lossy(attr.value).parse()?),
            _ => ()
        }
    }

    return Ok(way);
}

pub fn parse_rel(xml_node: &XMLEvents::BytesStart) -> Result<Rel, XMLError> {
    let mut rel = Rel::new();

    for attr in xml_node.attributes() {
        let attr = attr?;

        match attr.key {
            b"id" => rel.id = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"version"  => rel.version = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"user" => rel.user = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"uid" => rel.uid = Some(String::from_utf8_lossy(attr.value).parse()?),
            _ => ()
        }
    }

    return Ok(rel);
}

pub fn parse_nd(xml_node: &XMLEvents::BytesStart) -> Result<i64, XMLError> {
    let mut ndref: Option<i64> = None;

    for attr in xml_node.attributes() {
        let attr = attr?;

        match attr.key {
            b"ref" => ndref = Some(String::from_utf8_lossy(attr.value).parse()?),
            _ => ()
        }
    }

    match ndref {
        Some(val) => Ok(val),
        None => Err(XMLError::InternalError)
    }
}

pub fn parse_tag(xml_node: &XMLEvents::BytesStart) -> Result<(String, String), XMLError> {
    let mut k: Option<String> = None;
    let mut v: Option<String> = None;

    for attr in xml_node.attributes() {
        let attr = attr?;

        match attr.key {
            b"k" => k = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"v" => v = Some(String::from_utf8_lossy(attr.value).parse()?),
            _ => ()
        }
    }

    return Ok((match k {
        Some(key) => key,
        None => { return Err(XMLError::InternalError) }
    }, match v {
        Some(val) => val,
        None => { return Err(XMLError::InternalError) }
    }));
}

pub fn parse_member(xml_node: &XMLEvents::BytesStart) -> Result<(Option<Value>, Option<i64>, Option<String>), XMLError> {
    let mut rtype: Option<Value> = None;
    let mut rref: Option<i64> = None;
    let mut rrole: Option<String> = None;

    for attr in xml_node.attributes() {
        let attr = attr?;

        match attr.key {
            b"type" => rtype = Some(match attr.value {
                b"node" => Value::Node,
                b"way" => Value::Way,
                b"relation" => Value::Rel,
                _ => { return Err(XMLError::InternalError); }
            }),
            b"rref" => rref = Some(String::from_utf8_lossy(attr.value).parse()?),
            b"rrole" => rrole = Some(String::from_utf8_lossy(attr.value).parse()?),
            _ => ()
        }
    }

    return Ok((rtype, rref, rrole))
}
