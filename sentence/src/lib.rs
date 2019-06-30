use literal::Literal;
use tag::Tag;
use std::collections::LinkedList;

pub struct Sentence{
	literals : LinkedList<Literal>,
	tags : LinkedList<Tag>
}
impl Sentence{
	//Common private functions
	fn create_literal_list(text : String) -> LinkedList<Literal>{
		let mut list : LinkedList<Literal> = LinkedList::new();
		let mut temp = String::new();
		for ch in text.chars(){
			if ch.is_ascii_alphanumeric()&&!ch.is_ascii_whitespace(){
				temp.push(ch);
			}
			else{
				list.push_back(Literal::new(temp.clone()));
				temp = String::new();
				temp.push(ch);
				list.push_back(Literal::new(temp.clone()));
				temp = String::new();
			}
		}
		if temp.len() > 1{
			list.push_back(Literal::new(temp.clone()));
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
	
	//Getters
	pub fn get_nth_element(&self, n:u8) -> Option<Literal>{
		let mut literal = Literal::new();
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
			if(x<=n){
				temp.insert_str(temp.len(),&text.get_content());
				x = x+1;
			}
			else{break;}
		}
		if temp.len()<0{temp = String::from("Not found");}
		temp
	}

	pub fn get_nth_type(&self, n : u8) -> String{

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
