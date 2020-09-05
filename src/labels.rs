use postgres::{Client, NoTls};
use serde_xml_rs::from_reader;
use serde_xml_rs::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct Labels {
    #[serde(rename = "$value")]
    labels: Vec<Label>,
}

#[derive(Debug, Deserialize)]
struct Label {
    id: Id,
    name: Name,
    contactinfo: Option<ContactInfo>,
    profile: Option<Profile>,
    data_quality: DataQuality,
    urls: Option<Urls>,
    sublabels: Option<SubLabels>,
}

#[derive(Deserialize, Debug)]
struct Id {
    #[serde(rename = "$value")]
    id: i32,
}

#[derive(Deserialize, Debug)]
struct Name {
    #[serde(rename = "$value")]
    name: String,
}

#[derive(Deserialize, Debug)]
struct ContactInfo {
    #[serde(rename = "$value")]
    contact_info: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Profile {
    #[serde(rename = "$value")]
    profile: Option<String>,
}

#[derive(Deserialize, Debug)]
struct DataQuality {
    #[serde(rename = "$value")]
    data_quality: String,
}

#[derive(Deserialize, Debug)]
struct Urls {
    #[serde(rename = "$value")]
    urls: Option<Vec<Url>>,
}

#[derive(Deserialize, Debug)]
struct Url {
    #[serde(rename = "$value")]
    url: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SubLabels {
    #[serde(rename = "$value")]
    sublabels: Option<Vec<SubLabel>>,
}

#[derive(Deserialize, Debug)]
struct SubLabel {
    id: i32,
}

fn save_to_db(client: &mut Client, label: &Label) {
    let Label {
        id,
        name,
        contactinfo,
        profile,
        data_quality,
        urls,
        sublabels: _,
    } = label;

    client
        .execute(
            "INSERT INTO labels (id, name, contact_info, profile, data_quality)
                    VALUES ($1, $2, $3, $4, $5)",
            &[
                &id.id,
                &name.name,
                &contactinfo.as_ref().map(|c| &c.contact_info),
                &profile.as_ref().map(|x| &x.profile),
                &data_quality.data_quality,
            ],
        )
        .unwrap();

    match urls {
        Some(Urls { urls: Some(urls) }) => {
            for url in urls {
                match &url.url {
                    Some(url) => {
                        client
                            .execute(
                                "INSERT INTO label_urls (url, label_id) VALUES ($1, $2)",
                                &[&url, &id.id],
                            )
                            .unwrap();
                    }
                    None => {}
                }
            }
        }
        _ => {}
    };
}

fn save_sublabels(client: &mut Client, label: &Label) {
    match &label.sublabels {
        Some(SubLabels {
            sublabels: Some(sublabels),
        }) => {
            for sublabel in sublabels {
                client
                    .execute(
                        "UPDATE labels SET parent_label = $1 WHERE id = $2",
                        &[&label.id.id, &sublabel.id],
                    )
                    .unwrap();
            }
        }
        _ => {}
    }
}

pub fn load_labels() {
    let file = File::open("/home/linus/Downloads/discogs/discogs_20200806_labels.xml").unwrap();
    //let file = File::open("label.xml").unwrap();
    let file = BufReader::new(file);

    let labels: Result<Labels, Error> = from_reader(file);

    let mut client = Client::connect("host=localhost user=linus dbname=discogs", NoTls).unwrap();

    let tables_structure = fs::read_to_string("labels.sql").unwrap();
    client.batch_execute(&tables_structure).unwrap();

    match labels {
        Ok(ls) => {
            for label in &ls.labels {
                save_to_db(&mut client, &label);
            }
            for label in &ls.labels {
                save_sublabels(&mut client, &label);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
