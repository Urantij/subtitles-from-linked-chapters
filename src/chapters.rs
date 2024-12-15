use serde::Deserialize;
use time::macros::format_description;
use time::Time;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Chapters {
    pub edition_entry: EditionEntry,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct EditionEntry {
    pub chapter_atom: Vec<ChapterAtom>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ChapterAtom {
    pub chapter_time_start: String,
    pub chapter_time_end: String,
    #[serde(rename = "ChapterSegmentUID")]
    pub chapter_segment_uid: Option<String>,
    pub chapter_display: ChapterDisplay,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ChapterDisplay {
    pub chapter_string: String,
}

pub struct Chapter {
    pub name: String,
    pub start: Time,
    pub end: Time,
    pub ordered: bool,
}

pub fn parse_chapters(content: &str) -> Vec<Chapter> {
    quick_xml::de::from_str::<Chapters>(content)
        .unwrap()
        .edition_entry
        .chapter_atom
        .into_iter()
        .map(|atom| {
            // 00:00:00.000000000

            let format = format_description!("[hour]:[minute]:[second].[subsecond digits:9]");
            let start: Time = Time::parse(&atom.chapter_time_start, format).unwrap();
            let end: Time = Time::parse(&atom.chapter_time_end, format).unwrap();

            Chapter {
                name: atom.chapter_display.chapter_string,
                start,
                end,
                ordered: atom.chapter_segment_uid.is_some(),
            }
        })
        .collect::<Vec<Chapter>>()
}
