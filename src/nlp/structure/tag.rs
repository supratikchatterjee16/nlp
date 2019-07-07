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
#[derive(Debug, Clone)]
pub struct Tag{
	id : String,
	value : String
}

impl Tag{
	//Tags are a create and reference element.

	//Constructors
	pub fn create_new(id : String, text : String) -> Tag{
		Tag{id : id, value : text}
	}
	pub fn new() -> Tag{
		Tag{id : String::new(), value : String::new()}
	}

	//Setters
	pub fn set_id(&mut self, id : String){
		self.id = id;
	}
	pub fn set_value(&mut self, val : String){
		self.value = val;
	}

	//Getters
	pub fn get_id(&self) -> String{
		self.id.clone()
	}
	pub fn get_value(&self) -> String{
		self.value.clone()
	}
	pub fn to_string(&self) -> String{
		let mut res = String::new();
		res.insert_str(0,&self.value);
		res.insert_str(res.len()," <");
		res.insert_str(res.len(),&self.id);
		res.insert_str(res.len(),">");
		res
	}
}
