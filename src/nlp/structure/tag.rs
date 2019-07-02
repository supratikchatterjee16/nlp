pub struct Tag{
	tagger_id : String,
	class : String
}

impl Tag{
	//Tags are a create and reference element.

	//Constructors
	pub fn create_new(id : String, text : String) -> Tag{
		Tag{tagger_id : id, class : text}
	}
	pub fn new() -> Tag{
		Tag{tagger_id : String::new(), class : String::new()}
	}

	//Setters
	pub fn set_id(&mut self, id : String){
		self.tagger_id = id;
	}
	pub fn set_value(&mut self, val : String){
		self.class = val;
	}

	//Getters
	pub fn get_id(&self) -> String{
		self.tagger_id.clone()
	}
	pub fn get_value(&self) -> String{
		self.class.clone()
	}
	pub fn to_string(&self) -> String{
		let mut res = String::new();
		res.insert_str(0,&self.class);
		res.insert_str(res.len()," <");
		res.insert_str(res.len(),&self.tagger_id);
		res.insert_str(res.len(),">");
		res
	}
}
