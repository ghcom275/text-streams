mod disallowed_codepoints;

use disallowed_codepoints::DISALLOWED_CODEPOINTS;
use io_ext_adapters::StdReader;
use text_streams::TextReader;

fn to_text(input: &str) -> String {
    use std::io::Read;
    let mut reader = TextReader::new(StdReader::generic(input.as_bytes()));
    let mut s = String::new();
    reader.read_to_string(&mut s).unwrap();
    s
}

#[test]
fn test_text_input_nfc() {
    // TODO: Test that all the following are done:
    // - Convert all CJK Compatibility Ideograph codepoints that have corresponding
    //   [Standardized Variations] into their corresponding standardized variation
    //   sequences.
    // - Apply the [Stream-Safe Text Process (UAX15-D4)].
    // - Apply `toNFC` according to the [Normalization Process for Stabilized Strings].
    //
    // TODO: Test that svar is done before NFC
    // TODO: Test that stream-safe is done before NFC
}

#[test]
fn test_text_input_rules() {
    // If the stream starts with U+FEFF (BOM), it is removed.
    assert_eq!(to_text("\u{feff}"), "");
    assert_eq!(to_text("\u{feff}\n"), "\n");
    assert_eq!(to_text("\u{feff}hello"), "hello\n");

    // Replace U+000D U+000A with U+000A (newline).
    assert_eq!(to_text("\r"), "\u{fffd}\n");
    assert_eq!(to_text("\rhello\n"), "\u{fffd}hello\n");
    assert_eq!(to_text("\r\n"), "\n");
    assert_eq!(to_text("\n\r"), "\n\u{fffd}\n");
    assert_eq!(to_text("hello\r\nworld"), "hello\nworld\n");

    // *Disallowed codepoints* with U+FFFD (REPLACEMENT CHARACTER)
    for c in &DISALLOWED_CODEPOINTS {
        assert_eq!(
            to_text(&c.to_string()),
            "\u{fffd}\n",
            "disallowed codepoint {:?} was not replaced",
            c
        );
    }

    // Replace U+2329 with U+FFFD (before NFC).
    assert_eq!(to_text("\u{2329}"), "\u{fffd}\n");

    // Replace U+232a with U+FFFD (before NFC).
    assert_eq!(to_text("\u{232a}"), "\u{fffd}\n");

    // These happen as part of NFC.
    assert_eq!(to_text("\u{2126}"), "\u{3a9}\n");
    assert_eq!(to_text("\u{212a}"), "\u{4b}\n");
    assert_eq!(to_text("\u{212b}"), "\u{c5}\n");

    // Replace U+FEFF (BOM) with U+2060 (WJ).
    assert_eq!(to_text("hello\u{feff}world"), "hello\u{2060}world\n");
    assert_eq!(to_text("hello\u{feff}"), "hello\u{2060}\n");
    assert_eq!(to_text("hello\u{feff}\n"), "hello\u{2060}\n");

    // Replace U+0007 (BEL) with U+FFFD (REPLACEMENT CHARACTER).
    assert_eq!(to_text("\u{7}"), "\u{fffd}\n");
    assert_eq!(to_text("\u{7}\n"), "\u{fffd}\n");
    assert_eq!(to_text("hello\u{7}world"), "hello\u{fffd}world\n");

    // Replace U+000C (FF) with U+0020 (SP).
    assert_eq!(to_text("\u{c}"), " \n");
    assert_eq!(to_text("\u{c}\n"), " \n");
    assert_eq!(to_text("\n\u{c}\n"), "\n \n");
    assert_eq!(to_text("hello\u{c}world"), "hello world\n");

    // Replace U+001B (ESC) as part of an *escape sequence* with nothing.
    assert_eq!(to_text("\u{1b}["), "\n");
    assert_eq!(to_text("\u{1b}[A"), "\n");
    assert_eq!(to_text("\u{1b}[AB"), "B\n");
    assert_eq!(to_text("\u{1b}[+"), "\n");
    assert_eq!(to_text("\u{1b}[+A"), "\n");
    assert_eq!(to_text("\u{1b}[+AB"), "B\n");
    assert_eq!(to_text("\u{1b}[++"), "\n");
    assert_eq!(to_text("\u{1b}[++A"), "\n");
    assert_eq!(to_text("\u{1b}[++AB"), "B\n");
    assert_eq!(to_text("\u{1b}]\u{7}"), "\n");
    assert_eq!(to_text("\u{1b}]A\u{7}"), "\n");
    assert_eq!(to_text("\u{1b}]A\n\tB၌\u{7}"), "\n");
    assert_eq!(to_text("\u{1b}]\u{18}"), "\n");
    assert_eq!(to_text("\u{1b}]A\u{18}"), "\n");
    assert_eq!(to_text("\u{1b}]A\n\tB၌\u{18}"), "\n");
    assert_eq!(to_text("\u{1b}A"), "\n");
    assert_eq!(to_text("\u{1b}A\n"), "\n");
    assert_eq!(to_text("\u{1b}\t"), "\u{fffd}\t\n");
    assert_eq!(to_text("\u{1b}\n"), "\u{fffd}\n");
    assert_eq!(to_text("\u{1b}[["), "\n");
    assert_eq!(to_text("\u{1b}[[A"), "\n");
    assert_eq!(to_text("\u{1b}[[\0"), "\n");
    assert_eq!(to_text("\u{1b}[[\u{7f}"), "\n");
    assert_eq!(to_text("\u{1b}[[\n"), "\n");
    assert_eq!(to_text("\u{1b}[[A\n"), "\n");

    // Replace U+001B (ESC) otherwise with U+FFFD (REPLACEMENT CHARACTER).
    assert_eq!(to_text("\u{1b}"), "\u{fffd}\n");
    assert_eq!(to_text("\u{1b}\n"), "\u{fffd}\n");

    // Replace U+0149 with U+02BC U+006E.
    assert_eq!(to_text("\u{149}"), "\u{2bc}\u{6e}\n");

    // Replace U+0673 with U+0627 U+065F.
    assert_eq!(to_text("\u{673}"), "\u{627}\u{65f}\n");

    // Replace U+0F77 with U+0FB2 U+0F81.
    assert_eq!(to_text("\u{f77}"), "\u{fb2}\u{f81}\n");

    // Replace U+0F79 with U+0FB3 U+0F81.
    assert_eq!(to_text("\u{f79}"), "\u{fb3}\u{f81}\n");

    // Replace U+17A3 with U+17A2.
    assert_eq!(to_text("\u{17a3}"), "\u{17a2}\n");

    // Replace U+17A4 with U+17A2 U+17B6.
    assert_eq!(to_text("\u{17a4}"), "\u{17a2}\u{17b6}\n");

    // At the end of the stream, if any codepoints were transmitted and the last
    // codepoint is not U+000A, after replacements, a U+000A is appended.
    assert_eq!(to_text(""), "");
    assert_eq!(to_text("\n"), "\n");
    assert_eq!(to_text("hello"), "hello\n");
    assert_eq!(to_text("hello\nworld"), "hello\nworld\n");
    assert_eq!(to_text("hello\nworld\n"), "hello\nworld\n");
    assert_eq!(to_text("hello\r\nworld"), "hello\nworld\n");
    assert_eq!(to_text("hello\r\nworld\r\n"), "hello\nworld\n");
}
