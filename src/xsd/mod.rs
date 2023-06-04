pub mod elements;
pub mod data_types;
use self::data_types::XSDString;
use self::data_types::any_uri::AnyUri;
use self::elements::{XSDState, ID, SubstitutionGroup,NameSpace, Source, Xpath, Language, DataType};

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;
use std::collections::HashMap;
use std::io::Read;


struct XSDEntry{
    node: u32,
    parent: String,
    xname: XSDState,
    id: Option<ID>,
    name: Option<String>,
    namespace: Option<NameSpace>,
    source: Option<Source>,
    r#ref: Option<String>,
    r#type: Option<Type>,
    r#use: Option<Use>,
    substitution_group: Option<SubstitutionGroup>,
    default: Option<String>,
    fixed: Option<String>,
    form: Option<Form>,
    max_occurs: Option<u32>,
    min_occurs: Option<u32>,
    nillable: Option<bool>,
    r#abstract: Option<bool>,
    block: Option<Specifier>,
    r#final: Option<Specifier>,
    attributes: Option<Vec<(String, String)>>,
    process_contents: Option<ProcessContents>,
    mixed: Option<bool>,
    xml_lang: Option<Language>,
    base: Option<String>,
    xpath: Option<Xpath>,
    schema_location:Option<AnyUri>,
    item_type: Option<DataType>,
    public: Option<AnyUri>,
    system: Option<AnyUri>,
    attibute_form_default: Option<Form>,
    element_form_default: Option<Form>,
    block_default: Option<Specifier>,
    final_default: Option<Specifier>,
    target_namespace: Option<AnyUri>,
    version: Option<XSDString>,
    xmlns: Option<AnyUri>,
    union: Option<Vec<String>>
}

struct XsdIterator<R: Read> {
    parser: EventReader<R>,
    depth: u32,
    nodes: HashMap<u32, String>,
    id: Option<ID>,
    name: Option<String>,
    namespace: Option<NameSpace>,
    source: Option<Source>,
    r#ref: Option<String>,
    r#type: Option<Type>,
    r#use: Option<Use>,
    substitution_group: Option<SubstitutionGroup>,
    default: Option<String>,
    fixed: Option<String>,
    form: Option<Form>,
    max_occurs: Option<u32>,
    min_occurs: Option<u32>,
    nillable: Option<bool>,
    r#abstract: Option<bool>,
    block: Option<Specifier>,
    r#final: Option<Specifier>,
    process_contents: Option<ProcessContents>,
    mixed: Option<bool>,
    xml_lang: Option<Language>,
    base: Option<String>,
    xpath: Option<Xpath>,
    schema_location:Option<AnyUri>,
    item_type: Option<DataType>,
    public: Option<AnyUri>,
    system: Option<AnyUri>,
    attibute_form_default: Option<Form>,
    element_form_default: Option<Form>,
    block_default: Option<Specifier>,
    final_default: Option<Specifier>,
    target_namespace: Option<AnyUri>,
    version: Option<XSDString>,
    xmlns: Option<AnyUri>,
    union: Option<Vec<String>>,
    attributes: Option<Vec<OwnedAttribute>>,
    content: Option<String>,
    state: XSDState,
}


impl<R: Read> XsdIterator<R> {
    pub fn new(xml: R) -> XsdIterator<R> {
        XsdIterator {
            parser: EventReader::new(xml),
            depth: 0,
            nodes: HashMap::new(),
            id: None,
            name: None,
            namespace: None,
            source: None,
            r#ref: None,
            r#type: None,
            r#use: None,
            substitution_group: None,
            default: None,
            fixed: None,
            form: None,
            max_occurs: None,
            min_occurs: None,
            nillable: None,
            r#abstract: None,
            block: None,
            r#final: None,
            process_contents: None,
            mixed: None,
            xml_lang:None,
            base: None,
            xpath: None,
            schema_location:None,
            item_type: None,
            public: None,
            system: None,
            attibute_form_default: None,
            element_form_default: None,
            block_default: None,
            final_default: None,
            target_namespace: None,
            version: None,
            xmlns: None,
            union: None,
            attributes: None,
            content: None,
            state: XSDState::Off,
        }
    }
}

impl<R: Read> Iterator for XsdIterator<R> {
    type Item = xml::reader::Result<XSDEntry>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
             match self.parser.next(){
                Ok(XmlEvent::StartElement { name, attributes, .. }) =>{
                    self.depth += 1;
                    self.nodes.insert(self.depth, name.local_name.clone());

                    match name.local_name.as_str() {
                        "xml" =>{
                            self.attributes = Some(attributes);
                        }
                        "xs:schema" =>{
                            self.state = XSDState::Schema;
                            self.attributes = Some(attributes);
                            continue;
                        }
                        "xs:element"=>{
                            self.state = XSDState::Element;
                            self.attributes = Some(attributes);
                        }
                        "xs:any" =>{
                            self.state = XSDState::Any;
                        }
                        "xs:anyAttribute" =>{
                            self.state = XSDState::AnyAttribute;
                        }
                        "xs:appinfo" =>{
                            self.state = XSDState::AppInfo;
                        }
                        "xs:attribute" =>{
                            self.state = XSDState::Attribute;
                        }
                        "xs:attributeGroup" =>{
                            self.state = XSDState::AttributeGroup;
                        }
                        "xs:choice" =>{
                            self.state = XSDState::Choice;
                        }
                        "xs:complexContent" =>{
                            self.state = XSDState::ComplexContent;
                        }
                        "xs:complexType" =>{
                            self.state = XSDState::ComplexType;
                        }                       
                        "xs:documentation" =>{
                            self.state = XSDState::Documentation;
                        }
                        "xs:field" =>{
                            self.state = XSDState::Field;
                        }
                        "xs:group" =>{
                            self.state = XSDState::Group;
                        }
                        "xs:import" =>{
                            self.state = XSDState::Import;
                        }
                        "xs:include" =>{
                            self.state = XSDState::Include;
                        }
                        "xs:key" =>{
                            self.state = XSDState::Key;
                        }
                        "xs:keyref" =>{
                            self.state = XSDState::KeyRef;
                        }                        
                        "xs:list" =>{
                            self.state = XSDState::List;
                        }
                        "xs:notation" =>{
                            self.state = XSDState::Notation;
                        }
                        "xs:redefine" =>{
                            self.state = XSDState::Redefine;
                        }
                        "xs:restriction" =>{
                            self.state = XSDState::Restriction;
                        }
                        "xs:selector" =>{
                            self.state = XSDState::Selector;
                        }
                        "xs:sequence" =>{
                            self.state = XSDState::Sequence;
                        }
                        "xs:simpleContent" =>{
                            self.state = XSDState::SimpleContent;
                        }                       
                        "xs:simpleType" =>{
                            self.state = XSDState::SimpleType;
                        }
                        "xs:union" =>{
                            self.state = XSDState::Union;
                        }
                        "xs:unique" =>{
                            self.state = XSDState::Unique;
                        }
                        _ => {}
                    }






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

pub enum Indicator{
    All,
    Choice,
    Sequence,
    MaxOccurs,
    MinOccurs,
    GroupName,
    AttributeGroupName
}

pub enum ProcessContents {
    Lax,
    Skip,
    Strict
}

pub enum Form{
    Qualified,
    Unqualified
}

pub enum  Type {
    SimpleType,
    ComplexType
}

pub enum Use {
    Optional,
    Prohibited,
    Required
}

pub enum Specifier{
    All,
    Extension,
    Restriction,
    List,
    Union,
    Substitution,

}

