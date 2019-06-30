use sentence::Sentence;
use literal::Literal;
fn main(){
    let mut sentence = Sentence::from_str("Lorem52926 jnkjnasd 8967 9829 Ipsum dolores set amet karamba bitch nuggets.".to_string());
    println!("{}", sentence.to_string());
    println!("{}", sentence.get_types_string());
}
