use std::io::Write;
use std::path::{Path, PathBuf};

use comrak::nodes::NodeValue;
use comrak::{format_html, parse_document, Arena, ComrakOptions};
use names::Generator as NameGenerator;

pub struct SlideBuilder {
    buf: Vec<u8>,
}

impl SlideBuilder {
    pub fn new() -> Self {
        Self {
            buf: include_bytes!("start.html.partial").as_slice().to_owned(),
        }
    }

    pub fn push_title(&mut self, title: &str) {
        self.buf.extend(b"<section><h2>");
        self.buf.extend(title.as_bytes());
        self.buf.extend(b"</h2></section>");
    }

    pub fn add_slide(&mut self) {
        self.buf.extend(b"<section>");
    }

    pub fn finish_slide(&mut self) {
        self.buf.extend(b"</section>");
    }

    pub fn borrow_slide(&mut self) -> &mut dyn Write {
        &mut self.buf
    }

    pub fn finish_all(&mut self) {
        self.buf.extend(include_bytes!("end.html.partial"));
    }
}

pub struct Generator {
    path: PathBuf,
}

impl Generator {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().into(),
        }
    }

    pub fn update(&mut self, subject: &str, md: &str) -> anyhow::Result<String> {
        let arena = Arena::new();
        let root = parse_document(&arena, md, &ComrakOptions::default());

        let ops = ComrakOptions::default();
        let mut slide = SlideBuilder::new();
        let mut in_slide = false;

        slide.push_title(subject);

        for top_level in root.children() {
            match &mut top_level.data.borrow_mut().value {
                NodeValue::Heading(heading) if heading.level == 1 => {
                    if in_slide {
                        slide.finish_slide();
                    }

                    slide.add_slide();
                    in_slide = true;

                    heading.level = 3;
                }
                _ => {}
            }

            format_html(top_level, &ops, slide.borrow_slide())?;
        }

        slide.finish_slide();
        slide.finish_all();

        let mut gen = NameGenerator::default();
        let name = gen.next().unwrap() + ".html";
        std::fs::write(self.path.join(&name), &slide.buf)?;

        Ok(name)
    }
}
