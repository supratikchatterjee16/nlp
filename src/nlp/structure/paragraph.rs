use crate::nlp::structure::Sentence;
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

pub struct Paragraph{
	sentences : LinkedList<Sentence>,
	tags : LinkedList<Tag>
}

impl Paragraph{
	//Common private functions
	//Common public functions
	pub fn to_string() -> String{
		String::new()
	}
	//Constructors
	pub fn new() -> Paragraph{
		Paragraph{sentences : LinkedList::new(), tags : LinkedList::new()}
	}
	pub fn from_str(text : String) -> Paragraph{
		let mut temp = String::new();
		let arr = String::from("?.!");
		let mut flag = -1;
		let mut para = Paragraph::new();
		for ch in text.chars(){
			if ch.is_ascii_punctuation(){
				flag = -1;
				for ch2 in arr.chars(){
					if ch2 == ch{
						flag = 1;
					}
				}
				temp.push(ch);
				if flag > 0 {
					para.sentences.push_back(Sentence::from_str(temp));
					temp = String::new();
				}
				flag = -2;
			}
		}
		if flag == -2 && temp.len() > 0{
			para.sentences.push_back(Sentence::from_str(temp));
		}
		para
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
	
}