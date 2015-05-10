extern crate mylib;

use mylib::hash_one;
use mylib::hash_two;
use mylib::hash_three;

pub fn main() {
    let mut data_store: Vec<&str> = Vec::new();
    let mut bloom_filter: Vec<u8> = vec![0; 255];
    let north_houses = [ 
        "Amber" , "Bolton", "Condon", "Dustin", "Forrester", "Greystark",
        "Harclay", "Ironsmith", "Karstark", "Lake", "Marsh", "Norrey",
        "Overton", "Peat", "Pool", "Quagg", "Redbeard", "Reed", "Ryswell",
        "Slate", "Stane", "Stark", "Stout", "Thenn", "Umber", "Waterman",
        "Wells", "Whitehill", "Woodfoot", "Woods", "Woolfield", "Wull", "Wull" 
    ];

    // insert function takes a value, changes values at hashed indices
    // query fn takes value check indecies
    fn insert(string: &'static str,
              blewm: &mut Vec<u8>, tome: &mut Vec<&str>) -> bool {
        let index_1: usize = hash_one(string) as usize;
        let index_2: usize = hash_two(string) as usize;
        let index_3: usize = hash_three(string) as usize;

        blewm[index_1] = 1;
        blewm[index_2] = 1;
        blewm[index_3] = 1;

        tome.push(string);
        true
    }

    fn query(string: &str, blewm: &mut Vec<u8>) -> bool {
        let index_1: usize = hash_one(string) as usize;
        let index_2: usize = hash_two(string) as usize;
        let index_3: usize = hash_three(string) as usize;

        if blewm[index_1] == 0 {
            return false;
        }
        if blewm[index_2] == 0 {
            return false;
        }
        if blewm[index_3] == 0 {
            return false;
        }

        true
    }

    fn contains(string: &str,
                blewm: &mut Vec<u8>, tome: &mut Vec<&str>) -> bool {
        if query(string, blewm) {
            if !tome.contains(&string) {
                panic!("False positive");
            }
            return true;
        }
        false
    }

    insert(north_houses[0], &mut bloom_filter, &mut data_store);
    insert(north_houses[1], &mut bloom_filter, &mut data_store);
    insert(north_houses[5], &mut bloom_filter, &mut data_store);
    insert(north_houses[7], &mut bloom_filter, &mut data_store);
    insert(north_houses[17], &mut bloom_filter, &mut data_store);

    println!("inserted {:?} {:?} {:?} {:?} {:?}", north_houses[0],
            north_houses[1], north_houses[5], north_houses[7],
            north_houses[17]);

    println!("querying {:?} - {:?}", north_houses[5], query(north_houses[5],
            &mut bloom_filter));
    println!("querying {:?} - {:?}",
             "Aemon", query("Aemon", &mut bloom_filter));
    println!("contains {:?} ? {:?}",
             "john", contains("john", &mut bloom_filter, &mut data_store));
}

