# Text

The *text* format is built on top of the [UTF-8] format. It is intended for
general-purpose use most anywhere the informal notion of "plain text" is
intended. It permits homoglyphs and other visual ambiguities; see
[Restricted Text] for an alternative which provides some mitigations.

On input and output, data is implicitly converted into [NFC] by the
following steps, in order:
 - Replace all CJK Compatibility Ideograph codepoints that have corresponding
   [Standardized Variations] with their corresponding standardized variation
   sequences.
 - Replace U+2329 with U+FFFD (REPLACEMENT CHARACTER).
 - Replace U+232A with U+FFFD (REPLACEMENT CHARACTER).
 - Apply the [Stream-Safe Text Process (UAX15-D4)].
 - Apply `toNFC` according to the [Normalization Process for Stabilized Strings].

On input, after conversion to NFC:
 - If the stream starts with U+FEFF (BOM), it is removed.
 - Replace:
   - U+000D U+000A with U+000A (newline)
   - *Disallowed codepoints* with U+FFFD (REPLACEMENT CHARACTER)
   - U+0007 (BEL) with U+FFFD (REPLACEMENT CHARACTER)
   - U+000C (FF) with U+0020 (SP)
   - U+001B (ESC) as part of an *escape sequence* with nothing
   - U+001B (ESC) otherwise with U+FFFD (REPLACEMENT CHARACTER)
   - U+0149 with U+02BC U+006E
   - U+0673 with U+0627 U+065F
   - U+0F77 with U+0FB2 U+0F81
   - U+0F79 with U+0FB3 U+0F81
   - U+17A3 with U+17A2
   - U+17A4 with U+17A2 U+17B6
   - U+FEFF (BOM) with U+2060 (WJ)
 - At the end of the stream, if any codepoints were transmitted and the last
   codepoint is not U+000A, after replacements, a U+000A is appended.

On output, before conversion to NFC:
 - As an option (BOM compatibility), off by default, prepend U+FEFF to the stream.
 - As an option (CRLF compatibility), off by default, replace "\n" with "\r\n".
 - Fail at any of the following:
   - *Disallowed codepoints*
   - U+0007 (BEL)
   - U+000C (FF)
   - U+001B (ESC)
   - U+0149 (LATIN SMALL LETTER N PRECEDED BY APOSTROPHE)
   - U+0673 (ARABIC LETTER ALEF WITH WAVY HAMZA BELOW)
   - U+0F77 (TIBETAN VOWEL SIGN VOCALIC RR)
   - U+0F79 (TIBETAN VOWEL SIGN VOCALIC LL)
   - U+17A3 (KHMER INDEPENDENT VOWEL QAQ)
   - U+17A4 (KHMER INDEPENDENT VOWEL QAA)
   - U+2126 (OHM SIGN)
   - U+212A (KELVIN SIGN)
   - U+212B (ANGSTROM SIGN)
   - U+2329 (LEFT-POINTING ANGLE BRACKET)
   - U+232A (RIGHT-POINTING ANGLE BRACKET)
   - U+FEFF (BOM)
 - At the end of the stream, if any codepoints were transmitted and the last
   codepoint is not U+000A, fail.

The *disallowed codepoints* are:
 - All C0, U+007F, and C1 control codes other than U+000A (newline),
   U+0009 (horizontal tab), U+000C (form feed), U+001B (escape), and
   U+0007 (alert)
 - [Noncharacters]
 - [Deprecated Format Characters]
 - [Private-Use Characters]
 - [Tag Characters]

## Escape Sequences

An *escape sequence* is any of the following sequences, all of which start with
U+001B (ESC). If multiple rules match, the longest match is chosen.

```
U+001B U+005B [U+0020–U+003F]* [U+0040–U+007E]?
U+001B U+005D [^U+0007,U+0018,U+001B]* [U+0007,U+0018]?
U+001B [U+0040–U+007E]?
U+001B U+005B U+005B [U+0000–U+007F]?
```

## TODOs

TODO: Pull in some NFKC translations? https://github.com/rust-lang/rust/issues/2253#issuecomment-29050949

TODO: U+17B4 and U+17B5 "should be considered errors in the encoding"
and "The use of U+17D8 khmer sign beyyal is discouraged" though there
are no replacements.

TODO: Streams never start or resume after a push with a normalization-form
non-starter. `canonical_combining_class` doesn't know about the astral
compositions like U+11099 U+110BA => U+1109A. Restrict non-starters of that
form too? Or a joiner like WJ, CGJ, ZWJ, or ZWNJ?
Or use unicode-segmentation to detect grapheme boundaries?

TODO: NFC isn't closed under concatenation; can we restrict streams to starting with starters, or implicitly insert CGJs?

TODO: Should we say anything about bidi control codepoints? https://unicode.org/reports/tr9/

TODO: Should we say anything about inter-linear annotation codepoints?

TODO: Should we say anything about U+FFFC (object replacement character)?

TODO: Should we say anything about unrecognized and/or IVD variation selectors?

[NFC]: https://unicode.org/reports/tr15/#Norm_Forms
[Stream-Safe Text Process (UAX15-D4)]: https://unicode.org/reports/tr15/#UAX15-D4
[Standardized Variations]: http://unicode.org/faq/vs.html
[Normalization Process for Stabilized Strings]: https://unicode.org/reports/tr15/#Normalization_Process_for_Stabilized_Strings
[Noncharacters]: http://www.unicode.org/faq/private_use.html#noncharacters
[Deprecated Format Characters]: https://www.unicode.org/versions/Unicode13.0.0/ch23.pdf#G19593
[Private-Use Characters]: http://www.unicode.org/faq/private_use.html#private_use
[Tag Characters]: https://www.unicode.org/versions/Unicode13.0.0/ch23.pdf#G30110
[Restricted Text]: restricted-text.md
[UTF-8]: utf-8.md
