pub struct Literal{
	content : String,
	length : usize,
	numeric : bool,
	special : bool,
	error : bool
}
impl Literal{
	pub fn to_string(&self)  -> String{self.content.clone()}
	pub fn length(&self)   	 -> usize {self.length}
	pub fn is_numeric(&self) -> bool  {self.numeric}
	pub fn is_special(&self) -> bool  {self.special}
	pub fn is_error(&self)   -> bool  {self.error}
	
	pub fn clone(&self) -> Literal{
		Literal{content:self.to_string(),length:self.length(),numeric:self.is_numeric(),special:self.is_special(),error:self.is_error()}
	}
	
	pub fn is_name(&self) -> bool {
		match self.is_special() && self.is_numeric() == false{
			true => true,
			false => false
		}
	}
	
	pub fn new_word(txt : String) -> Literal{
		match txt.contains(|c:char| c.is_ascii_digit()){
			true => {Literal{content : String::new(), length : 0, numeric:false, special:false, error:true}},
			false => 
			{ 
				match txt.contains(|c:char|c.is_uppercase()){
					true => Literal{content : txt.clone(), length : txt.len(), numeric : false, special : true, error : false},	
					false => match txt.contains(|c:char|c.is_whitespace()){
						false => Literal{content : txt.clone(), length : txt.len(), numeric : false, special : false, error : false},
						true => Literal{content : String::new(), length : 0, numeric:false, special:false, error:true}
					}
				}
			}
		}
	}
	
	pub fn new_numeral(txt : String) -> Literal{
		match txt.contains(|c:char| c.is_ascii_alphabetic()||c.is_whitespace()){
			true => {Literal{content : String::new(), length : 0, numeric:false, special:false, error:true}},
			false => 
			{ 
				match txt.contains("."){
					true => Literal{content : txt.clone(), length : txt.len(), numeric : true, special : true, error : false},	
					false => Literal{content : txt.clone(), length : txt.len(), numeric : true, special : false, error : false}
				}
			}
		}
	}
	pub fn new(txt : String) -> Literal{
		let mut literal = Literal::new_word(txt.clone());
		match literal.is_error(){
			true => {
				literal = Literal::new_numeral(txt);
				match literal.is_error(){
					true => Literal{content : String::new(), length : 0, numeric:false, special:false, error:true},	
					false => literal.clone()
				}
				},
			false => {literal.clone()}
		}
	}
}