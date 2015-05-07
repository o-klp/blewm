pub fn hash_one(datum: &str) -> u8 {
    let nums: Vec<u8> = datum.bytes().collect();
    println!("here be bytes {:?}", nums);
    nums[0]
}
#[test]
fn it_works() {
    assert_eq!(hash_one("atest"), "atest".bytes().collect::<Vec<_>>()[0]);
}
