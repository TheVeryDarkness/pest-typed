//! Wrapped types for unicode property. See [pest::unicode] for details.

use crate::{
    iterators::{Pairs, Token},
    tracker::Tracker,
    Input, RuleType, Span, Stack, TypedNode,
};
use core::fmt;

macro_rules! unicode {
    ($property_ident:ident) => {
        #[allow(non_camel_case_types)]
        #[doc = concat!("Auto generated. Unicode property ", stringify!($property_ident))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Hash, PartialEq, Eq)]
        pub struct $property_ident {
            /// Matched character.
            ///
            /// Do not trust this field as it may be assigned to after creation.
            pub content: char,
        }
        impl From<char> for $property_ident {
            fn from(content: char) -> Self {
                Self { content }
            }
        }
        impl<'i, R: RuleType> TypedNode<'i, R> for $property_ident {
            #[inline]
            fn try_parse_partial_with<I: Input<'i>>(
                mut input: I,
                _stack: &mut Stack<Span<'i>>,
                _tracker: &mut Tracker<'i, R>,
            ) -> Option<(I, Self)> {
                match super::match_char_by(&mut input, pest::unicode::$property_ident) {
                    Some(content) => Some((input, Self::from(content))),
                    None => None,
                }
            }
            #[inline]
            fn try_check_partial_with<I: Input<'i>>(
                mut input: I,
                _stack: &mut Stack<Span<'i>>,
                _tracker: &mut Tracker<'i, R>,
            ) -> Option<I> {
                match super::match_char_by(&mut input, pest::unicode::$property_ident) {
                    Some(_) => Some(input),
                    None => None,
                }
            }
        }
        impl fmt::Debug for $property_ident {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($property_ident))
                    .field("content", &self.content)
                    .finish()
            }
        }
        impl<'i, R: RuleType> Pairs<'i, R> for $property_ident {
            fn for_self_or_each_child(&self, _f: &mut impl FnMut(Token<'i, R>)) {}
        }
    };
}
unicode!(ALPHABETIC);
unicode!(BIDI_CONTROL);
unicode!(CASE_IGNORABLE);
unicode!(CASED);
unicode!(CHANGES_WHEN_CASEFOLDED);
unicode!(CHANGES_WHEN_CASEMAPPED);
unicode!(CHANGES_WHEN_LOWERCASED);
unicode!(CHANGES_WHEN_TITLECASED);
unicode!(CHANGES_WHEN_UPPERCASED);
unicode!(DASH);
unicode!(DEFAULT_IGNORABLE_CODE_POINT);
unicode!(DEPRECATED);
unicode!(DIACRITIC);
unicode!(EMOJI);
unicode!(EMOJI_COMPONENT);
unicode!(EMOJI_MODIFIER);
unicode!(EMOJI_MODIFIER_BASE);
unicode!(EMOJI_PRESENTATION);
unicode!(EXTENDED_PICTOGRAPHIC);
unicode!(EXTENDER);
unicode!(GRAPHEME_BASE);
unicode!(GRAPHEME_EXTEND);
unicode!(GRAPHEME_LINK);
unicode!(HEX_DIGIT);
unicode!(HYPHEN);
unicode!(IDS_BINARY_OPERATOR);
unicode!(IDS_TRINARY_OPERATOR);
unicode!(ID_CONTINUE);
unicode!(ID_START);
unicode!(IDEOGRAPHIC);
unicode!(JOIN_CONTROL);
unicode!(LOGICAL_ORDER_EXCEPTION);
unicode!(LOWERCASE);
unicode!(MATH);
unicode!(NONCHARACTER_CODE_POINT);
unicode!(OTHER_ALPHABETIC);
unicode!(OTHER_DEFAULT_IGNORABLE_CODE_POINT);
unicode!(OTHER_GRAPHEME_EXTEND);
unicode!(OTHER_ID_CONTINUE);
unicode!(OTHER_ID_START);
unicode!(OTHER_LOWERCASE);
unicode!(OTHER_MATH);
unicode!(OTHER_UPPERCASE);
unicode!(PATTERN_SYNTAX);
unicode!(PATTERN_WHITE_SPACE);
unicode!(PREPENDED_CONCATENATION_MARK);
unicode!(QUOTATION_MARK);
unicode!(RADICAL);
unicode!(REGIONAL_INDICATOR);
unicode!(SENTENCE_TERMINAL);
unicode!(SOFT_DOTTED);
unicode!(TERMINAL_PUNCTUATION);
unicode!(UNIFIED_IDEOGRAPH);
unicode!(UPPERCASE);
unicode!(VARIATION_SELECTOR);
unicode!(WHITE_SPACE);
unicode!(XID_CONTINUE);
unicode!(XID_START);
unicode!(CASED_LETTER);
unicode!(CLOSE_PUNCTUATION);
unicode!(CONNECTOR_PUNCTUATION);
unicode!(CONTROL);
unicode!(CURRENCY_SYMBOL);
unicode!(DASH_PUNCTUATION);
unicode!(DECIMAL_NUMBER);
unicode!(ENCLOSING_MARK);
unicode!(FINAL_PUNCTUATION);
unicode!(FORMAT);
unicode!(INITIAL_PUNCTUATION);
unicode!(LETTER);
unicode!(LETTER_NUMBER);
unicode!(LINE_SEPARATOR);
unicode!(LOWERCASE_LETTER);
unicode!(MARK);
unicode!(MATH_SYMBOL);
unicode!(MODIFIER_LETTER);
unicode!(MODIFIER_SYMBOL);
unicode!(NONSPACING_MARK);
unicode!(NUMBER);
unicode!(OPEN_PUNCTUATION);
unicode!(OTHER);
unicode!(OTHER_LETTER);
unicode!(OTHER_NUMBER);
unicode!(OTHER_PUNCTUATION);
unicode!(OTHER_SYMBOL);
unicode!(PARAGRAPH_SEPARATOR);
unicode!(PRIVATE_USE);
unicode!(PUNCTUATION);
unicode!(SEPARATOR);
unicode!(SPACE_SEPARATOR);
unicode!(SPACING_MARK);
unicode!(SURROGATE);
unicode!(SYMBOL);
unicode!(TITLECASE_LETTER);
unicode!(UNASSIGNED);
unicode!(UPPERCASE_LETTER);
unicode!(ADLAM);
unicode!(AHOM);
unicode!(ANATOLIAN_HIEROGLYPHS);
unicode!(ARABIC);
unicode!(ARMENIAN);
unicode!(AVESTAN);
unicode!(BALINESE);
unicode!(BAMUM);
unicode!(BASSA_VAH);
unicode!(BATAK);
unicode!(BENGALI);
unicode!(BHAIKSUKI);
unicode!(BOPOMOFO);
unicode!(BRAHMI);
unicode!(BRAILLE);
unicode!(BUGINESE);
unicode!(BUHID);
unicode!(CANADIAN_ABORIGINAL);
unicode!(CARIAN);
unicode!(CAUCASIAN_ALBANIAN);
unicode!(CHAKMA);
unicode!(CHAM);
unicode!(CHEROKEE);
unicode!(CHORASMIAN);
unicode!(COMMON);
unicode!(COPTIC);
unicode!(CUNEIFORM);
unicode!(CYPRIOT);
unicode!(CYPRO_MINOAN);
unicode!(CYRILLIC);
unicode!(DESERET);
unicode!(DEVANAGARI);
unicode!(DIVES_AKURU);
unicode!(DOGRA);
unicode!(DUPLOYAN);
unicode!(EGYPTIAN_HIEROGLYPHS);
unicode!(ELBASAN);
unicode!(ELYMAIC);
unicode!(ETHIOPIC);
unicode!(GEORGIAN);
unicode!(GLAGOLITIC);
unicode!(GOTHIC);
unicode!(GRANTHA);
unicode!(GREEK);
unicode!(GUJARATI);
unicode!(GUNJALA_GONDI);
unicode!(GURMUKHI);
unicode!(HAN);
unicode!(HANGUL);
unicode!(HANIFI_ROHINGYA);
unicode!(HANUNOO);
unicode!(HATRAN);
unicode!(HEBREW);
unicode!(HIRAGANA);
unicode!(IMPERIAL_ARAMAIC);
unicode!(INHERITED);
unicode!(INSCRIPTIONAL_PAHLAVI);
unicode!(INSCRIPTIONAL_PARTHIAN);
unicode!(JAVANESE);
unicode!(KAITHI);
unicode!(KANNADA);
unicode!(KATAKANA);
unicode!(KAWI);
unicode!(KAYAH_LI);
unicode!(KHAROSHTHI);
unicode!(KHITAN_SMALL_SCRIPT);
unicode!(KHMER);
unicode!(KHOJKI);
unicode!(KHUDAWADI);
unicode!(LAO);
unicode!(LATIN);
unicode!(LEPCHA);
unicode!(LIMBU);
unicode!(LINEAR_A);
unicode!(LINEAR_B);
unicode!(LISU);
unicode!(LYCIAN);
unicode!(LYDIAN);
unicode!(MAHAJANI);
unicode!(MAKASAR);
unicode!(MALAYALAM);
unicode!(MANDAIC);
unicode!(MANICHAEAN);
unicode!(MARCHEN);
unicode!(MASARAM_GONDI);
unicode!(MEDEFAIDRIN);
unicode!(MEETEI_MAYEK);
unicode!(MENDE_KIKAKUI);
unicode!(MEROITIC_CURSIVE);
unicode!(MEROITIC_HIEROGLYPHS);
unicode!(MIAO);
unicode!(MODI);
unicode!(MONGOLIAN);
unicode!(MRO);
unicode!(MULTANI);
unicode!(MYANMAR);
unicode!(NABATAEAN);
unicode!(NAG_MUNDARI);
unicode!(NANDINAGARI);
unicode!(NEW_TAI_LUE);
unicode!(NEWA);
unicode!(NKO);
unicode!(NUSHU);
unicode!(NYIAKENG_PUACHUE_HMONG);
unicode!(OGHAM);
unicode!(OL_CHIKI);
unicode!(OLD_HUNGARIAN);
unicode!(OLD_ITALIC);
unicode!(OLD_NORTH_ARABIAN);
unicode!(OLD_PERMIC);
unicode!(OLD_PERSIAN);
unicode!(OLD_SOGDIAN);
unicode!(OLD_SOUTH_ARABIAN);
unicode!(OLD_TURKIC);
unicode!(OLD_UYGHUR);
unicode!(ORIYA);
unicode!(OSAGE);
unicode!(OSMANYA);
unicode!(PAHAWH_HMONG);
unicode!(PALMYRENE);
unicode!(PAU_CIN_HAU);
unicode!(PHAGS_PA);
unicode!(PHOENICIAN);
unicode!(PSALTER_PAHLAVI);
unicode!(REJANG);
unicode!(RUNIC);
unicode!(SAMARITAN);
unicode!(SAURASHTRA);
unicode!(SHARADA);
unicode!(SHAVIAN);
unicode!(SIDDHAM);
unicode!(SIGNWRITING);
unicode!(SINHALA);
unicode!(SOGDIAN);
unicode!(SORA_SOMPENG);
unicode!(SOYOMBO);
unicode!(SUNDANESE);
unicode!(SYLOTI_NAGRI);
unicode!(SYRIAC);
unicode!(TAGALOG);
unicode!(TAGBANWA);
unicode!(TAI_LE);
unicode!(TAI_THAM);
unicode!(TAI_VIET);
unicode!(TAKRI);
unicode!(TAMIL);
unicode!(TANGSA);
unicode!(TANGUT);
unicode!(TELUGU);
unicode!(THAANA);
unicode!(THAI);
unicode!(TIBETAN);
unicode!(TIFINAGH);
unicode!(TIRHUTA);
unicode!(TOTO);
unicode!(UGARITIC);
unicode!(VAI);
unicode!(VITHKUQI);
unicode!(WANCHO);
unicode!(WARANG_CITI);
unicode!(YEZIDI);
unicode!(YI);
unicode!(ZANABAZAR_SQUARE);
