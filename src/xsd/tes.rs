use crate::xsd::elements::XSDState;

use xml::reader::{ EventReader, XmlEvent };
use xml::attribute::OwnedAttribute;
use std::collections::HashMap;
use std::io::Read;

struct XsdEntry {
    tag: String,
    parent: String,
    node: u32,
    attributes: Option<Vec<OwnedAttribute>>,
    value: Option<String>,
}
pub enum XsdEvent {
    Off,
    On,
}
struct XmlSchema<R: Read> {
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
                    self.node += 1;
                    self.parent = self.nodes
                        .iter()
                        .find_map(|(key, val)| (
                            if key == &(self.node - 1) {
                                Some(val.to_owned())
                            } else {
                                None
                            }
                        ));

                    match self.state {
                        XsdEvent::Off => {
                            self.nodes.insert(self.node, name.local_name.clone());
                            self.tag = Some(name.local_name.to_owned());
                            self.attributes = Some(attributes);
                            self.state = XsdEvent::On;
                        }
                        XsdEvent::On => {
                            let out = XsdEntry {
                                tag: self.tag.take().unwrap(),
                                node: self.node,
                                parent: self.parent.take().unwrap(),
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
                    self.value = Some(val);
                    let out = XsdEntry {
                        tag: self.tag.take().unwrap(),
                        node: self.node,
                        parent: self.parent.take().unwrap(),
                        attributes: self.attributes.take(),
                        value: self.value.take(),
                    };
                    self.state = XsdEvent::Off;
                    return Some(Ok(out));
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    self.node -= 1;
                    self.state = XsdEvent::Off;
                }

                Err(ref e) if e.kind() == &xml::reader::ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(e) => {
                    return Some(Err(e));
                }
                _ => {}
            }
        }
        None
    }
}

// impl<R: Read> Iterator for XmlSchema<R> {
//     type Item = xml::reader::Result<XsdEntry
// >;
//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//              match self.parser.next(){
//                 Ok(XmlEvent::StartElement { name, attributes, .. }) =>{
//                     self.depth += 1;
//                     self.nodes.insert(self.depth, name.local_name.clone());

//                     match name.local_name.as_str() {
//                         "xml" =>{
//                             self.attributes = Some(attributes);
//                         }
//                         "xs:schema" =>{
//                             self.state = XSDState::Schema;
//                             self.attributes = Some(attributes);
//                             continue;
//                         }
//                         "xs:element"=>{
//                             self.state = XSDState::Element;
//                             self.attributes = Some(attributes);
//                         }
//                         "xs:any" =>{
//                             self.state = XSDState::Any;
//                         }
//                         "xs:anyAttribute" =>{
//                             self.state = XSDState::AnyAttribute;
//                         }
//                         "xs:appinfo" =>{
//                             self.state = XSDState::AppInfo;
//                         }
//                         "xs:attribute" =>{
//                             self.state = XSDState::Attribute;
//                         }
//                         "xs:attributeGroup" =>{
//                             self.state = XSDState::AttributeGroup;
//                         }
//                         "xs:choice" =>{
//                             self.state = XSDState::Choice;
//                         }
//                         "xs:complexContent" =>{
//                             self.state = XSDState::ComplexContent;
//                         }
//                         "xs:complexType" =>{
//                             self.state = XSDState::ComplexType;
//                         }
//                         "xs:documentation" =>{
//                             self.state = XSDState::Documentation;
//                         }
//                         "xs:field" =>{
//                             self.state = XSDState::Field;
//                         }
//                         "xs:group" =>{
//                             self.state = XSDState::Group;
//                         }
//                         "xs:import" =>{
//                             self.state = XSDState::Import;
//                         }
//                         "xs:include" =>{
//                             self.state = XSDState::Include;
//                         }
//                         "xs:key" =>{
//                             self.state = XSDState::Key;
//                         }
//                         "xs:keyref" =>{
//                             self.state = XSDState::KeyRef;
//                         }
//                         "xs:list" =>{
//                             self.state = XSDState::List;
//                         }
//                         "xs:notation" =>{
//                             self.state = XSDState::Notation;
//                         }
//                         "xs:redefine" =>{
//                             self.state = XSDState::Redefine;
//                         }
//                         "xs:restriction" =>{
//                             self.state = XSDState::Restriction;
//                         }
//                         "xs:selector" =>{
//                             self.state = XSDState::Selector;
//                         }
//                         "xs:sequence" =>{
//                             self.state = XSDState::Sequence;
//                         }
//                         "xs:simpleContent" =>{
//                             self.state = XSDState::SimpleContent;
//                         }
//                         "xs:simpleType" =>{
//                             self.state = XSDState::SimpleType;
//                         }
//                         "xs:union" =>{
//                             self.state = XSDState::Union;
//                         }
//                         "xs:unique" =>{
//                             self.state = XSDState::Unique;
//                         }
//                         _ => {}
//                     }

//                 }
//                 Err(ref e) if e.kind() == &xml::reader::ErrorKind::UnexpectedEof => {
//                     break;
//                 }
//                 Err(e) => {
//                     return Some(Err(e));
//                 }
//                 _ => {}
//         }
//     }
//     None
// }
// }

pub enum Indicator {
    All,
    Choice,
    Sequence,
    MaxOccurs,
    MinOccurs,
    GroupName,
    AttributeGroupName,
}

pub enum ProcessContents {
    Lax,
    Skip,
    Strict,
}

pub enum Form {
    Qualified,
    Unqualified,
}

pub enum Type {
    SimpleType,
    ComplexType,
}

pub enum Use {
    Optional,
    Prohibited,
    Required,
}

pub enum Specifier {
    All,
    Extension,
    Restriction,
    List,
    Union,
    Substitution,
}
