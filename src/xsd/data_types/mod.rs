pub mod boolean;
pub mod any_uri;
pub mod date_time;
pub mod numeric;


pub type XSDString = String;

pub trait XString {
    fn is_normalized(&self) -> bool;
    fn to_norm(self) -> Self;
    fn is_token(&self)->bool;
    fn to_token(self)->Self;
}

impl XString for XSDString {
    fn is_normalized(&self)->bool{
        let invalid = vec!["\n","\r","\t"];
        for char in invalid{
            if self.contains(char){
                return false;
            } 
        }
        return true;
    }
    fn to_norm(mut self) -> Self {
        let invalid = vec!["\n","\r","\t"];

        while self.is_normalized() == false{
            for char in &invalid{
                self = self.replace(char, "");
            }
        }
      return self;
    }
    fn is_token(&self)->bool{
        let invalid = vec!["\n","\r","\t","  "];
        if self.ends_with(" "){
            return false;
        }
        if self.starts_with(" "){
            return false;
        }
        for char in invalid{
            if self.contains(char){
                return false;
            } 
        }
        return true;
    }
    fn to_token(mut self)->Self {

        let invalid = vec!["\n","\r","\t","  "];
        while self.is_token() == false{


            for char in &invalid{
                match char.to_owned() {
                    "  " => self = self.replace(char, " "),
                    _ => self = self.replace(char, "")
                };
            }
            self = String::from(self.trim());

            
        }
      return self;
    }
}