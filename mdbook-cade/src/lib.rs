use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use pulldown_cmark::{CowStr, Event, Options, Parser};

use crate::anguish::{make_table, AnguishChoices};

mod anguish;

/// The cade preprocessor.
///
/// Looks for specific patterns in the md file and source data from configuration files to recreate
/// the associated elements.
#[derive(Default)]
pub struct CadePreprocessor {
    pub anguish_choices: AnguishChoices,
}

impl CadePreprocessor {
    #[must_use]
    pub fn new() -> Self {
        CadePreprocessor {
            anguish_choices: AnguishChoices::from_yaml("orna_data/anguish-choices.yaml"),
        }
    }
}

impl Preprocessor for CadePreprocessor {
    fn name(&self) -> &'static str {
        "mdbook-cade"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                self.process_chapter(chapter);
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

impl CadePreprocessor {
    fn process_chapter(&self, chapter: &mut Chapter) {
        let parser = Parser::new_ext(
            &chapter.content,
            Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS | Options::ENABLE_TABLES,
        );
        let substitued = parser.into_iter().map(|event| match event {
            Event::Html(text) => Event::Html(self.process_html_event(text)),
            x => x,
        });
        let mut new_contents = String::new();
        let state = pulldown_cmark_to_cmark::cmark(substitued, &mut new_contents).unwrap();
        state.finalize(&mut new_contents).unwrap();
        chapter.content = new_contents;
    }

    fn process_html_event<'a>(&self, source: CowStr<'a>) -> CowStr<'a> {
        match &*source {
            "<table id=\"ang-despair-choices\"></table>\n" => {
                make_table(&self.anguish_choices.despair, "despair").into()
            }
            "<table id=\"ang-melancholy-choices\"></table>\n" => {
                make_table(&self.anguish_choices.melancholy, "melancholy").into()
            }
            "<table id=\"ang-agony-choices\"></table>\n" => {
                make_table(&self.anguish_choices.agony, "agony").into()
            }
            "<table id=\"ang-torment-choices\"></table>\n" => {
                make_table(&self.anguish_choices.torment, "torment").into()
            }
            _ => source,
        }
    }
}
