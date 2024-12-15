use crate::chapters::{parse_chapters, Chapter};
use rsubs_lib::{SSAEvent, SSA};
use std::cell::Cell;
use std::error::Error;
use std::fs;
use std::path::Path;
use time::Time;

mod chapters;

pub struct ResultChapter {
    pub chapter: Chapter,
    pub entries: Vec<SSAEvent>,
    /// Время чаптера после добавления времени всех отрезков и прочего
    pub result_start: Cell<Time>,
    /// Время чаптера после добавления времени всех отрезков и прочего
    pub result_end: Cell<Time>,
}

pub fn read_chapters_file(path: &Path) -> Result<Vec<Chapter>, Box<dyn Error>> {
    let content = fs::read(path)?;

    let (content, _, _) = encoding_rs::UTF_8.decode(&content);

    let chapters = parse_chapters(&content);

    Ok(chapters)
}

pub fn read_subtitles_file(path: &Path) -> Result<SSA, Box<dyn Error>> {
    let content = fs::read(path)?;

    let (content, _, _) = encoding_rs::UTF_8.decode(&content);

    let file = rsubs_lib::SSA::parse(&content)?;

    Ok(file)
}

pub fn get_time_related_subs(
    entries: &Vec<SSAEvent>,
    start: &Time,
    end: &Time,
) -> Vec<SSAEvent> {
    entries
        .iter()
        .filter(|entry| &entry.start >= start && &entry.start < end)
        .map(|entry| entry.clone())
        .collect::<Vec<SSAEvent>>()
}

pub fn update_real_times(chapters: &Vec<ResultChapter>) {
    for (index, chp) in chapters.iter().enumerate() {
        // если чаптер отрезанный, нужно добавить его длительность всем чаптерам после него
        // а самому отрезанному чаптеру добавить конечное время предыдущего чаптера

        if !chp.chapter.ordered {
            continue;
        }

        let delta = chp.result_end.get() - chp.result_start.get();
        for next_chapter in chapters.iter().skip(index + 1) {
            // Время будущих отрезаных чаптеров всё равно зависит от предыдущих чаптеров
            if next_chapter.chapter.ordered {
                continue;
            }

            let s = next_chapter.result_start.get() + delta;
            let e = next_chapter.result_end.get() + delta;

            next_chapter.result_start.replace(s);
            next_chapter.result_end.replace(e);
        }

        if index == 0 {
            continue;
        }

        let prev_chapter = chapters.iter().skip(index - 1).next().unwrap();

        let s = prev_chapter.result_end.get();
        let e = prev_chapter.result_end.get() + delta;

        chp.result_start.replace(s);
        chp.result_end.replace(e);
    }
}

pub fn update_subtitle_times(chapters: &mut Vec<ResultChapter>) {
    for chp in chapters.iter_mut() {
        if chp.result_start.get() == chp.chapter.start {
            continue;
        }

        let delta = chp.result_start.get() - chp.chapter.start;

        for entry in chp.entries.iter_mut() {
            entry.start = entry.start + delta;
            entry.end = entry.end + delta;
        }
    }
}
