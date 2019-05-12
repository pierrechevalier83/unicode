/// Unicode characters, grouped by enums which represent one unicode block each.
/// Each variant of these enums uniquely identifies once unicode character.

/// \u{0} → \u{7f}
pub mod basic_latin;

/// \u{80} → \u{ff}
pub mod latin_1_supplement;

/// \u{100} → \u{17f}
pub mod latin_extended_a;

/// \u{180} → \u{24f}
pub mod latin_extended_b;

/// \u{250} → \u{2af}
pub mod ipa_extensions;

/// \u{2b0} → \u{2ff}
pub mod spacing_modifier_letters;

/// \u{300} → \u{36f}
pub mod combining_diacritical_marks;

/// \u{370} → \u{3ff}
pub mod greek_and_coptic;

/// \u{400} → \u{4ff}
pub mod cyrillic;

/// \u{500} → \u{52f}
pub mod cyrillic_supplement;

/// \u{530} → \u{58f}
pub mod armenian;

/// \u{590} → \u{5ff}
pub mod hebrew;

/// \u{600} → \u{6ff}
pub mod arabic;

/// \u{700} → \u{74f}
pub mod syriac;

/// \u{750} → \u{77f}
pub mod arabic_supplement;

/// \u{780} → \u{7bf}
pub mod thaana;

/// \u{7c0} → \u{7ff}
pub mod nko;

/// \u{800} → \u{83f}
pub mod samaritan;

/// \u{840} → \u{85f}
pub mod mandaic;

/// \u{860} → \u{86f}
pub mod syriac_supplement;

/// \u{8a0} → \u{8ff}
pub mod arabic_extended_a;

/// \u{900} → \u{97f}
pub mod devanagari;

/// \u{980} → \u{9ff}
pub mod bengali;

/// \u{a00} → \u{a7f}
pub mod gurmukhi;

/// \u{a80} → \u{aff}
pub mod gujarati;

/// \u{b00} → \u{b7f}
pub mod oriya;

/// \u{b80} → \u{bff}
pub mod tamil;

/// \u{c00} → \u{c7f}
pub mod telugu;

/// \u{c80} → \u{cff}
pub mod kannada;

/// \u{d00} → \u{d7f}
pub mod malayalam;

/// \u{d80} → \u{dff}
pub mod sinhala;

/// \u{e00} → \u{e7f}
pub mod thai;

/// \u{e80} → \u{eff}
pub mod lao;

/// \u{f00} → \u{fff}
pub mod tibetan;

/// \u{1000} → \u{109f}
pub mod myanmar;

/// \u{10a0} → \u{10ff}
pub mod georgian;

/// \u{1100} → \u{11ff}
pub mod hangul_jamo;

/// \u{1200} → \u{137f}
pub mod ethiopic;

/// \u{1380} → \u{139f}
pub mod ethiopic_supplement;

/// \u{13a0} → \u{13ff}
pub mod cherokee;

/// \u{1400} → \u{167f}
pub mod unified_canadian_aboriginal_syllabics;

/// \u{1680} → \u{169f}
pub mod ogham;

/// \u{16a0} → \u{16ff}
pub mod runic;

/// \u{1700} → \u{171f}
pub mod tagalog;

/// \u{1720} → \u{173f}
pub mod hanunoo;

/// \u{1740} → \u{175f}
pub mod buhid;

/// \u{1760} → \u{177f}
pub mod tagbanwa;

/// \u{1780} → \u{17ff}
pub mod khmer;

/// \u{1800} → \u{18af}
pub mod mongolian;

/// \u{18b0} → \u{18ff}
pub mod unified_canadian_aboriginal_syllabics_extended;

/// \u{1900} → \u{194f}
pub mod limbu;

/// \u{1950} → \u{197f}
pub mod tai_le;

/// \u{1980} → \u{19df}
pub mod new_tai_lue;

/// \u{19e0} → \u{19ff}
pub mod khmer_symbols;

/// \u{1a00} → \u{1a1f}
pub mod buginese;

/// \u{1a20} → \u{1aaf}
pub mod tai_tham;

/// \u{1ab0} → \u{1aff}
pub mod combining_diacritical_marks_extended;

/// \u{1b00} → \u{1b7f}
pub mod balinese;

/// \u{1b80} → \u{1bbf}
pub mod sundanese;

/// \u{1bc0} → \u{1bff}
pub mod batak;

/// \u{1c00} → \u{1c4f}
pub mod lepcha;

/// \u{1c50} → \u{1c7f}
pub mod ol_chiki;

/// \u{1c80} → \u{1c8f}
pub mod cyrillic_extended_c;

/// \u{1c90} → \u{1cbf}
pub mod georgian_extended;

/// \u{1cc0} → \u{1ccf}
pub mod sundanese_supplement;

/// \u{1cd0} → \u{1cff}
pub mod vedic_extensions;

/// \u{1d00} → \u{1d7f}
pub mod phonetic_extensions;

/// \u{1d80} → \u{1dbf}
pub mod phonetic_extensions_supplement;

/// \u{1dc0} → \u{1dff}
pub mod combining_diacritical_marks_supplement;

/// \u{1e00} → \u{1eff}
pub mod latin_extended_additional;

/// \u{1f00} → \u{1fff}
pub mod greek_extended;

/// \u{2000} → \u{206f}
pub mod general_punctuation;

/// \u{2070} → \u{209f}
pub mod superscripts_and_subscripts;

/// \u{20a0} → \u{20cf}
pub mod currency_symbols;

/// \u{20d0} → \u{20ff}
pub mod combining_diacritical_marks_for_symbols;

/// \u{2100} → \u{214f}
pub mod letterlike_symbols;

/// \u{2150} → \u{218f}
pub mod number_forms;

/// \u{2190} → \u{21ff}
pub mod arrows;

/// \u{2200} → \u{22ff}
pub mod mathematical_operators;

/// \u{2300} → \u{23ff}
pub mod miscellaneous_technical;

/// \u{2400} → \u{243f}
pub mod control_pictures;

/// \u{2440} → \u{245f}
pub mod optical_character_recognition;

/// \u{2460} → \u{24ff}
pub mod enclosed_alphanumerics;

/// \u{2500} → \u{257f}
pub mod box_drawing;

/// \u{2580} → \u{259f}
pub mod block_elements;

/// \u{25a0} → \u{25ff}
pub mod geometric_shapes;

/// \u{2600} → \u{26ff}
pub mod miscellaneous_symbols;

/// \u{2700} → \u{27bf}
pub mod dingbats;

/// \u{27c0} → \u{27ef}
pub mod miscellaneous_mathematical_symbols_a;

/// \u{27f0} → \u{27ff}
pub mod supplemental_arrows_a;

/// \u{2800} → \u{28ff}
pub mod braille_patterns;

/// \u{2900} → \u{297f}
pub mod supplemental_arrows_b;

/// \u{2980} → \u{29ff}
pub mod miscellaneous_mathematical_symbols_b;

/// \u{2a00} → \u{2aff}
pub mod supplemental_mathematical_operators;

/// \u{2b00} → \u{2bff}
pub mod miscellaneous_symbols_and_arrows;

/// \u{2c00} → \u{2c5f}
pub mod glagolitic;

/// \u{2c60} → \u{2c7f}
pub mod latin_extended_c;

/// \u{2c80} → \u{2cff}
pub mod coptic;

/// \u{2d00} → \u{2d2f}
pub mod georgian_supplement;

/// \u{2d30} → \u{2d7f}
pub mod tifinagh;

/// \u{2d80} → \u{2ddf}
pub mod ethiopic_extended;

/// \u{2de0} → \u{2dff}
pub mod cyrillic_extended_a;

/// \u{2e00} → \u{2e7f}
pub mod supplemental_punctuation;

/// \u{2e80} → \u{2eff}
pub mod cjk_radicals_supplement;

/// \u{2f00} → \u{2fdf}
pub mod kangxi_radicals;

/// \u{2ff0} → \u{2fff}
pub mod ideographic_description_characters;

/// \u{3000} → \u{303f}
pub mod cjk_symbols_and_punctuation;

/// \u{3040} → \u{309f}
pub mod hiragana;

/// \u{30a0} → \u{30ff}
pub mod katakana;

/// \u{3100} → \u{312f}
pub mod bopomofo;

/// \u{3130} → \u{318f}
pub mod hangul_compatibility_jamo;

/// \u{3190} → \u{319f}
pub mod kanbun;

/// \u{31a0} → \u{31bf}
pub mod bopomofo_extended;

/// \u{31c0} → \u{31ef}
pub mod cjk_strokes;

/// \u{31f0} → \u{31ff}
pub mod katakana_phonetic_extensions;

/// \u{3200} → \u{32ff}
pub mod enclosed_cjk_letters_and_months;

/// \u{3300} → \u{33ff}
pub mod cjk_compatibility;

/// \u{3400} → \u{4dbf}
pub mod cjk_unified_ideographs_extension_a;

/// \u{4dc0} → \u{4dff}
pub mod yijing_hexagram_symbols;

/// \u{4e00} → \u{9fff}
pub mod cjk_unified_ideographs;

/// \u{a000} → \u{a48f}
pub mod yi_syllables;

/// \u{a490} → \u{a4cf}
pub mod yi_radicals;

/// \u{a4d0} → \u{a4ff}
pub mod lisu;

/// \u{a500} → \u{a63f}
pub mod vai;

/// \u{a640} → \u{a69f}
pub mod cyrillic_extended_b;

/// \u{a6a0} → \u{a6ff}
pub mod bamum;

/// \u{a700} → \u{a71f}
pub mod modifier_tone_letters;

/// \u{a720} → \u{a7ff}
pub mod latin_extended_d;

/// \u{a800} → \u{a82f}
pub mod syloti_nagri;

/// \u{a830} → \u{a83f}
pub mod common_indic_number_forms;

/// \u{a840} → \u{a87f}
pub mod phags_pa;

/// \u{a880} → \u{a8df}
pub mod saurashtra;

/// \u{a8e0} → \u{a8ff}
pub mod devanagari_extended;

/// \u{a900} → \u{a92f}
pub mod kayah_li;

/// \u{a930} → \u{a95f}
pub mod rejang;

/// \u{a960} → \u{a97f}
pub mod hangul_jamo_extended_a;

/// \u{a980} → \u{a9df}
pub mod javanese;

/// \u{a9e0} → \u{a9ff}
pub mod myanmar_extended_b;

/// \u{aa00} → \u{aa5f}
pub mod cham;

/// \u{aa60} → \u{aa7f}
pub mod myanmar_extended_a;

/// \u{aa80} → \u{aadf}
pub mod tai_viet;

/// \u{aae0} → \u{aaff}
pub mod meetei_mayek_extensions;

/// \u{ab00} → \u{ab2f}
pub mod ethiopic_extended_a;

/// \u{ab30} → \u{ab6f}
pub mod latin_extended_e;

/// \u{ab70} → \u{abbf}
pub mod cherokee_supplement;

/// \u{abc0} → \u{abff}
pub mod meetei_mayek;

/// \u{ac00} → \u{d7af}
pub mod hangul_syllables;

/// \u{d7b0} → \u{d7ff}
pub mod hangul_jamo_extended_b;

/// \u{e000} → \u{f8ff}
pub mod private_use_area;

/// \u{f900} → \u{faff}
pub mod cjk_compatibility_ideographs;

/// \u{fb00} → \u{fb4f}
pub mod alphabetic_presentation_forms;

/// \u{fb50} → \u{fdff}
pub mod arabic_presentation_forms_a;

/// \u{fe00} → \u{fe0f}
pub mod variation_selectors;

/// \u{fe10} → \u{fe1f}
pub mod vertical_forms;

/// \u{fe20} → \u{fe2f}
pub mod combining_half_marks;

/// \u{fe30} → \u{fe4f}
pub mod cjk_compatibility_forms;

/// \u{fe50} → \u{fe6f}
pub mod small_form_variants;

/// \u{fe70} → \u{feff}
pub mod arabic_presentation_forms_b;

/// \u{ff00} → \u{ffef}
pub mod halfwidth_and_fullwidth_forms;

/// \u{fff0} → \u{ffff}
pub mod specials;

/// \u{10000} → \u{1007f}
pub mod linear_b_syllabary;

/// \u{10080} → \u{100ff}
pub mod linear_b_ideograms;

/// \u{10100} → \u{1013f}
pub mod aegean_numbers;

/// \u{10140} → \u{1018f}
pub mod ancient_greek_numbers;

/// \u{10190} → \u{101cf}
pub mod ancient_symbols;

/// \u{101d0} → \u{101ff}
pub mod phaistos_disc;

/// \u{10280} → \u{1029f}
pub mod lycian;

/// \u{102a0} → \u{102df}
pub mod carian;

/// \u{102e0} → \u{102ff}
pub mod coptic_epact_numbers;

/// \u{10300} → \u{1032f}
pub mod old_italic;

/// \u{10330} → \u{1034f}
pub mod gothic;

/// \u{10350} → \u{1037f}
pub mod old_permic;

/// \u{10380} → \u{1039f}
pub mod ugaritic;

/// \u{103a0} → \u{103df}
pub mod old_persian;

/// \u{10400} → \u{1044f}
pub mod deseret;

/// \u{10450} → \u{1047f}
pub mod shavian;

/// \u{10480} → \u{104af}
pub mod osmanya;

/// \u{104b0} → \u{104ff}
pub mod osage;

/// \u{10500} → \u{1052f}
pub mod elbasan;

/// \u{10530} → \u{1056f}
pub mod caucasian_albanian;

/// \u{10600} → \u{1077f}
pub mod linear_a;

/// \u{10800} → \u{1083f}
pub mod cypriot_syllabary;

/// \u{10840} → \u{1085f}
pub mod imperial_aramaic;

/// \u{10860} → \u{1087f}
pub mod palmyrene;

/// \u{10880} → \u{108af}
pub mod nabataean;

/// \u{108e0} → \u{108ff}
pub mod hatran;

/// \u{10900} → \u{1091f}
pub mod phoenician;

/// \u{10920} → \u{1093f}
pub mod lydian;

/// \u{10980} → \u{1099f}
pub mod meroitic_hieroglyphs;

/// \u{109a0} → \u{109ff}
pub mod meroitic_cursive;

/// \u{10a00} → \u{10a5f}
pub mod kharoshthi;

/// \u{10a60} → \u{10a7f}
pub mod old_south_arabian;

/// \u{10a80} → \u{10a9f}
pub mod old_north_arabian;

/// \u{10ac0} → \u{10aff}
pub mod manichaean;

/// \u{10b00} → \u{10b3f}
pub mod avestan;

/// \u{10b40} → \u{10b5f}
pub mod inscriptional_parthian;

/// \u{10b60} → \u{10b7f}
pub mod inscriptional_pahlavi;

/// \u{10b80} → \u{10baf}
pub mod psalter_pahlavi;

/// \u{10c00} → \u{10c4f}
pub mod old_turkic;

/// \u{10c80} → \u{10cff}
pub mod old_hungarian;

/// \u{10d00} → \u{10d3f}
pub mod hanifi_rohingya;

/// \u{10e60} → \u{10e7f}
pub mod rumi_numeral_symbols;

/// \u{10f00} → \u{10f2f}
pub mod old_sogdian;

/// \u{10f30} → \u{10f6f}
pub mod sogdian;

/// \u{10fe0} → \u{10fff}
pub mod elymaic;

/// \u{11000} → \u{1107f}
pub mod brahmi;

/// \u{11080} → \u{110cf}
pub mod kaithi;

/// \u{110d0} → \u{110ff}
pub mod sora_sompeng;

/// \u{11100} → \u{1114f}
pub mod chakma;

/// \u{11150} → \u{1117f}
pub mod mahajani;

/// \u{11180} → \u{111df}
pub mod sharada;

/// \u{111e0} → \u{111ff}
pub mod sinhala_archaic_numbers;

/// \u{11200} → \u{1124f}
pub mod khojki;

/// \u{11280} → \u{112af}
pub mod multani;

/// \u{112b0} → \u{112ff}
pub mod khudawadi;

/// \u{11300} → \u{1137f}
pub mod grantha;

/// \u{11400} → \u{1147f}
pub mod newa;

/// \u{11480} → \u{114df}
pub mod tirhuta;

/// \u{11580} → \u{115ff}
pub mod siddham;

/// \u{11600} → \u{1165f}
pub mod modi;

/// \u{11660} → \u{1167f}
pub mod mongolian_supplement;

/// \u{11680} → \u{116cf}
pub mod takri;

/// \u{11700} → \u{1173f}
pub mod ahom;

/// \u{11800} → \u{1184f}
pub mod dogra;

/// \u{118a0} → \u{118ff}
pub mod warang_citi;

/// \u{119a0} → \u{119ff}
pub mod nandinagari;

/// \u{11a00} → \u{11a4f}
pub mod zanabazar_square;

/// \u{11a50} → \u{11aaf}
pub mod soyombo;

/// \u{11ac0} → \u{11aff}
pub mod pau_cin_hau;

/// \u{11c00} → \u{11c6f}
pub mod bhaiksuki;

/// \u{11c70} → \u{11cbf}
pub mod marchen;

/// \u{11d00} → \u{11d5f}
pub mod masaram_gondi;

/// \u{11d60} → \u{11daf}
pub mod gunjala_gondi;

/// \u{11ee0} → \u{11eff}
pub mod makasar;

/// \u{11fc0} → \u{11fff}
pub mod tamil_supplement;

/// \u{12000} → \u{123ff}
pub mod cuneiform;

/// \u{12400} → \u{1247f}
pub mod cuneiform_numbers_and_punctuation;

/// \u{12480} → \u{1254f}
pub mod early_dynastic_cuneiform;

/// \u{13000} → \u{1342f}
pub mod egyptian_hieroglyphs;

/// \u{13430} → \u{1343f}
pub mod egyptian_hieroglyph_format_controls;

/// \u{14400} → \u{1467f}
pub mod anatolian_hieroglyphs;

/// \u{16800} → \u{16a3f}
pub mod bamum_supplement;

/// \u{16a40} → \u{16a6f}
pub mod mro;

/// \u{16ad0} → \u{16aff}
pub mod bassa_vah;

/// \u{16b00} → \u{16b8f}
pub mod pahawh_hmong;

/// \u{16e40} → \u{16e9f}
pub mod medefaidrin;

/// \u{16f00} → \u{16f9f}
pub mod miao;

/// \u{16fe0} → \u{16fff}
pub mod ideographic_symbols_and_punctuation;

/// \u{17000} → \u{187ff}
pub mod tangut;

/// \u{18800} → \u{18aff}
pub mod tangut_components;

/// \u{1b000} → \u{1b0ff}
pub mod kana_supplement;

/// \u{1b100} → \u{1b12f}
pub mod kana_extended_a;

/// \u{1b130} → \u{1b16f}
pub mod small_kana_extension;

/// \u{1b170} → \u{1b2ff}
pub mod nushu;

/// \u{1bc00} → \u{1bc9f}
pub mod duployan;

/// \u{1bca0} → \u{1bcaf}
pub mod shorthand_format_controls;

/// \u{1d000} → \u{1d0ff}
pub mod byzantine_musical_symbols;

/// \u{1d100} → \u{1d1ff}
pub mod musical_symbols;

/// \u{1d200} → \u{1d24f}
pub mod ancient_greek_musical_notation;

/// \u{1d2e0} → \u{1d2ff}
pub mod mayan_numerals;

/// \u{1d300} → \u{1d35f}
pub mod tai_xuan_jing_symbols;

/// \u{1d360} → \u{1d37f}
pub mod counting_rod_numerals;

/// \u{1d400} → \u{1d7ff}
pub mod mathematical_alphanumeric_symbols;

/// \u{1d800} → \u{1daaf}
pub mod sutton_signwriting;

/// \u{1e000} → \u{1e02f}
pub mod glagolitic_supplement;

/// \u{1e100} → \u{1e14f}
pub mod nyiakeng_puachue_hmong;

/// \u{1e2c0} → \u{1e2ff}
pub mod wancho;

/// \u{1e800} → \u{1e8df}
pub mod mende_kikakui;

/// \u{1e900} → \u{1e95f}
pub mod adlam;

/// \u{1ec70} → \u{1ecbf}
pub mod indic_siyaq_numbers;

/// \u{1ed00} → \u{1ed4f}
pub mod ottoman_siyaq_numbers;

/// \u{1ee00} → \u{1eeff}
pub mod arabic_mathematical_alphabetic_symbols;

/// \u{1f000} → \u{1f02f}
pub mod mahjong_tiles;

/// \u{1f030} → \u{1f09f}
pub mod domino_tiles;

/// \u{1f0a0} → \u{1f0ff}
pub mod playing_cards;

/// \u{1f100} → \u{1f1ff}
pub mod enclosed_alphanumeric_supplement;

/// \u{1f200} → \u{1f2ff}
pub mod enclosed_ideographic_supplement;

/// \u{1f300} → \u{1f5ff}
pub mod miscellaneous_symbols_and_pictographs;

/// \u{1f600} → \u{1f64f}
pub mod emoticons;

/// \u{1f650} → \u{1f67f}
pub mod ornamental_dingbats;

/// \u{1f680} → \u{1f6ff}
pub mod transport_and_map_symbols;

/// \u{1f700} → \u{1f77f}
pub mod alchemical_symbols;

/// \u{1f780} → \u{1f7ff}
pub mod geometric_shapes_extended;

/// \u{1f800} → \u{1f8ff}
pub mod supplemental_arrows_c;

/// \u{1f900} → \u{1f9ff}
pub mod supplemental_symbols_and_pictographs;

/// \u{1fa00} → \u{1fa6f}
pub mod chess_symbols;

/// \u{1fa70} → \u{1faff}
pub mod symbols_and_pictographs_extended_a;

/// \u{20000} → \u{2a6df}
pub mod cjk_unified_ideographs_extension_b;

/// \u{2a700} → \u{2b73f}
pub mod cjk_unified_ideographs_extension_c;

/// \u{2b740} → \u{2b81f}
pub mod cjk_unified_ideographs_extension_d;

/// \u{2b820} → \u{2ceaf}
pub mod cjk_unified_ideographs_extension_e;

/// \u{2ceb0} → \u{2ebef}
pub mod cjk_unified_ideographs_extension_f;

/// \u{2f800} → \u{2fa1f}
pub mod cjk_compatibility_ideographs_supplement;

/// \u{e0000} → \u{e007f}
pub mod tags;

/// \u{e0100} → \u{e01ef}
pub mod variation_selectors_supplement;

/// \u{f0000} → \u{fffff}
pub mod supplementary_private_use_area_a;

/// \u{100000} → \u{10ffff}
pub mod supplementary_private_use_area_b;

