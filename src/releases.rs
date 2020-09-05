use postgres::{Client, NoTls};
use quick_xml::de::from_reader;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::time::SystemTime;

#[derive(Debug, Deserialize)]
struct Releases {
    #[serde(rename = "$value")]
    releases: Vec<Release>,
}

#[derive(Debug, Deserialize)]
struct Release {
    id: i32,
    status: String,
    title: Option<Title>,
    country: Option<Country>,
    released: Option<Released>,
    notes: Option<Notes>,
    data_quality: String,
    master_id: Option<MasterId>,
}

#[derive(Debug, Deserialize)]
struct Title {
    #[serde(rename = "$value")]
    title: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Country {
    #[serde(rename = "$value")]
    country: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Released {
    #[serde(rename = "$value")]
    released: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Notes {
    #[serde(rename = "$value")]
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MasterId {
    #[serde(rename = "$value")]
    master_id: i32,
}

pub fn load_releases() {
    let connection_string = "host=localhost user=linus dbname=discogs";
    let mut client = Client::connect(connection_string, NoTls).unwrap();
    let tables_structure = fs::read_to_string("releases.sql").unwrap();
    client.batch_execute(&tables_structure).unwrap();

    let file = File::open("/home/linus/Downloads/discogs/discogs_20200806_releases.xml").unwrap();
    let file = BufReader::new(file);

    println!("Gonna parse a gazillion releases now");
    let start = SystemTime::now();
    let releases: Releases = from_reader(file).unwrap();
    let stop = SystemTime::now();
    println!(
        "Succesfully parsed {} releases in {:?}",
        releases.releases.len().to_string(),
        stop.duration_since(start).unwrap()
    );
}
