use rsubs_lib::{SSAEvent, SSA};
use std::cell::Cell;
use std::error::Error;
use std::fs;
use subtitles_from_linked_chapters::{
    get_time_related_subs, read_chapters_file, read_subtitles_file, update_real_times,
    update_subtitle_times, ResultChapter,
};

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let args = args::get_args();

    let linked_name_to_file: Vec<_> = args
        .names_to_ssa_pathes
        .iter()
        .map(|arg| {
            let subs = read_subtitles_file(&arg.path).unwrap();

            (arg.name.to_owned(), subs)
        })
        .collect();

    let mut sub_main_file = read_subtitles_file(args.subtitles_path.as_ref())?;

    let chapters: Vec<_> = read_chapters_file(args.chapters_path.as_ref())?;

    {
        let not_found: Vec<_> = chapters
            .iter()
            .filter(|chp| {
                if !chp.ordered {
                    return false;
                }

                linked_name_to_file
                    .iter()
                    .find(|s| s.0 == chp.name)
                    .is_none()
            })
            .collect();

        if not_found.len() > 0 {
            let not_found = not_found
                .iter()
                .map(|nf| nf.name.to_owned())
                .collect::<Vec<String>>()
                .join(", ");

            let text = format!("Не найдены ордеред субтитры: {}", not_found);

            return Err(text.into());
        }
    }

    let mut chapters = chapters
        .into_iter()
        .map(|chp| {
            let ssa: &SSA;
            if chp.ordered {
                ssa = linked_name_to_file
                    .iter()
                    .find(|s| s.0 == chp.name)
                    .map(|a| &a.1)
                    .unwrap();
            } else {
                ssa = &sub_main_file;
            }

            ResultChapter {
                entries: get_time_related_subs(&ssa.events, &chp.start, &chp.end),
                result_start: Cell::new(chp.start.clone()),
                result_end: Cell::new(chp.end.clone()),
                chapter: chp,
            }
        })
        .collect();

    update_real_times(&chapters);

    update_subtitle_times(&mut chapters);

    let mut comments: Vec<SSAEvent> = Vec::new();
    let mut rest: Vec<SSAEvent> = Vec::new();

    chapters
        .into_iter()
        .flat_map(|chp| chp.entries)
        .for_each(|entry| {
            if entry.event_type == "Comment" {
                comments.push(entry);
            } else {
                rest.push(entry);
            }
        });

    let result_events: Vec<SSAEvent> = comments.into_iter().chain(rest.into_iter()).collect();

    sub_main_file.events = result_events;

    let res = sub_main_file.to_string();

    fs::write(args.result_path, &res)?;

    println!(":)");

    Ok(())
}
