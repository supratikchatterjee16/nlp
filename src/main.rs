use sentence::Sentence;

fn main() {
    let sentence = Sentence::from_str("192.168.1.2 is a fine little IP address.".to_string());
    println!("{}",sentence.to_string());
}
