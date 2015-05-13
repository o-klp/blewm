extern crate mylib;

use mylib::Bloom;

pub fn main() {
    let mut bloom = Bloom {
        filter: vec![0; 1000],
        data: Vec::new(),
    };

    let data = vec![
        "Aenar", "Gaemon", "Daenys", "Aegon", "Elaena", "Maegon", "Aerys",
        "Aelyx", "Baelon", "Daemion", "Valaena Velaryon", "Aerion", "Visenya",
        "Aegon I", "Rhaenys", "Maegor", "daughter", "Aenys", "Alyssa Velaryon",
        "Aegon", "Rhaena", "Viserys", "Jaehaerys I", "Alysanne", "Aerea",
        "Rhalla", "Aemon", "Jocelyn Baratheon", "Baelon", "Rodrik Arryn",
        "Daella", "Saera", "Rhaenys", "Aemma Arryn", "Viserys I",
        "Alicent Hightower", "Laena Velaryon", "Daemon", "Rhaenyra",
        "Laenor Velaryon", "Aegon II", "Helaena", "Aemond", "Daeron", "Baela",
        "Rhaena", "Jaehaera", "Aegon III", "Daenaera Velaryon", "Viserys II",
        "Visenya", "Jacaerys Velaryon", "Lucerys Velaryon", "Joffrey Velaryon",
        "Jaehaerys", "Aegon III", "Jaehaera duplicate", "Maelor", "Daeron I",
        "Rhaena", "Elaena", "Baelor", "Daena", "Aegon IV", "Naerys", "Aemon",
        "Daemon Blackfyre", "Aegor Rivers", "Brynden Rivers", "Shiera Seastar",
        "Daeron II", "Myriah Martell", "Daenerys", "Maron Martell",
        "Aegon Blackfyre", "Aemon Blackfyre", "Daemon II Blackfyre",
        "Haegon Blackfyre", "Baelor", "Aerys I", "Aelinor", "Rhaegel", "Maekar",
        "Valarr", "Matarys", "Daeron", "Aerion", "Maester Aemon", "Aegon V",
        "Rhae", "Daella", "Duncan", "Jaehaerys II", "Rhaelle",
        "Ormund Baratheon", "Aerys II", "Rhaella", "Elia Martell", "Rhaegar",
        "Viserys III", "Drogo", "Daenerys", "Rhaenys", "Aegon", "Rhaego"
            ];

    let non_data = vec![
        "Benjen", "Rickon", "Bennard", "Lynara", "Cregan", "Arra Norrey",
        "Bennard", "Rickon", "Jeyne Manderly", "Barthogan", "Jonnel",
        "Robyn Ryswell", "Alys Karstark", "Brandon", "Wyalla Fenn", "Edric",
        "Serena", "Sansa", "Myriame Manderly", "Rodwell", "Arsa", "Beron",
        "Lorra Royce", "Lonnel Snow", "Aregelle", "Torrhen", "Cregard",
        "Arrana", "Donnor", "Lyanne Glover", "Willam", "Melantha Blackwood",
        "Rodrik", "Arya Flint", "Berena", "Alysanne", "Brandon", "Errold",
        "Artos", "Lysara Karstark", "Brandon", "Edwyle", "Marna Locke",
        "Jocelyn", "Brandon", "Benjen", "Rickard", "Lyarra", "Branda",
        "Brandon", "Catelyn Tully", "Eddard", "Unknown", "Lyanna", "Benjen",
        "Robb I", "Jeyne Westerling", "Sansa", "Tyrion Lannister", "Arya",
        "Bran", "Rickon", "Jon Snow"
            ];

    for name in data {
        bloom.insert(name);
    }

    let mut false_positives: u8 = 0;
    let mut duplicates: u8 = 0;
    for name in non_data {
        let result = bloom.contains(name);
        if result.is_err() {
            false_positives += 1;
        } else {
            if result.unwrap() {
                duplicates += 1;
            } else {
                println!("don't have a - {:?}", name);
            }
        }
    }

    println!("{:?} duplicates", duplicates);
    println!("{:?} false positives", false_positives);
    println!("{:?} elements in bloom filter", bloom.data.len());
}

