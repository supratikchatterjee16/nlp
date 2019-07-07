/*
* TITLE  : NLP
* DESCRIPTION : NLP PACKAGE FOR MORE FASTER AND EFFICIENT ANALYSIS AND UNDERSTANDING OF TEXTS.
* AUTHOR : SUPRATIK CHATTERJEE
* YEAR OF CREATION   : 2019
* YEAR OF COMPLETION : ----
*
* MODULE DESCRIPTON
* Contains the structure for tokenization of text.
* 
*/

use crate::nlp::structure::Tag;
use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub struct Token{
	content :String,
	tags: LinkedList<Tag>
}
impl Token{
	//Constructors
	pub fn new() -> Token{
		Token{content : String::new(), tags : LinkedList::new()}
	}
	pub fn from_str(text:String) -> Token{
		//This function generates the token that is to be used as a token everywhere...
		Token{content : text, tags : LinkedList::new()}
	}
	
	//Setters
	pub fn clear(&mut self){
		self.content = String::new();
		self.tags.clear();
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
		self.tags.push_back(Tag::create_new(tagger_id, tag_value))
	}
	
	//Getters
	pub fn get_content(&self) -> String{
		self.content.clone()
	}
	
	pub fn get_size(&self) -> usize{
		self.content.len()
	}
	
	pub fn get_tag(&self, tagger_id : String) -> String{
		let mut resp = String::from("Not found");
		for tag in &self.tags{
			if tag.get_id() == tagger_id{
				resp = tag.get_value()
			}
		}
		resp.clone()
	}
	
	pub fn get_all_tags(&self) -> &LinkedList<Tag>{
		&self.tags
	}
}
