use xml::reader::{ EventReader, XmlEvent };
use xml::attribute::OwnedAttribute;
use std::collections::HashMap;
use std::io::Read;
#[derive(Debug)]
pub struct XsdEntry {
    pub tag: String,
    pub parent: Option<String>,
    pub node: u32,
    pub attributes: Option<Vec<OwnedAttribute>>,
    pub value: Option<String>,
}
pub enum XsdEvent {
    Off,
    On,
}
pub struct XmlSchema<R: Read> {
    parser: EventReader<R>,
    node: u32,
    nodes: HashMap<u32, String>,
    tag: Option<String>,
    parent: Option<String>,
    attributes: Option<Vec<OwnedAttribute>>,
    value: Option<String>,
    state: XsdEvent,
}

impl<R: Read> XmlSchema<R> {
    pub fn new(xml: R) -> XmlSchema<R> {
        XmlSchema {
            parser: EventReader::new(xml),
            node: 0,
            nodes: HashMap::new(),
            tag: None,
            attributes: None,
            parent: None,
            value: None,
            state: XsdEvent::Off,
        }
    }
}

impl<R: Read> Iterator for XmlSchema<R> {
    type Item = xml::reader::Result<XsdEntry>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    
                    match self.state {
                        XsdEvent::Off => { 
                            self.node += 1;
                            self.nodes.insert(self.node, name.local_name.clone());
                            self.tag = Some(name.local_name.to_owned());
                            self.attributes = Some(attributes);
                            self.state = XsdEvent::On;
                            continue;
                        }
                        XsdEvent::On => {
                            self.node += 1;
                            self.parent = self.nodes
                            .iter()
                            .find_map(|(key, val)| (
                                if key == &(self.node-2) {
                                    Some(val.to_owned())
                                } else {
                                    None
                                }
                            ));

                            let out = XsdEntry {
                                tag: self.tag.take().unwrap(),
                                node: self.node,
                                parent: self.parent.take(),
                                attributes: self.attributes.take(),
                                value: self.value.take(),
                            };
                            self.nodes.insert(self.node, name.local_name.clone());
                            self.tag = Some(name.local_name.to_owned());
                            self.attributes = Some(attributes);
                            return Some(Ok(out));
                        }
                    }
                }
                Ok(XmlEvent::Characters(val)) => {
                    self.parent = self.nodes
                    .iter()
                    .find_map(|(key, val)| (
                        if key == &(self.node-1) {
                            Some(val.to_owned())
                        } else {
                            None
                        }
                    ));
                    self.value = Some(val);
                    let out = XsdEntry {
                        tag: self.tag.take().unwrap(),
                        node: self.node,
                        parent: self.parent.take(),
                        attributes: self.attributes.take(),
                        value: self.value.take(),
                    };
                    self.state = XsdEvent::Off;
                    return Some(Ok(out));
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    self.node -= 1;


                    self.state = XsdEvent::Off;
                    continue;
                }
                Ok(XmlEvent::EndDocument { .. }) => {
                    break;
                }
                Err(ref e) if e.kind() == &xml::reader::ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(e) => {
                    return Some(Err(e));
                }
                _ => {
                    continue;
                }
            }
        }
        None
    }
}
