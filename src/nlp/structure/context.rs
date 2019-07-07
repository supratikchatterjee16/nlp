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
use crate::nlp::structure::Paragraph;
use crate::nlp::structure::Tag;
use std::collections::LinkedList;

pub struct Context{
	paragraphs : LinkedList<Paragraph>,
	title : String, //Optional
	count : u32,//Optional
	tags : LinkedList<Tag>
}

impl Context{
	//Common private functions
	//Common public functions
	//Constructors
	pub fn new() -> Context{
		Context{title : String::new(), count : 0, paragraphs : LinkedList::new(), tags : LinkedList::new()}
	}
	pub fn from_str(text : String) -> Context{
		let mut resp = Context::new();
		let mut temp = String::new();
		for ch in text.chars(){
			temp.push(ch);
			if ch == '\n'{
				if temp.len() > 0{
					resp.count = resp.count + 1;
					resp.paragraphs.push_back(Paragraph::from_str(temp));
				}
				temp = String::new();
			}
		}
		
		if temp.len() > 0{
			resp.count = resp.count + 1;
			resp.paragraphs.push_back(Paragraph::from_str(temp));
		}
		
		resp
	}
	//Setters
	pub fn set_title(&mut self, title : String){
		self.title = title;
	}
	//Getters
	pub fn get_count(&self) -> u32{
		self.count.clone()
	}
	
	pub fn get_title(&self) -> &String{&self.title}
	pub fn get_tags(&self) -> &LinkedList<Tag>{&self.tags}
}