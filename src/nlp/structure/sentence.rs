use crate::nlp::structure::Token;
/**
* TITLE  : NLP
* DESCRIPTION : NLP PACKAGE FOR MORE FASTER AND EFFICIENT ANALYSIS AND UNDERSTANDING OF TEXTS.
* AUTHOR : SUPRATIK CHATTERJEE
* YEAR OF CREATION   : 2019
* YEAR OF COMPLETION : ----
*
* MODULE DESCRIPTON
* 
* 
*/
use crate::nlp::structure::Tag;
use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub struct Sentence{
	text : String,
	delimiter : String,
	tokens : LinkedList<Token>,
	tags : LinkedList<Tag>,
	is_tokenized : bool
}

impl Sentence{
	//Common private functions
	//Common public functions
	pub fn tokenize(&mut self){
		let mut token = String::new();
		for i in self.text.chars(){
			let mut flag = false;
			for j in self.delimiter.chars(){
				if i==j{
					flag = true;
				}
			}
			if flag == true{
				self.tokens.push_back(Token::from_str(token.clone()));
				token.clear();
				token.insert(0,i);
				self.tokens.push_back(Token::from_str(token.clone()));
				token.clear();
			}
			else { token.insert(token.len(), i); }
		}
		if token.len() > 0{
			self.tokens.push_back(Token::from_str(token.clone()));
		}
	}
	//Constructing functions
	pub fn new() -> Sentence{
		Sentence{tokens : LinkedList::new(), tags : LinkedList::new(), text : String::new(), delimiter : String::from(" .!?,;\'\"".to_string()), is_tokenized : false}
	}
	
	pub fn from_str(text : String) -> Sentence{
		Sentence{text : text, tokens: LinkedList::new(), tags : LinkedList::new(), delimiter : String::from(" ".to_string()), is_tokenized : false}
	}
	 
	//Setters
	pub fn set_tag(&mut self, tagger_id : String, tag_value : String){
		//When not sure if id has been set already, use this to lower chances of redundancy
		//Checks and assign if not found
		for tag in &mut self.tags{
			if tag.get_id() == tagger_id{
				tag.set_value(tag_value);
				return;
			}
		}
		self.tags.push_back(Tag::create_new(tagger_id, tag_value));
	}
	
	pub fn set_delimiter(&mut self, delim : String){
		self.delimiter = delim;
	}
	
	//Getters
	pub fn get_tokens(&self) -> &LinkedList<Token>{
		&self.tokens
	}//Only borrow, ownership reserved to the object
	pub fn get_tags(&self) -> &LinkedList<Tag>{
		&self.tags
	}
	pub fn get_tag(&self, id : String) -> Option<Tag>{
		for tag in &self.tags{
			if tag.get_id() == id{
				return Some(tag.clone());
			}
		}
		None
	}
	pub fn get_token(&self, index : usize) -> Option<Token>{
		let mut x = 0;
		for token in &self.tokens{
			if x == index{
				return Some(token.clone());
			}
			x = x + 1;
		}
		None
	}
}
