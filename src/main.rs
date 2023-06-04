mod models;
mod xsd;
use anyhow::{ Error };

use odbc_api::IntoParameter;
use odbc_api::{ ConnectionOptions, Environment, Cursor, buffers::TextRowSet, Connection };
use xml::attribute::OwnedAttribute;
use std::collections::HashMap;
const BATCH_SIZE: usize = 5000;
use std::fs::File;
use std::io::{ BufReader, Read };

use xml::reader::{ EventReader, XmlEvent };



fn main() {



    
   
    // let env = Environment::new().expect("Cannot Connect To Database.");

    // let connection_string =
    //     "
    //     Driver={ODBC Driver 17 for SQL Server};\
    //     Server=VENUS\\LEVARE_SQL_1;\
    //     Trusted_Connection=yes;\
    // ";

    // let conn = env
    //     .connect_with_connection_string(connection_string, ConnectionOptions::default())
    //     .expect("Cannot Connect To Database.");

    // let query = String::from(
    //     "exec VCdb.dbo.SearchBasicHighwayVehicleInfo @Year = ?, @Make = ?, @Model = ?"
    // );
    // let year = String::from("2007");
    // let make = String::from("Honda");
    // let model = String::from("Civic");

    // let params = Some(vec![year, make, model]);
    // let mut vehicles = Statement::new(&conn, query, params);

    // vehicles = vehicles.exec().unwrap();

    // vehicles.print_results();

    // let f_path = "D:\\Projects\\WORK\\PIES\\Gates.xml";

    // parse_xml(f_path).expect("No Work!");
}

pub fn parse_xml(f_path: &str) -> Result<(), Error> {
    let file = File::open(f_path)?;
    let file = BufReader::new(file);
    let entries: Vec<_> = PiesXmlIterator::new(file)
        .map(|x| x.unwrap())
        .collect();

    println!("{}", entries.len());
    // for item in entries {

    //         // if item.item_num == None && item.segment == "Header" {

    //         //     println!("Element: {}", item.tag);
    //         // }

    //     // if item.segment == "Items" && item.parent == "Item" && item.tag == "PartNumber"{
    //     //     println!("Part Number:{}", item.value.unwrap());
    //     //     continue;
    //     // }

    // }
    // let item_no = entries
    //     .filter(|x| x.item_num != None)
    //     .last()
    //     .expect("Should have an item.")
    //     .item_num
    //     .expect("There should be a number here!");
    // println!("File Contains: {} Items", item_no);

    Ok(())
}

// enum XSDState {
//     Off,
//     Root,
//     Element,
//     Attribute,
//     Restriction,
//     ElementType,
//     Sequence,
    
// }








#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PiesEntry {
    tag: String,
    item_num: Option<u32>,
    parent: String,
    segment: String,
    attributes: Option<Vec<OwnedAttribute>>,
    value: Option<String>,
}
pub enum Segment {
    Off,
    Header,
    Item,
    Mkt,
    Price,
    Trailer,
}

struct PiesXmlIterator<R: Read> {
    parser: EventReader<R>,
    depth: u32,
    nodes: HashMap<u32, String>,
    item_num: u32,
    tag: Option<String>,
    segment: Option<String>,
    attributes: Option<Vec<OwnedAttribute>>,
    content: Option<String>,
    p_state: Segment,
}

impl<R: Read> PiesXmlIterator<R> {
    pub fn new(xml: R) -> PiesXmlIterator<R> {
        PiesXmlIterator {
            parser: EventReader::new(xml),
            depth: 0,
            nodes: HashMap::new(),
            item_num: 0,
            tag: None,
            segment: None,
            attributes: None,
            content: None,
            p_state: Segment::Off,
        }
    }
}

impl<R: Read> Iterator for PiesXmlIterator<R> {
    type Item = xml::reader::Result<PiesEntry>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    self.depth += 1;
                    self.nodes.insert(self.depth, name.local_name.clone());
                    match self.p_state {
                        Segment::Off => {
                            if name.local_name.as_str() == "Header" {
                                self.p_state = Segment::Header;
                                continue;
                            }
                        }
                        Segment::Header => {
                            let segment = String::from("Header");
                            if name.local_name.as_str() == "PriceSheets" {
                                self.p_state = Segment::Price;
                                continue;
                            }
                            self.tag = Some(name.local_name);
                            self.segment = Some(segment);
                        }
                        Segment::Price => {
                            let segment = String::from("Price");
                            if name.local_name.as_str() == "MarketingCopy" {
                                self.p_state = Segment::Mkt;
                                continue;
                            }
                            self.tag = Some(name.local_name);
                            self.attributes = Some(attributes);
                            self.segment = Some(segment);
                        }
                        Segment::Mkt => {
                            let segment = String::from("MarketingCopy");
                            if name.local_name.as_str() == "Items" {
                                self.p_state = Segment::Item;
                                continue;
                            }
                            self.tag = Some(name.local_name);
                            self.attributes = Some(attributes);
                            self.segment = Some(segment);
                        }
                        Segment::Item => {
                            let segment = String::from("Items");
                            if name.local_name.to_lowercase().as_str() == "item" {
                                self.item_num += 1;
                            }
                            if name.local_name.as_str() == "Trailer" {
                                self.p_state = Segment::Trailer;
                                self.segment = Some(String::from("Trailer"));
                                continue;
                            }
                            self.tag = Some(name.local_name);
                            self.attributes = Some(attributes);
                            self.segment = Some(segment);
                        }
                        Segment::Trailer => {
                            self.segment = Some(String::from("Trailer"));
                            self.tag = Some(name.local_name);
                            self.attributes = Some(attributes);
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    self.depth -= 1;

                    match self.p_state {
                        Segment::Off => {}
                        Segment::Header => {
                            if self.depth < 2 {
                                continue;
                            }
                            let parent_depth = &self.depth;
                            let parent = self.nodes
                                .iter()
                                .find_map(|(key, val)| (
                                    if key == parent_depth {
                                        Some(val)
                                    } else {
                                        None
                                    }
                                ));
                            let out = PiesEntry {
                                tag: self.tag.take().unwrap_or(name.local_name),
                                item_num: None,
                                parent: String::from(parent.unwrap().clone()),
                                segment: self.segment.clone().unwrap(),
                                attributes: None,
                                value: self.content.take(),
                            };
                            return Some(Ok(out));
                        }
                        Segment::Item => {
                            if self.depth < 2 {
                                continue;
                            }
                            let parent_depth = &self.depth;
                            let parent = self.nodes
                                .iter()
                                .find_map(|(key, val)| (
                                    if key == parent_depth {
                                        Some(val)
                                    } else {
                                        None
                                    }
                                ));
                            let out = PiesEntry {
                                tag: self.tag.take().unwrap_or(name.local_name),
                                item_num: Some(self.item_num),
                                parent: String::from(parent.unwrap().clone()),
                                segment: self.segment.clone().unwrap(),
                                attributes: self.attributes.take(),
                                value: self.content.take(),
                            };
                            return Some(Ok(out));
                        }
                        _ => {
                            if self.depth < 2 {
                                continue;
                            }
                            let parent_depth = &self.depth;
                            let parent = self.nodes
                                .iter()
                                .find_map(|(key, val)| (
                                    if key == parent_depth {
                                        Some(val)
                                    } else {
                                        None
                                    }
                                ));
                            let out = PiesEntry {
                                tag: self.tag.take().unwrap_or(name.local_name),
                                item_num: None,
                                parent: String::from(parent.unwrap().clone()),
                                segment: self.segment.clone().unwrap(),
                                attributes: self.attributes.take(),
                                value: self.content.take(),
                            };
                            return Some(Ok(out));
                        }
                    }
                }
                Ok(XmlEvent::Characters(s)) => {
                    match self.p_state {
                        Segment::Off => {}
                        _ => {
                            self.content = Some(s);
                        }
                    }
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
                _ => {}
            }
        }
        None
    }
}


struct Statement<'a> {
    conn: &'a Connection<'a>,
    query: String,
    params: Option<Vec<String>>,
    results: Option<Vec<Vec<Vec<u8>>>>,
}

impl Statement<'_> {
    pub fn new<'a>(
        conn: &'a Connection,
        query: String,
        params: Option<Vec<String>>
    ) -> Statement<'a> {
        Statement {
            conn,
            query,
            params,
            results: None,
        }
    }

    fn exec(mut self) -> Result<Self, Error> {
        match self.params.take() {
            Some(p) => {
                let mut rows: Vec<Vec<_>> = vec![];
                let input = p.to_owned();
                let params: Vec<_> = input
                    .iter()
                    .map(|x| x.as_str().into_parameter())
                    .collect();
                match self.conn.execute(&self.query, &params[..])? {
                    Some(mut cursor) => {
                        let mut buffers = TextRowSet::for_cursor(
                            BATCH_SIZE,
                            &mut cursor,
                            Some(4096)
                        )?;
                        let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                        while let Some(batch) = row_set_cursor.fetch()? {
                            for row_index in 0..batch.num_rows() {
                                let record = (0..batch.num_cols()).map(|col_index| {
                                    batch.at(col_index, row_index).unwrap_or(&[])
                                });

                                let row: Vec<_> = record.map(|x| x.to_owned()).collect();
                                rows.push(row);
                            }
                        }
                    }
                    None => {
                        self.results = None;
                    }
                }
                self.results = Some(rows);
                Ok(self)
            }
            None => {
                let mut rows: Vec<Vec<_>> = vec![];
                match self.conn.execute(&self.query, ())? {
                    Some(mut cursor) => {
                        let mut buffers = TextRowSet::for_cursor(
                            BATCH_SIZE,
                            &mut cursor,
                            Some(4096)
                        )?;
                        let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                        while let Some(batch) = row_set_cursor.fetch()? {
                            for row_index in 0..batch.num_rows() {
                                let record = (0..batch.num_cols()).map(|col_index| {
                                    batch.at(col_index, row_index).unwrap_or(&[])
                                });

                                let row: Vec<_> = record.map(|x| x.to_owned()).collect();

                                rows.push(row);
                            }
                        }
                    }
                    None => {
                        self.results = None;
                    }
                }
                self.results = Some(rows);
                Ok(self)
            }
        }
    }
    fn print_results(&mut self) {
        match self.results.take() {
            Some(rows) => {
                for row in rows {
                    let mut sentence = String::new();
                    for col in row {
                        if sentence.as_str() == "" {
                            sentence = sentence + String::from_utf8(col).unwrap().as_str();
                        } else {
                            sentence = sentence + "," + String::from_utf8(col).unwrap().as_str();
                        }
                    }
                    println!("{}", sentence);
                }
            }
            None => println!("Statement has no results!"),
        }
    }
}
