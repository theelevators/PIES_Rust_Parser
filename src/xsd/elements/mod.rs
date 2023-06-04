
pub enum XSDState {
    Off,
    All,
    Annotation,
    Any,
    AnyAttribute,
    AppInfo,
    Attribute,
    AttributeGroup,
    Choice,
    ComplexContent,
    ComplexType,
    Documentation,
    Element,
    Extension,
    Field,
    Group,
    Import,
    Include,
    Key,
    KeyRef,
    List,
    Notation,
    Redefine,
    Restriction,
    Schema,
    Selector,
    Sequence,
    SimpleContent,
    SimpleType,
    Union,
    Unique
}

pub enum NameSpace {
    Any,
    Other,
    Local,
    Target,
    List
}
pub enum DataType{
    Boolean,
    Numeric,
    Date,
    String
}


pub type AnyAttributes = Option<Vec<(String, String)>>;


pub type Source = String;
pub type SubstitutionGroup = String;
pub type ID =String;
pub type Language = String;
pub type Xpath = String;