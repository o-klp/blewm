#![feature(test)]
#![feature(std_misc)]
extern crate test;
extern crate KLPhash;

use KLPhash::hash_one;
use KLPhash::hash_two;
use KLPhash::hash_three;

#[derive(Debug)]
pub enum LookupError {
    FalsePositive
}

pub struct Bloom<'a> { 
    pub filter: Vec<u8>, 
    pub data: Vec<&'a str>,
}

impl<'a> Bloom<'a> {
    // insert a string value into the data store
    pub fn insert(&mut self, string: &'a str) -> usize {
        let max: u32 = self.filter.len() as u32;
        let index_1: usize = hash_one(string, max as u32) as usize;
        let index_2: usize = hash_two(string, max as u32) as usize;
        let index_3: usize = hash_three(string, max as u32) as usize;

        self.filter[index_1] = 1;
        self.filter[index_2] = 1;
        self.filter[index_3] = 1;

        self.data.push(string);
        self.data.len()
    }

    // query if there is a *chance* data store has string value
    fn query(&self, string: &str) -> bool {
        let max: u32 = self.filter.len() as u32;
        let index_1: usize = hash_one(string, max) as usize;
        let index_2: usize = hash_two(string, max) as usize;
        let index_3: usize = hash_three(string, max) as usize;

        if self.filter[index_1] == 0 {
            return false;
        }
        if self.filter[index_2] == 0 {
            return false;
        }
        if self.filter[index_3] == 0 {
            return false;
        }
        true
    }

    // check if data store *actually* contains string value    
    pub fn contains(&self, string: &str) -> Result<bool, LookupError> {
        if self.query(string) {
            if !self.data.contains(&string) {
               return Err(LookupError::FalsePositive);
            }
            return Ok(true);
        }
        Ok(false)
    }
}

#[cfg(test)]
mod hash_tests {
    use hash::hash_one;
    use hash::hash_two;
    use hash::hash_three;
    use test::Bencher;

    #[test]
    fn it_works() {
        // yes I cheated. well, this tests if fn is deterministic
        // see it_actually_works for more tests...

        assert_eq!(hash_one("Greystark", 255_u32), 218);
        assert_eq!(hash_one("Greystark", 255_u32), 218);
        assert_eq!(hash_two("Karstark", 255_u32), 142);
        assert_eq!(hash_two("Karstark", 255_u32), 142);
        assert_eq!(hash_three("Stark", 255_u32), 194);
        assert_eq!(hash_three("Stark", 255_u32), 194);
    }
    #[test]
    fn it_actually_works() {
        let north_houses = [
            "Amber" , "Bolton", "Condon", "Dustin", "Forrester",
            "Greystark", "Harclay", "Ironsmith", "Karstark", "Lake", "Marsh",
            "Norrey", "Overton", "Peat", "Pool", "Quagg", "Redbeard", "Reed",
            "Ryswell", "Slate", "Stane", "Stark", "Stout", "Thenn", "Umber",
            "Waterman", "Wells", "Whitehill", "Woodfoot", "Woods", "Woolfield",
            "Wull", "Wull"
            ];
        let mut northern_hash_one = Vec::new();
        let mut northern_hash_two = Vec::new();
        let mut northern_hash_three = Vec::new();

        for house in north_houses.iter() {
            northern_hash_one.push(hash_one(house, 255_u32));
            northern_hash_two.push(hash_two(house, 255_u32));
            northern_hash_three.push(hash_three(house, 255_u32));
        }

        // 33 elements and a range of 255 - a "well distributed" hash function
        // would hash one element every interval of 7.73
        // To test distribution we'll sort the  hashed values
        northern_hash_one.sort_by(|a, b| a.cmp(b));
        northern_hash_two.sort_by(|a, b| a.cmp(b));
        northern_hash_three.sort_by(|a, b| a.cmp(b));

        // & check there's a *relatively* consistent difference b/w elements
        // The better the hashing fn the smaller the interval
        // Let's start with 5 < x < 10 as the range - %30 away from "ideal"
        // ( 7.73 ± 2.27 )
        {
            let mut iter = northern_hash_one.iter().peekable();
            while let Some(num) = iter.next() {
                if iter.peek() != None {
                    let difference: u8 = *iter.peek().unwrap() - *num;
                    assert!(difference >= 5 || difference <= 10);
                }
            }
        }
        {
            let mut iter = northern_hash_two.iter().peekable();
            while let Some(num) = iter.next() {
                if iter.peek() != None {
                    let difference: u8 = *iter.peek().unwrap() - *num;
                    assert!(difference >= 5 || difference <= 10);
                }
            }
        }
        {
            let mut iter = northern_hash_three.iter().peekable();
            while let Some(num) = iter.next() {
                if iter.peek() != None {
                    let difference: u8 = *iter.peek().unwrap() - *num;
                    assert!(difference >= 5 || difference <= 10);
                }
            }
        }

        // to test for collisions we'll dedup and expect a difference of <= 2,
        // or 3% (33 elements with 1 intentional duplicate - Wull)
        let northern_one_len: usize = northern_hash_one.len();
        let northern_two_len: usize = northern_hash_two.len();
        let northern_three_len: usize = northern_hash_three.len();
        northern_hash_one.dedup();
        northern_hash_two.dedup();
        northern_hash_three.dedup();

        assert!(northern_one_len - northern_hash_one.len() <= 2);
        assert!(northern_two_len - northern_hash_two.len() <= 2);
        assert!(northern_three_len - northern_hash_three.len() <= 2);
    }
    #[bench]
    fn hash_one_bench(b: &mut Bencher) {
        b.iter(|| hash_one("Great Danton", 255_u32));
    }
    #[bench]
    fn hash_two_bench(b: &mut Bencher) {
        b.iter(|| hash_two("The Professor", 255_u32));
    }
    #[bench]
    fn hash_three_bench(b: &mut Bencher) {
        b.iter(|| hash_three("Cutter", 255_u32));
    }
}

