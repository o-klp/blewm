extern crate mylib;

use mylib::hash_one;
use mylib::hash_two;
use mylib::hash_three;

pub fn main() {
    let north_houses = [ 
        "Amber" , "Bolton", "Condon", "Dustin", "Forrester", "Greystark",
        "Harclay", "Ironsmith", "Karstark", "Lake", "Marsh", "Norrey",
        "Overton", "Peat", "Pool", "Quagg", "Redbeard", "Reed", "Ryswell",
        "Slate", "Stane", "Stark", "Stout", "Thenn", "Umber", "Waterman",
        "Wells", "Whitehill", "Woodfoot", "Woods", "Woolfield", "Wull", "Wull" 
    ];

    for house in north_houses.iter() {
        //println!("and here at the turn I leave you bourden{:?} {:?}",
        //         house, hash_one(house));
        //println!("\t\tAngier?????????\t\t{:?} {:?}", house, hash_two(house));
        println!("langford double {:?} {:?}", house, hash_three(house));
    }
}

