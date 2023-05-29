use anyhow::Error;
// use odbc_api::{ ConnectionOptions, Environment, Cursor, buffers::TextRowSet, Connection };
use xml::attribute::OwnedAttribute;
// const BATCH_SIZE: usize = 5000;
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

    // let query = "exec VCdb.dbo.SearchMakeByYear @Year = ?;";

    // let results = fetch_rows(conn, query).expect("Unable to retrieve data.");

    // println!("{}", results)

    let f_path = "D:\\Projects\\WORK\\PIES\\Gates.xml";

    parse_xml(f_path).expect("No Work!");
}

// pub fn fetch_rows(conn: Connection, query: &str) -> Result<String, Error> {
//     let year = 2007;
//     match conn.execute(query, &year)? {
//         Some(mut cursor) => {
//             let mut writer = csv::Writer::from_writer(vec![]);
//             let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
//             // Bind the buffer to the cursor. It is now being filled with every call to fetch.
//             let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

//             while let Some(batch) = row_set_cursor.fetch()? {
//                 for row_index in 0..batch.num_rows() {
//                     let record = (0..batch.num_cols()).map(|col_index| {
//                         batch.at(col_index, row_index).unwrap_or(&[])
//                     });
//                     writer.write_record(record)?;
//                 }
//             }
//             Ok(String::from_utf8(writer.into_inner()?)?)
//         }
//         None => { Ok(String::from("Query came back empty. No output has been created.")) }
//     }
// }

pub fn parse_xml(f_path: &str) -> Result<(), Error> {
    let file = File::open(f_path)?;
    let file = BufReader::new(file);
    let entries = PiesXmlIterator::new(file).map(|x| x.unwrap());

    let item_no = entries
        .filter(|x| x.item_num != None)
        .last()
        .expect("Should have an item.")
        .item_num
        .expect("There should be a number here!");
    println!("File Contains: {} Items", item_no);

    Ok(())
}

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
    parent: Option<String>,
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
            item_num: 0,
            parent: None,
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

                    match self.p_state {
                        Segment::Off => {
                            if name.local_name.as_str() == "Header" {
                                self.p_state = Segment::Header;
                                self.parent = Some(name.local_name.clone());
                                continue;
                            }
                        }
                        Segment::Header => {
                            let segment = "Header";
                            if name.local_name.as_str() == "PriceSheets" {
                                self.p_state = Segment::Price;
                                self.parent = Some(name.local_name.clone());
                                continue;
                            }
                            self.tag = Some(name.local_name);
                            self.segment = Some(String::from(segment));
                        }
                        Segment::Price => {
                            let segment = "Price";
                            if name.local_name.as_str() == "MarketingCopy" {
                                self.p_state = Segment::Mkt;
                                self.parent = Some(name.local_name.clone());
                                continue;
                            }
                            self.tag = Some(name.local_name);
                            self.attributes = Some(attributes);
                            self.segment = Some(String::from(segment));
                        }
                        Segment::Mkt => {
                            let segment = "MarketingCopy";
                            if name.local_name.as_str() == "Items" {
                                self.p_state = Segment::Item;
                                self.parent = Some(name.local_name.clone());
                                continue;
                            }
                            self.tag = Some(name.local_name);
                            self.attributes = Some(attributes);
                            self.segment = Some(String::from(segment));
                        }
                        Segment::Item => {
                            let segment = "Items";
                            if name.local_name.to_lowercase().as_str() == "item" {
                                self.item_num += 1;
                            }
                            if name.local_name.as_str() == "Trailer" {
                                self.p_state = Segment::Trailer;
                                self.segment = Some(String::from("Trailer"));
                                self.parent = Some(name.local_name.clone());
                                continue;
                            }
                            self.tag = Some(name.local_name);
                            self.attributes = Some(attributes);
                            self.segment = Some(String::from(segment));
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
                    self.parent = Some(name.local_name.clone());
                    match self.p_state {
                        Segment::Off => {}
                        Segment::Header => {
                            if self.depth < 2 {
                                continue;
                            }
                            let out = PiesEntry {
                                tag: self.tag.take().unwrap_or(name.local_name.clone()),
                                item_num: None,
                                parent: self.parent.take().unwrap(),
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
                            let out = PiesEntry {
                                tag: self.tag.take().unwrap_or(name.local_name.clone()),
                                item_num: Some(self.item_num),
                                parent: self.parent.take().unwrap(),
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
                            let out = PiesEntry {
                                tag: self.tag.take().unwrap_or(name.local_name.clone()),
                                item_num: None,
                                parent: self.parent.take().unwrap(),
                                segment: self.segment.clone().unwrap_or(name.local_name),
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
