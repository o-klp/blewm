extern crate mylib;

use mylib::Bloom;

pub fn main() {
    let mut bloom = Bloom {
        filter: vec![0; 255],
        data: Vec::new(),
    };

    bloom.insert("Stark");
    assert!(bloom.contains("Stark"));
    bloom.insert("Karstark");
    assert!(bloom.contains("Karstark"));
    assert!(!bloom.contains("Ned"));
}

