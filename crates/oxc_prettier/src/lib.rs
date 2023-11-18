//! Prettier
//!
//! A port of <https://github.com/prettier/prettier>

mod comment;
mod doc;
mod format;
mod macros;
mod options;
mod printer;

use std::{iter::Peekable, vec};

use doc::Doc;
use oxc_allocator::Allocator;
use oxc_ast::{ast::Program, CommentKind, Trivias};

pub use crate::options::{ArrowParens, EndOfLine, PrettierOptions, QuoteProps, TrailingComma};
use crate::{format::Format, printer::Printer};

pub struct Prettier<'a> {
    allocator: &'a Allocator,

    source_text: &'a str,

    options: PrettierOptions,

    /// A stack of comments that will be carefully placed in the right places.
    trivias: Peekable<vec::IntoIter<(u32, u32, CommentKind)>>,
}

impl<'a> Prettier<'a> {
    pub fn new(
        allocator: &'a Allocator,
        source_text: &'a str,
        trivias: Trivias,
        options: PrettierOptions,
    ) -> Self {
        let trivias = trivias.into_iter().peekable();
        Self { allocator, source_text, options, trivias }
    }

    pub fn build(mut self, program: &Program<'a>) -> String {
        let doc = program.format(&mut self);
        Printer::new(doc, self.source_text, self.options).build()
    }

    pub fn doc(mut self, program: &Program<'a>) -> Doc<'a> {
        program.format(&mut self)
    }

    pub(crate) fn should_print_es5_comma(&self) -> bool {
        self.should_print_comma_impl(false)
    }

    #[allow(unused)]
    pub(crate) fn should_print_all_comma(&self) -> bool {
        self.should_print_comma_impl(true)
    }

    pub(crate) fn should_print_comma_impl(&self, level_all: bool) -> bool {
        let trailing_comma = self.options.trailing_comma;
        trailing_comma.is_all() || (trailing_comma.is_es5() && !level_all)
    }

    pub(crate) fn is_next_line_empty(&self, end: u32) -> bool {
        self.source_text[end as usize..].chars().nth(1).is_some_and(|c| c == '\n')
    }
}