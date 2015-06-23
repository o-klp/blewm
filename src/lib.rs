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

