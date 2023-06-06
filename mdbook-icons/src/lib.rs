use std::iter::Once;

use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use pulldown_cmark::{Event, Options, Parser};

/// The icon preprocessor.
///
/// Changes patterns of the form `:foo:` into links to images named `foo.png`.
/// Images must be located in an `img/icons` subfolder of the root folder.
#[derive(Default)]
pub struct IconPreprocessor;

impl IconPreprocessor {
    pub fn new() -> Self {
        IconPreprocessor {}
    }
}

impl Preprocessor for IconPreprocessor {
    fn name(&self) -> &str {
        "mdbook-icons"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                process_chapter(chapter)
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

/// Change occurences of `:foo:` in a chapter to links to images.
fn process_chapter(chapter: &mut Chapter) {
    let parser = Parser::new_ext(
        &chapter.content,
        Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS | Options::ENABLE_TABLES,
    );
    let substitued = parser.into_iter().flat_map(|event| match event {
        Event::Text(text) => EventIterator::Cursor(ReplacerCursor::new(text)),
        x => EventIterator::Once(std::iter::once(x)),
    });
    let mut new_contents = String::new();
    let state = pulldown_cmark_to_cmark::cmark(substitued, &mut new_contents).unwrap();
    state.finalize(&mut new_contents).unwrap();
    chapter.content = new_contents;
}

enum EventIterator<'a> {
    Once(Once<Event<'a>>),
    Cursor(ReplacerCursor),
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EventIterator::Once(once) => once.next(),
            EventIterator::Cursor(cursor) => cursor.next(),
        }
    }
}

/// Fed with a string, this is an iterator that turns the string into a series of events.
struct ReplacerCursor {
    /// The string we iterate over.
    s: String,
    /// Index of the next character to check.
    idx: usize,
}

impl ReplacerCursor {
    /// Create a new `ReplacerCursor` from a string.
    fn new<T: ToString>(s: T) -> Self {
        Self {
            s: s.to_string(),
            idx: 0,
        }
    }

    /// Advance cursor to the end and return a single token for the remaining buffer.
    fn text_event_to_end(&mut self) -> Event<'static> {
        let old_idx = self.idx;
        self.idx = self.s.len();
        Event::Text(self.s[old_idx..].to_string().into())
    }

    /// Advance cursor n characters and return a single token for the remaining buffer.
    fn text_event_n(&mut self, n: usize) -> Event<'static> {
        let old_idx = self.idx;
        self.idx += n;
        Event::Text(self.s[old_idx..self.idx].to_string().into())
    }
}

impl Iterator for ReplacerCursor {
    type Item = Event<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.s.len() {
            return None;
        }

        match self.s[self.idx..].find(':') {
            Some(0) => {
                if let Some(end) = self.s[self.idx + 1..].find(':') {
                    // If we found "::".
                    if end == 0 {
                        self.idx += 1;
                        return Some(Event::Text(":".to_string().into()));
                    }

                    // Otherwise, check if the identifier is valid.
                    let id = &self.s[self.idx + 1..self.idx + 1 + end]; // Exclude ':'s.
                    if id.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        // Identifier is valid, output as HTML.
                        self.idx += end + 2;
                        Some(Event::Html(
                            format!(r#"<img class="mdbook-icon" src="/img/icons/{id}.png"/>"#)
                                .into(),
                        ))
                    } else {
                        // Otherwise, print as text.
                        Some(self.text_event_n(end))
                    }
                } else {
                    // Just a single colon. Paste to the end.
                    Some(self.text_event_to_end())
                }
            }
            Some(n) => Some(self.text_event_n(n)),
            None => Some(self.text_event_to_end()),
        }
    }
}
