pub fn hash_one(datum: &str) -> bool {
    let v: Vec<u8> = "bors".bytes().collect();
    println!("here be bytes {:?}", v);
    println!("here be datum{:?}", datum);
    true
}
#[test]
fn it_works() {
    assert!(hash_one("atest"));
}
