use crate::paragraph::Paragraph;
use crate::tag::Tag;
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
	//Getters
	pub fn get_count(&self) -> u32{
		self.count.clone()
	}
}