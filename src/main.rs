use std::{env, fs::File};

use csv::Reader;
use indexmap::IndexMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (out_file, args) = if args.len() <= 2 {
        println!("{} <out_file> input1.csv input2.csv ....", args[0]);
        return;
    } else {
        (args[1].clone(), args[2..].to_vec())
    };

    println!("CSV {args:?}, with output to {out_file}");

    // Read files data into memory
    let mut data_map = IndexMap::new();
    let mut map = IndexMap::new();
    for p in args.iter() {
        let data = read_file(p);
        map.insert(p.clone(), data);
    }

    for (k, rdr) in map.iter_mut() {
        println!("{k}");

        let headers = rdr.headers().unwrap().clone();
        println!("{:?}", headers);

        // Iterate over records (skips header by default)
        for result in rdr.records() {
            let record = match result {
                Err(e) => {
                    println!("Error: {e:?}");
                    continue;
                }
                Ok(r) => r,
            };
            // Access fields by index

            println!("{:?}", record);
            for (r, rec) in record.iter().enumerate() {
                let header = headers.get(r);
                let header = match header {
                    None => continue,
                    Some(h) => h,
                };

                let v = data_map.get_mut(header);

                match v {
                    None => {
                        let v = vec![rec.to_string()];
                        // create a new empty vector in the map
                        data_map.insert(header.to_string(), v);
                    }
                    Some(v) => v.push(rec.to_string()),
                };
            }
        }
    }

    write_column_data(&data_map, out_file);
}

fn write_column_data(columns: &IndexMap<String, Vec<String>>, out_file: String) {
    let fields: Vec<&str> = columns.keys().map(|k| k.as_str()).collect();

    let mut iters: Vec<std::slice::Iter<'_, String>> =
        fields.iter().map(|f| columns[*f].iter()).collect();

    // let mut wtr = csv::Writer::from_writer(std::io::stdout().lock());
    let mut wtr = csv::Writer::from_path(out_file).unwrap();
    wtr.write_record(fields.iter()).unwrap();
    'OUTER: loop {
        for it in iters.iter_mut() {
            let Some(value) = it.next() else { break 'OUTER };
            wtr.write_field(value).unwrap();
        }
        wtr.write_record(None::<&[u8]>).unwrap();
    }
    wtr.flush().unwrap();
}

fn read_file(p: &str) -> Reader<File> {
    Reader::from_path(p).unwrap()
}
