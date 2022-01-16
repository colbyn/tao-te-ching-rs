#![allow(unused)]
use std::collections::{HashMap, LinkedList};
use std::str::FromStr;

pub static IS_LOCAL: bool = false;
pub static GITHUB_ROOT: &'static str = "https://colbyn.github.io/tao-te-ching-rs";
pub static CURRENT_ROOT: &'static str = GITHUB_ROOT;
// pub static CURRENT_ROOT: &'static str = "docs";

#[derive(Debug, Default, Clone)]
struct Line(String);

#[derive(Debug, Default, Clone)]
struct Segment(Vec<Line>);

#[derive(Debug, Default, Clone)]
struct Chapter {
    index: usize,
    segments: Vec<Segment>,
}

#[derive(Debug, Clone)]
pub enum Mode {
    Index,
    Page {
        page_name: String
    },
}

#[derive(Debug, Clone, Default)]
pub struct Settings {
    mode: Option<Mode>,
    chapter_ix: usize,
    segment_ix: Option<usize>,
    line_ix: Option<usize>,
}

pub trait ToText {
    fn to_text(&self, settings: Settings) -> String;
}


impl ToText for Line {
    fn to_text(&self, settings: Settings) -> String {
        let chapter_ix = settings.chapter_ix + 1;
        let segment_ix = settings.segment_ix.unwrap() + 1;
        let line_ix = settings.line_ix.unwrap() + 1;
        let local_id = format!("{}-{}-{}", chapter_ix, segment_ix, line_ix);
        let local_id_txt = format!("{}.{}", segment_ix, line_ix);
        let local_path = format!("#{}", local_id);
        let a = format!("<a href=\"#{id}\">{txt}</a>", id=local_id, txt=local_id_txt);
        format!("<p id=\"{id}\" line>{a} {txt}</p>", id=local_id, txt=self.0, a=a)
    }
}
impl ToText for Segment {
    fn to_text(&self, settings: Settings) -> String {
        let contents = self.0
            .clone()
            .into_iter()
            .enumerate()
            .map(|(ix, line)| {
                let mut settings = settings.clone();
                settings.line_ix = Some(ix);
                line.to_text(settings)
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!("<section>{}</section>", contents)
    }
}
impl ToText for Chapter {
    fn to_text(&self, settings: Settings) -> String {
        let contents = self.segments
            .clone()
            .into_iter()
            .enumerate()
            .map(|(ix, segment)| {
                let mut settings = settings.clone();
                settings.segment_ix = Some(ix);
                segment.to_text(settings)
            })
            .collect::<Vec<String>>()
            .join("\n");
        // TODO
        let index_link = format!("{}/index.html", CURRENT_ROOT);
        let page_link = format!("{}/chapters/chapter-{}.html", CURRENT_ROOT, self.index);
        // GO!
        let index_link = format!("<a href=\"{}\">Tao Te Ching</a>", index_link);
        let page_link = format!("<a href=\"{}\">Chapter {}</a>", page_link, self.index);
        let chapter = match settings.mode {
            Some(Mode::Page{..}) => {
                format!("<h1>{} • {}</h1>", index_link, page_link)
            }
            _ => {
                format!("<h1>{}</h1>", page_link)
            }
        };
        format!("<article chapter>{}\n<div content>\n{}\n</div>\n</article>", chapter, contents)
    }
}


fn pack(contents: String) -> String {
    let deps = r#"
        <meta charset="UTF-8">
        <link rel="preconnect" href="https://fonts.googleapis.com">
        <link rel="preconnect" href="https://fonts.gstatic.com">
        <link href="https://fonts.googleapis.com/css2?family=Noto+Serif:ital,wght@0,400;0,700;1,400;1,700&family=Playfair+Display+SC:ital,wght@0,400;0,700;0,900;1,400;1,700;1,900&family=Playfair+Display:ital,wght@0,400;0,500;0,600;0,700;0,800;0,900;1,400;1,500;1,600;1,700;1,800;1,900&family=Source+Sans+Pro:ital,wght@0,200;0,300;0,400;0,600;0,700;0,900;1,200;1,300;1,400;1,600;1,700;1,900&display=swap" rel="stylesheet">
        <link href="https://fonts.googleapis.com/css2?family=Noto+Serif:ital,wght@0,400;0,700;1,400;1,700&family=Playfair+Display+SC:ital,wght@0,400;0,700;0,900;1,400;1,700;1,900&family=Playfair+Display:ital,wght@0,400;0,500;0,600;0,700;0,800;0,900;1,400;1,500;1,600;1,700;1,800;1,900&family=Source+Sans+Pro:ital,wght@0,200;0,300;0,400;0,600;0,700;0,900;1,200;1,300;1,400;1,600;1,700;1,900&display=swap" rel="stylesheet">
    "#;
    let head = format!("<head>{}<link rel=\"stylesheet\" href=\"{}/styling.css\"></head>", deps, CURRENT_ROOT);
    let main = format!("<main>\n{}\n</main>", contents);
    let body = format!("<body>\n{}\n</body>", main);
    format!("<html>\n{}\n{}\n</html>", head, body)
}


fn run() {
    let source = std::fs::read_to_string("source.txt").unwrap();
    let mut chapters = Vec::<Chapter>::new();
    // PARSER
    for line in source.lines() {
        // CASES
        if let Some(number) = line.trim().parse::<u32>().ok() {
            chapters.push(Chapter {
                index: chapters.len() + 1,
                segments: Vec::new(),
            });
            continue;
        }
        if line.is_empty() {
            if let Some(last_chapter) = chapters.last_mut() {
                last_chapter.segments.push(Segment::default());
            }
            continue;
        }
        // DEFAULT
        if chapters.is_empty() {
            chapters.push(Chapter::default());
        }
        if let Some(last_chapter) = chapters.last_mut() {
            if last_chapter.segments.is_empty() {
                last_chapter.segments.push(Segment::default());
            }
            if let Some(last_segment) = last_chapter.segments.last_mut() {
                last_segment.0.push(Line(line.to_owned()));
            }
        }
    }
    // CODE-GEN
    let chapters = chapters
        .into_iter()
        .map(|mut chapter| {
            // NORMALIZE
            chapter.segments = chapter.segments
                .into_iter()
                .filter(|segment| !segment.0.is_empty())
                .collect::<Vec<Segment>>();
            chapter
        })
        .map(|chapter: Chapter| {
            // TO TEXT
            let page_name = format!("Chapter {}", chapter.index);
            let settings = Settings {
                mode: Some(Mode::Page {page_name}),
                chapter_ix: chapter.index,
                ..Settings::default()
            };
            let file_text = pack(chapter.to_text(settings));
            let file_name = format!("docs/chapters/chapter-{}.html", chapter.index);
            std::fs::write(file_name, file_text).unwrap();
            chapter
        })
        .map(|chapter: Chapter| {
            let settings = Settings {
                mode: Some(Mode::Index),
                chapter_ix: chapter.index,
                ..Settings::default()
            };
            chapter.to_text(settings)
        })
        .collect::<Vec<_>>()
        .join("\n");
    let file = pack(chapters);
    std::fs::write("docs/index.html", file).unwrap();
}

fn main() {
    run()
}
