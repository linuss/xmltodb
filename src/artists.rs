use postgres::{Client, CopyInWriter, NoTls};
use quick_xml::de::from_reader;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct Artists {
    #[serde(rename = "$value")]
    artists: Vec<Artist>,
}

#[derive(Debug, Deserialize)]
struct Artist {
    id: Id,
    name: Name,
    realname: Option<RealName>,
    profile: Profile,
    data_quality: DataQuality,
    aliases: Option<Aliases>,
    members: Option<Members>,
    groups: Option<Groups>,
    urls: Option<Urls>,
    namevariations: Option<NameVariations>,
}

#[derive(Debug, Deserialize)]
struct NameVariations {
    #[serde(rename = "$value")]
    namevariations: Option<Vec<Name>>,
}

#[derive(Debug, Deserialize)]
struct Urls {
    #[serde(rename = "$value")]
    urls: Vec<Url>,
}

#[derive(Debug, Deserialize)]
struct Url {
    #[serde(rename = "$value")]
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Id {
    #[serde(rename = "$value")]
    id: i32,
}

#[derive(Debug, Deserialize)]
struct Name {
    id: Option<i32>,
    #[serde(rename = "$value")]
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RealName {
    #[serde(rename = "$value")]
    real_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Profile {
    #[serde(rename = "$value")]
    profile: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DataQuality {
    #[serde(rename = "$value")]
    data_quality: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Aliases {
    #[serde(rename = "$value")]
    aliases: Option<Vec<Name>>,
}

#[derive(Debug, Deserialize)]
struct Members {
    #[serde(rename = "$value")]
    members: Option<Vec<Member>>,
}

#[derive(Debug, Deserialize)]
enum Member {
    #[serde(rename = "id")]
    Id(Id),
    #[serde(rename = "name")]
    Name(Name),
}

#[derive(Debug, Deserialize)]
struct Groups {
    #[serde(rename = "$value")]
    groups: Option<Vec<Group>>,
}

#[derive(Debug, Deserialize)]
struct Group {
    id: i32,
    #[serde(rename = "$value")]
    name: String,
}

fn write_artist_links(
    alias_writer: &mut CopyInWriter,
    group_writer: &mut CopyInWriter,
    artist: &Artist,
) {
    match &artist.aliases {
        Some(Aliases {
            aliases: Some(aliases),
        }) => {
            for alias in aliases {
                write!(
                    alias_writer,
                    "\"{}\",\"{}\",\"{}\"\n",
                    artist.id.id,
                    &alias.id.map_or("".to_string(), |i| i.to_string()),
                    alias
                        .name
                        .as_ref()
                        .unwrap_or(&"".to_string())
                        .replace("\"", "\"\""),
                )
                .unwrap();
            }
        }
        _ => {}
    }

    match &artist.groups {
        Some(Groups {
            groups: Some(groups),
        }) => {
            for group in groups {
                write!(
                    group_writer,
                    "\"{}\",\"{}\",\"{}\"\n",
                    artist.id.id,
                    group.id,
                    group.name.replace("\"", "\"\"")
                )
                .unwrap();
            }
        }
        _ => {}
    }

    match &artist.members {
        Some(Members {
            members: Some(members),
        }) => {
            for member in members {
                match member {
                    Member::Name(name) => {
                        write!(
                            group_writer,
                            "\"{}\",\"{}\",\"{}\"\n",
                            artist.id.id,
                            &name.id.map_or("".to_string(), |i| i.to_string()),
                            name.name
                                .as_ref()
                                .unwrap_or(&"".to_string())
                                .replace("\"", "\"\"")
                        )
                        .unwrap();
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn write_artist_name_variations(name_varation_writer: &mut CopyInWriter, artist: &Artist) {
    match &artist.namevariations {
        Some(NameVariations {
            namevariations: Some(name_variations),
        }) => {
            for name in name_variations {
                match &name.name {
                    Some(name) => {
                        write!(
                            name_varation_writer,
                            "\"{}\",\"{}\"\n",
                            name.replace("\"", "\"\""),
                            artist.id.id
                        )
                        .unwrap();
                    }
                    None => {}
                }
            }
        }
        _ => {}
    }
}

fn write_artist_urls(url_writer: &mut CopyInWriter, artist: &Artist) {
    match &artist.urls {
        Some(urls) => {
            for url in &urls.urls {
                match &url.url {
                    Some(url) => write!(
                        url_writer,
                        "\"{}\",\"{}\"\n",
                        url.replace("\"", "\"\""),
                        artist.id.id
                    )
                    .unwrap(),
                    None => {}
                }
            }
        }
        None => {}
    };
}

fn write_artist(artist_writer: &mut CopyInWriter, artist: &Artist) {
    let Artist {
        id,
        name,
        realname,
        profile,
        data_quality,
        aliases: _,
        members: _,
        groups: _,
        urls: _,
        namevariations: _,
    } = artist;

    let rn = match realname {
        Some(RealName {
            real_name: Some(realname),
        }) => realname.replace("\"", "\"\""),
        _ => "".to_string(),
    };

    let p = match &profile.profile {
        Some(profile) => profile.replace("\"", "\"\""),
        _ => "".to_string(),
    };

    match &name.name {
        Some(name) => {
            write!(
                artist_writer,
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                &id.id.to_string(),
                &name.replace("\"", "\"\""),
                rn,
                p,
                &data_quality
                    .data_quality
                    .as_ref()
                    .unwrap_or(&"".to_string())
            )
            .unwrap();
        }
        None => {}
    };
}

pub fn load_artists() {
    let connection_string = "host=localhost user=linus dbname=discogs";
    let mut client = Client::connect(connection_string, NoTls).unwrap();
    let tables_structure = fs::read_to_string("artists.sql").unwrap();
    client.batch_execute(&tables_structure).unwrap();

    let file = File::open("/home/linus/Downloads/discogs/artists.xml").unwrap();
    let file = BufReader::new(file);

    let artists: Artists = from_reader(file).unwrap();
    println!(
        "Succesfully parsed {} artists",
        artists.artists.len().to_string()
    );

    let mut artist_writer = Client::connect(connection_string, NoTls).unwrap();
    let mut artist_writer = artist_writer
        .copy_in("COPY artists FROM stdin CSV")
        .unwrap();

    for artist in &artists.artists {
        write_artist(&mut artist_writer, &artist);
    }
    artist_writer.finish().unwrap();

    let mut artist_urls_writer = Client::connect(connection_string, NoTls).unwrap();
    let mut artist_urls_writer = artist_urls_writer
        .copy_in("COPY artist_urls FROM stdin CSV")
        .unwrap();

    for artist in &artists.artists {
        write_artist_urls(&mut artist_urls_writer, &artist);
    }
    artist_urls_writer.finish().unwrap();

    let mut artist_name_variations_writer = Client::connect(connection_string, NoTls).unwrap();
    let mut artist_name_variations_writer = artist_name_variations_writer
        .copy_in("COPY artist_name_variations FROM stdin CSV")
        .unwrap();
    for artist in &artists.artists {
        write_artist_name_variations(&mut artist_name_variations_writer, &artist);
    }

    artist_name_variations_writer.finish().unwrap();

    let mut alias_writer = Client::connect(connection_string, NoTls).unwrap();
    let mut alias_writer = alias_writer
        .copy_in("COPY artist_aliases FROM stdin CSV")
        .unwrap();

    let mut group_writer = Client::connect(connection_string, NoTls).unwrap();
    let mut group_writer = group_writer
        .copy_in("COPY artist_group FROM stdin CSV")
        .unwrap();

    for artist in &artists.artists {
        write_artist_links(&mut alias_writer, &mut group_writer, &artist);
    }
    alias_writer.finish().unwrap();
    group_writer.finish().unwrap();

    println!("Saved all artists!");
}
