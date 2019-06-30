use literal::Literal;
use tag::Tag;
fn main(){
let mut boxed = Literal::new("Hello123".to_string());
boxed.add_tag(Tag::create_new("POS".to_string(),"Number".to_string()));
boxed.add_tag(Tag::create_new("CRAP".to_string(),"Rapper".to_string()));
println!("{}",boxed.get_type());
}
