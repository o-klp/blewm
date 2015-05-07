/// a good hashing function is well-distributed & deterministic
/// to achieve those qualities, there can't be too many destructive operations
/// (we want to avoid situations that trend to a certain range of values)
/// and must reduce (fold) to take advantage of a string's uniqueness
/// hash_one will take the difference
pub fn hash_one(datum: &str) -> u8 {
    let nums: Vec<u8> = datum.bytes().collect();
    /// we'll multiply each byte by the difference b/w it and the next byte
    /// (as a decimal) and then clear out the decimals
    let mut hashedDatum: f32 = num[0] as f32;
    println!("here be bytes {:?}", nums);
    nums[0]
}
#[test]
fn it_works() {
    assert_eq!(hash_one("atest"), "atest".bytes().collect::<Vec<_>>()[0]);
}
