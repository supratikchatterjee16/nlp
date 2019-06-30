use tag::Tag;

fn main(){
let mut tag = Tag::new();
tag.set_id("POS".to_string());
tag.set_value("Noun".to_string());
println!("{}",tag.to_string());
}
