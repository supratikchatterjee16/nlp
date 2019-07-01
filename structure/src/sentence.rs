use crate::token::Token;
use crate::tag::Tag;
use std::collections::LinkedList;

pub struct Sentence{
	literals : LinkedList<Token>,
	tags : LinkedList<Tag>
}
impl Sentence{
	//Common private functions
	fn create_literal_list(text : String) -> LinkedList<Token>{
		let mut list : LinkedList<Token> = LinkedList::new();
		let mut temp = String::new();
		for ch in text.chars(){
			if ch.is_ascii_alphanumeric()&&!ch.is_ascii_whitespace(){
				temp.push(ch);
			}
			else{
				list.push_back(Token::from_str(temp.clone()));
				temp = String::new();
				temp.push(ch);
				list.push_back(Token::from_str(temp.clone()));
				temp = String::new();
			}
		}
		if temp.len() > 1{
			list.push_back(Token::from_str(temp.clone()));
		}
		list
	}
	//Common public functions
	pub fn to_string(&self) -> String{
		let mut temp = String::new();
		for text in &self.literals{
			temp.insert_str(temp.len(),&text.get_content());
		}
		temp
	}
	//Constructing functions
	pub fn new() -> Sentence{
		Sentence{literals : LinkedList::new(), tags : LinkedList::new()}
	}

	pub fn from_str(text : String) -> Sentence{
		Sentence{literals: Sentence::create_literal_list(text), tags : LinkedList::new()}
	}

	//Setters
	pub fn add_tag(&mut self, tag : Tag){
		self.tags.push_back(tag);
	}
	pub fn set_tag(&mut self, tagger_id : String, tag_value : String){
		//When not sure if id has been set already, use this to lower chances of redundancy
		//Checks and assign if not found
		for tag in &mut self.tags{
			if tag.get_id() == tagger_id{
				tag.set_value(tag_value);
				return;
			}
		}
		self.add_tag(Tag::create_new(tagger_id, tag_value))
	}
	//Getters
	pub fn get_nth_element(&self, n:u8) -> Option<Token>{
		let mut literal = Token::new();
		let mut x = 0;
		for literal in &self.literals{
			if x<=n{
				x = x+1;
			}
			else{return None;}
		}
		Some(literal)
	}
	pub fn get_nth_string(&self, n: u8) -> String{
		let mut temp = String::new();
		let mut x = 0;
		for text in &self.literals{
			if x <= n{
				temp.insert_str(temp.len(),&text.get_content());
				x = x+1;
			}
			else{break;}
		}
		if temp.len()==0{temp = String::from("Not found");}
		temp
	}

	pub fn get_nth_type(&self, n : u8) -> String{
		String::new()
	}
	pub fn get_types_string(&self) -> String{
		let mut temp = String::new();
		for text in &self.literals{
			temp.insert_str(temp.len(),&text.get_type().to_string());
			temp.insert_str(temp.len()," ");
		}
		temp
	}
}
