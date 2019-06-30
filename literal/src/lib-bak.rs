pub struct Literal{
	content :String,
	category : i8 
	//Categories:
	//0 int, 1 words, 2 alphanumeric, 3 whitespace
	//4 punctuator, 5 end-punctuator, 6 special character
}
impl Literal{
	fn create_new(text : String, cat : i8 )-> Box<Literal>{
		Box::new(Literal{content : text,category : cat})
	}
	pub fn get_content(&self) -> String{
		self.content.clone()
	}
	pub fn get_type(&self) -> i8{
		self.category
	}
	pub fn get_size(&self) -> usize{
		self.content.len()
	}
	pub fn is_number(&self) -> bool{
		match self.category==0{
			true => true,
			false => false
		}
	}
	pub fn is_word(&self) -> bool{
		match self.category == 1{
			true => true,
			false => false
		}
	}
	pub fn is_alphanumeric(&self) -> bool{
		match self.category == 2{
			true => true,
			false => false
		}
	}
	pub fn is_whitespace(&self) -> bool{
		match self.category == 3{
			true => true,
			false => false
		}
	}
	pub fn is_puntuator(&self) -> bool{
		match self.category == 4|| self.category == 5{
			true => true,
			false => false
		}
	}
	pub fn is_end_puntuator(&self) -> bool{
		match self.category == 5{
			true => true,
			false => false
		}
	}
	pub fn is_special_char(&self) -> bool{
		match self.category == 6{
			true => true,
			false => false
		}
	}
	pub fn is_error(&self) -> bool{
		match self.category<0{
			true => true,
			false => false
		}
	}
	pub fn new(text:String) -> Box<Literal>{
		//This function generates the token that is to be used as a token everywhere...
		let mut kind : i8 = -1;
		
		for character in text.chars(){
			if character.is_numeric(){
				if kind==0||kind==-1{ 		kind = 0; }
				else if kind==1{ 	kind = 2; }
				else{
					kind = -2;
					break;
				}
			}
			else if character.is_alphabetic(){
				if kind==-1||kind == 1{ kind = 1; }
				else if kind==0{ kind = 2; }
				else{
					kind = -2;
					break;
				}
			}
			else if character.is_ascii_whitespace(){
				if kind==-1||kind ==3{ kind = 3; }
				else{
					kind = -2;
					break;
				}
			}
			else if character.is_ascii_punctuation(){
				if kind==-1{
					if character=='.'||character == '!'||character=='?'{
						kind = 5;
					}
					else{
						kind = 4;
					}
				}
				else{
					kind = -2;
					break;
				}
				}
			else{
				if kind==-1{
					kind = 6;
				}
				else{
					kind = -2;
					break;
				}
			}
		}
		//println!("{}",kind);
		Literal::create_new(text,kind)
	}
}
