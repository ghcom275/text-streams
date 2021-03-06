use unicode_normalization::char::canonical_combining_class;

/// The size of the longest UTF-8 scalar value encoding. Note that even though
/// RFC-2279 allowed longer encodings, it's obsoleted by RFC-3629 which doesn't.
/// This limit is also documented in [the relevant section of Rust's documentation].
///
/// [the relevant section of Rust's documentation]: https://doc.rust-lang.org/stable/std/primitive.char.html#method.encode_utf8
pub(crate) const MAX_UTF8_SIZE: usize = 4;

/// From unicode-normalization.
const MAX_NONSTARTERS: usize = 30;

// Enough for a composed start, a long sequence of nonstarters, followed by a
// composed end.
//
// TODO: Investigate whether we can avoid considering composed starters and stoppers.
pub(crate) const NORMALIZATION_BUFFER_LEN: usize = 2 + MAX_NONSTARTERS + 2;

/// The minimum size of a buffer needed to perform NFC normalization,
/// and thus the minimum size needed to pass to
/// [`TextReader::read`](crate::TextReader::read).
pub const NORMALIZATION_BUFFER_SIZE: usize = MAX_UTF8_SIZE * NORMALIZATION_BUFFER_LEN;

/// ASCII FF, known as '\f' in some contexts.
pub(crate) const FF: char = '\u{c}';

/// ASCII ESC, known as '\e' in some contexts.
pub(crate) const ESC: char = '\u{1b}';

/// ASCII SUB.
pub(crate) const SUB: char = '\u{1a}';

/// ASCII DEL, which is not what's generated by the "delete" key on the keyboard
pub(crate) const DEL: char = '\u{7f}';

/// COMBINING GRAPHEME JOINER
pub(crate) const CGJ: char = '\u{34f}';

/// ZERO WIDTH NO-BREAK SPACE, also known as the byte-order mark, or BOM
pub(crate) const BOM: char = '\u{feff}';

/// WORD JOINER
pub(crate) const WJ: char = '\u{2060}';

/// REPLACEMENT CHARACTER
pub(crate) const REPL: char = '\u{fffd}';

// TODO: include ZWJ, WJ, ZWNJ, CGJ as non-starters?
pub(crate) fn is_normalization_form_starter(c: char) -> bool {
    canonical_combining_class(c) == 0
}
