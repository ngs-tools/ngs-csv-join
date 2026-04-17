use std::{collections::{BTreeMap, HashMap}, env, fs::File};

use csv::Reader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args: Vec<String> = if args.len() == 1 {
        println!("Me want CSV file(s) !!");
        return
    } else {
        args[1..].to_vec()
    };

    println!("CSV {args:?}");

    // Read files data into memory
    let data_map = BTreeMap::new();
    let mut map = HashMap::new();
    for p in args.iter() {
        let data = read_file(p);
        map.insert(p.clone(), data);
    }

    for (k, mut rdr) in map {
        println!("{k}");

        let headers = rdr.headers().unwrap();
        println!("{:?}", headers);

        // Iterate over records (skips header by default)
        for result in rdr.records() {
            let record = result.unwrap();
            // Access fields by index

            println!("{:?}", record);
        }
    }
}

struct NgsData {
    sequ_id: String,
    count: Option<usize>,
    transport_name: Option<String>
}

fn read_file(p: &str) -> Reader<File> {
    Reader::from_path(p).unwrap()
}
