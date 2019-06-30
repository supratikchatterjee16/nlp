use literal::Literal;

fn main() {
    let literal = Literal::new("192. 168".to_string());
    println!("{}",literal.is_error());
}
