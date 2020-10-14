use std::collections::HashMap;

use clap::{App, Arg};

use tokio::{fs::File, io::AsyncReadExt};

use serde_json::value::Value;

use log::{error, debug};

/// given a filename, reads file contents as String
async fn get_file_contents(filename: &str) -> Result<String, ()> {
    let mut file = File::open(filename)
        .await
        .map_err(|e| error!("Error opening file {}: {}", filename, e))?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .await
        .map_err(|e| error!("Error reading file {}: {}", filename, e))?;

    Ok(buf)
}

/// really ugly method to find the length of a number's string representation
fn get_str_len(n: usize) -> usize {
    let log = (n as f32).log(10_f32);
    let ceil = log.ceil();
    (ceil as usize) + if ceil == log { 1 } else { 0 }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    // init input args parser
    let matches = App::new("log_reader")
        .version("0.1")
        .author("Marco Napetti <marco.napetti@gmail.com>")
        .about("Reads a logfile and outputs per-type byte-counts")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    // get input filename, error is already managed by clap
    let filename = matches.value_of("INPUT").ok_or_else(|| unreachable!())?;

    let contents = get_file_contents(filename).await?;

    // split file contents by new line
    let mut dict = contents.lines()
        // filter empty lines
        .filter(|s| s.len() > 0)
        // sum per-type byte-count
        .fold(HashMap::new(), |mut dict, s| {
            // we use the string representation of type, this way we accept any json type
            let row_type = serde_json::from_str::<Value>(s)
                .map_err(|e| debug!("Error deserializing row: {}", e))
                .ok()
                .and_then(|json| json.get("type").map(Value::to_string));
            let entry = dict.entry(row_type).or_insert((0_usize, 0_usize));
            entry.0 += 1;
            entry.1 += s.len();
            dict
        });

    // print error checksum first
    if let Some((lines, bytes)) = dict.remove(&None) {
        println!("Found {} invalid log lines ({} bytes)", lines, bytes);
    }
    // print valid checksum
    println!("Found {} valid log types", dict.len());

    if dict.len() > 0 {
        // get max columns width
        let col1_width = dict.iter()
            .map(|(log_type, _)| log_type.as_ref().map(|s| s.len()).unwrap_or(0))
            .max()
            .unwrap_or(0)
            .max(4);// Type
        let col2_width = dict.iter()
            .map(|(_, (count, _))| get_str_len(*count))
            .max()
            .unwrap_or(0)
            .max(5);// Count
        let col3_width = dict.iter()
            .map(|(_, (_, bytes))| get_str_len(*bytes))
            .max()
            .unwrap_or(0)
            .max(5);// Bytes

        // prepare table headers
        println!("┏━{0:━>1$}━┳━{0:━>2$}━┳━{0:━>3$}━┓", "", col1_width, col2_width, col3_width);
        println!("┃ {3:0$} ┃ {4:1$} ┃ {5:2$} ┃", col1_width, col2_width, col3_width, "Type", "Count", "Bytes");
        println!("┡━{0:━>1$}━╇━{0:━>2$}━╇━{0:━>3$}━┩", "", col1_width, col2_width, col3_width);

        // print table body
        dict.into_iter().for_each(|(log_type, (count, bytes))| {
            println!("│ {3:0$} │ {4:1$} │ {5:2$} │", col1_width, col2_width, col3_width, log_type.unwrap_or_else(String::new), count, bytes);
        });

        // close table
        println!("└─{0:─>1$}─┴─{0:─>2$}─┴─{0:─>3$}─┘", "", col1_width, col2_width, col3_width);
    }

    Ok(())
}
