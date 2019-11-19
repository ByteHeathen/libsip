use std::fmt;

/// Sip Protocol languages.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Language {
    Abkhazian,
    Afar,
    Akan,
    Afrikaans,
    Albanian,
    Amharic,
    Arabic,
    Aragonese,
    Armenian,
    Assamese,
    Avaric,
    Avestan,
    Aymara,
    Azerbaijani,
    Bambara,
    Bashkir,
    Basque,
    Belarusian,
    Bengali,
    Bihari,
    Bislama,
    Bosnian,
    Breton,
    Bulgarian,
    Burmese,
    Catalan,
    Chamorro,
    Chechen,
    Chichewa,
    Chinese,
    Chuvash,
    Cornish,
    Corsican,
    Cree,
    Croatian,
    Czech,
    Danish,
    Divehi,
    Dutch,
    Dzongkha,
    Esperanto,
    Estonian,
    Ewe,
    Faroese,
    Fijian,
    Finnish,
    French,
    English,
    Fulah,
    Galician,
    Georgian,
    German,
    Greek,
    Guarani,
    Gujarati,
    Haitian,
    Hausa,
    Hebrew,
    Herero,
    Hindi,
    HiriMotu,
    Hungarian,
    Interlingua,
    Indonesian,
    Interlingue,
    Irish,
    Igbo,
    Inupiaq,
    Ido,
    Icelandic,
    Italian,
    Inuktitut,
    Japanese,
    Javanese,
    Kalaallisut,
    Kannada,
    Kanuri,
    Kashmiri,
    Kazakh,
    Khmer,
    Kikuyu,
    Kinyarwanda,
    Kirghiz,
    Komi,
    Kongo,
    Korean,
    Kurdish,
    Kuanyama,
    Latin,
    Luxembourgish,
    Ganda,
    Limburgan,
    Lingala,
    Lao,
    Lithuanian,
    LubaKatanga,
    Latvian,
    Manx,
    Macedonian,
    Malagasy,
    Malay,
    Malayalam,
    Maltese,
    Maori,
    Marathi,
    Marshallese,
    Mongolian,
    Nepali,
    Nauru,
    Navajo,
    NorthNdebele,
    Ndonga,
    NorwegianBokmal,
    NorwegianNynorsk,
    Norwegian,
    SichuanYi,
    SouthNdebele,
    Occitan,
    Ojibwa,
    ChurchSlavic,
    Oromo,
    Oriya,
    Ossetian,
    Punjabi,
    Pali,
    Persian,
    Polish,
    Pashto,
    Portuguese,
    Quechua,
    Romansh,
    Rundi,
    Romanian,
    Russian,
    Sanskrit,
    Sardinian,
    Sindhi,
    NorthernSami,
    Samoan,
    Sango,
    Serbian,
    Gaelic,
    Shona,
    Sinhala,
    Slovak,
    Slovenian,
    Somali,
    SouthernSotho,
    Spanish,
    Sundanese,
    Swahili,
    Swati,
    Swedish,
    Tamil,
    Telugu,
    Tajik,
    Thai,
    Tigrinya,
    Tibetan,
    Turkmen,
    Tagalog,
    Tswana,
    Tonga,
    Turkish,
    Tsonga,
    Tatar,
    Twi,
    Tahitian,
    Uighur,
    Ukrainian,
    Urdu,
    Uzbek,
    Venda,
    Vietnamese,
    Volapuk,
    Walloon,
    Welsh,
    Wolof,
    WesternFrisian,
    Xhosa,
    Yiddish,
    Yoruba,
    Zhuang,
    Zulu,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::Abkhazian => write!(f, "ab"),
            Language::Afar => write!(f, "aa"),
            Language::Afrikaans => write!(f, "af"),
            Language::Akan => write!(f, "ak"),
            Language::Albanian => write!(f, "sq"),
            Language::Amharic => write!(f, "am"),
            Language::Arabic => write!(f, "ar"),
            Language::Aragonese => write!(f, "an"),
            Language::Armenian => write!(f, "hy"),
            Language::Assamese => write!(f, "as"),
            Language::Avaric => write!(f, "av"),
            Language::Avestan => write!(f, "ae"),
            Language::Aymara => write!(f, "ay"),
            Language::Azerbaijani => write!(f, "az"),
            Language::Bambara => write!(f, "bm"),
            Language::Bashkir => write!(f, "ba"),
            Language::Basque => write!(f, "eu"),
            Language::Belarusian => write!(f, "be"),
            Language::Bengali => write!(f, "bn"),
            Language::Bihari => write!(f, "bh"),
            Language::Bislama => write!(f, "bi"),
            Language::Bosnian => write!(f, "bs"),
            Language::Breton => write!(f, "br"),
            Language::Bulgarian => write!(f, "bg"),
            Language::Burmese => write!(f, "my"),
            Language::Catalan => write!(f, "ca"),
            Language::Chamorro => write!(f, "ch"),
            Language::Chechen => write!(f, "ce"),
            Language::Chichewa => write!(f, "ny"),
            Language::Chinese => write!(f, "zh"),
            Language::Chuvash => write!(f, "cv"),
            Language::Cornish => write!(f, "kw"),
            Language::Corsican => write!(f, "co"),
            Language::Cree => write!(f, "cr"),
            Language::Croatian => write!(f, "hr"),
            Language::Czech => write!(f, "cs"),
            Language::Danish => write!(f, "da"),
            Language::Divehi => write!(f, "dv"),
            Language::Dutch => write!(f, "nl"),
            Language::Dzongkha => write!(f, "dz"),
            Language::English => write!(f, "en"),
            Language::Esperanto => write!(f, "eo"),
            Language::Estonian => write!(f, "et"),
            Language::Ewe => write!(f, "ee"),
            Language::Faroese => write!(f, "fo"),
            Language::Fijian => write!(f, "fj"),
            Language::Finnish => write!(f, "fi"),
            Language::French => write!(f, "fr"),
            Language::Fulah => write!(f, "ff"),
            Language::Galician => write!(f, "gl"),
            Language::Georgian => write!(f, "ka"),
            Language::German => write!(f, "de"),
            Language::Greek => write!(f, "el"),
            Language::Guarani => write!(f, "gn"),
            Language::Gujarati => write!(f, "gu"),
            Language::Haitian => write!(f, "ht"),
            Language::Hausa => write!(f, "ha"),
            Language::Hebrew => write!(f, "he"),
            Language::Herero => write!(f, "hz"),
            Language::Hindi => write!(f, "hi"),
            Language::HiriMotu => write!(f, "ho"),
            Language::Hungarian => write!(f, "hu"),
            Language::Interlingua => write!(f, "ia"),
            Language::Indonesian => write!(f, "id"),
            Language::Interlingue => write!(f, "ie"),
            Language::Irish => write!(f, "ga"),
            Language::Igbo => write!(f, "ig"),
            Language::Inupiaq => write!(f, "ik"),
            Language::Ido => write!(f, "io"),
            Language::Icelandic => write!(f, "is"),
            Language::Italian => write!(f, "it"),
            Language::Inuktitut => write!(f, "iu"),
            Language::Japanese => write!(f, "ja"),
            Language::Javanese => write!(f, "jv"),
            Language::Kalaallisut => write!(f, "kl"),
            Language::Kannada => write!(f, "kn"),
            Language::Kanuri => write!(f, "kr"),
            Language::Kashmiri => write!(f, "ks"),
            Language::Kazakh => write!(f, "kk"),
            Language::Khmer => write!(f, "km"),
            Language::Kikuyu => write!(f, "ki"),
            Language::Kinyarwanda => write!(f, "rw"),
            Language::Kirghiz => write!(f, "ky"),
            Language::Komi => write!(f, "kv"),
            Language::Kongo => write!(f, "kg"),
            Language::Korean => write!(f, "ko"),
            Language::Kurdish => write!(f, "ku"),
            Language::Kuanyama => write!(f, "kj"),
            Language::Latin => write!(f, "la"),
            Language::Luxembourgish => write!(f, "lb"),
            Language::Ganda => write!(f, "lg"),
            Language::Limburgan => write!(f, "li"),
            Language::Lingala => write!(f, "ln"),
            Language::Lao => write!(f, "lo"),
            Language::Lithuanian => write!(f, "lt"),
            Language::LubaKatanga => write!(f, "lu"),
            Language::Latvian => write!(f, "lv"),
            Language::Manx => write!(f, "gv"),
            Language::Macedonian => write!(f, "mk"),
            Language::Malagasy => write!(f, "mg"),
            Language::Malay => write!(f, "ms"),
            Language::Malayalam => write!(f, "ml"),
            Language::Maltese => write!(f, "mt"),
            Language::Maori => write!(f, "mi"),
            Language::Marathi => write!(f, "mr"),
            Language::Marshallese => write!(f, "mh"),
            Language::Mongolian => write!(f, "mn"),
            Language::Nepali => write!(f, "ne"),
            Language::Nauru => write!(f, "na"),
            Language::Navajo => write!(f, "nv"),
            Language::NorthNdebele => write!(f, "nd"),
            Language::Ndonga => write!(f, "ng"),
            Language::NorwegianBokmal => write!(f, "nb"),
            Language::NorwegianNynorsk => write!(f, "nn"),
            Language::Norwegian => write!(f, "no"),
            Language::SichuanYi => write!(f, "ii"),
            Language::SouthNdebele => write!(f, "nr"),
            Language::Occitan => write!(f, "oc"),
            Language::Ojibwa => write!(f, "oj"),
            Language::ChurchSlavic => write!(f, "cu"),
            Language::Oromo => write!(f, "om"),
            Language::Oriya => write!(f, "or"),
            Language::Ossetian => write!(f, "os"),
            Language::Punjabi => write!(f, "pa"),
            Language::Pali => write!(f, "pi"),
            Language::Persian => write!(f, "fa"),
            Language::Polish => write!(f, "pi"),
            Language::Pashto => write!(f, "ps"),
            Language::Portuguese => write!(f, "pt"),
            Language::Quechua => write!(f, "qu"),
            Language::Romansh => write!(f, "rm"),
            Language::Rundi => write!(f, "rn"),
            Language::Romanian => write!(f, "ro"),
            Language::Russian => write!(f, "ru"),
            Language::Sanskrit => write!(f, "sa"),
            Language::Sardinian => write!(f, "sc"),
            Language::Sindhi => write!(f, "sd"),
            Language::NorthernSami => write!(f, "se"),
            Language::Samoan => write!(f, "sm"),
            Language::Sango => write!(f, "sg"),
            Language::Serbian => write!(f, "sr"),
            Language::Gaelic => write!(f, "gd"),
            Language::Shona => write!(f, "sn"),
            Language::Sinhala => write!(f, "si"),
            Language::Slovak => write!(f, "sk"),
            Language::Slovenian => write!(f, "sl"),
            Language::Somali => write!(f, "so"),
            Language::SouthernSotho => write!(f, "st"),
            Language::Spanish => write!(f, "es"),
            Language::Sundanese => write!(f, "su"),
            Language::Swahili => write!(f, "sw"),
            Language::Swati => write!(f, "ss"),
            Language::Swedish => write!(f, "sv"),
            Language::Tamil => write!(f, "ta"),
            Language::Telugu => write!(f, "te"),
            Language::Tajik => write!(f, "tg"),
            Language::Thai => write!(f, "th"),
            Language::Tigrinya => write!(f, "ti"),
            Language::Tibetan => write!(f, "bo"),
            Language::Turkmen => write!(f, "tk"),
            Language::Tagalog => write!(f, "tl"),
            Language::Tswana => write!(f, "tn"),
            Language::Tonga => write!(f, "to"),
            Language::Turkish => write!(f, "tr"),
            Language::Tsonga => write!(f, "ts"),
            Language::Tatar => write!(f, "tt"),
            Language::Twi => write!(f, "tw"),
            Language::Tahitian => write!(f, "ty"),
            Language::Uighur => write!(f, "ug"),
            Language::Ukrainian => write!(f, "uk"),
            Language::Urdu => write!(f, "ur"),
            Language::Uzbek => write!(f, "uz"),
            Language::Venda => write!(f, "ve"),
            Language::Vietnamese => write!(f, "vi"),
            Language::Volapuk => write!(f, "vo"),
            Language::Walloon => write!(f, "wa"),
            Language::Welsh => write!(f, "cy"),
            Language::Wolof => write!(f, "wo"),
            Language::WesternFrisian => write!(f, "fy"),
            Language::Xhosa => write!(f, "xh"),
            Language::Yiddish => write!(f, "yi"),
            Language::Yoruba => write!(f, "yo"),
            Language::Zhuang => write!(f, "za"),
            Language::Zulu => write!(f, "zu"),
        }
    }
}

named!(pub parse_language<Language>, alt!(
    map!(tag_no_case!("ab"), |_| Language::Abkhazian) |
    map!(tag_no_case!("aa"), |_| Language::Afar) |
    map!(tag_no_case!("af"), |_| Language::Afrikaans) |
    map!(tag_no_case!("ak"), |_| Language::Akan) |
    map!(tag_no_case!("sq"), |_| Language::Albanian) |
    map!(tag_no_case!("ar"), |_| Language::Arabic) |
    map!(tag_no_case!("an"), |_| Language::Aragonese) |
    map!(tag_no_case!("hy"), |_| Language::Armenian) |
    map!(tag_no_case!("as"), |_| Language::Assamese) |
    map!(tag_no_case!("av"), |_| Language::Avaric) |
    map!(tag_no_case!("ae"), |_| Language::Avestan) |
    map!(tag_no_case!("az"), |_| Language::Azerbaijani) |
    map!(tag_no_case!("bm"), |_| Language::Bambara) |
    map!(tag_no_case!("ba"), |_| Language::Bashkir) |
    map!(tag_no_case!("eu"), |_| Language::Basque) |
    map!(tag_no_case!("be"), |_| Language::Belarusian) |
    map!(tag_no_case!("bn"), |_| Language::Bengali) |
    map!(tag_no_case!("bh"), |_| Language::Bihari) |
    map!(tag_no_case!("bi"), |_| Language::Bislama) |
    map!(tag_no_case!("bs"), |_| Language::Bosnian) |
    map!(tag_no_case!("br"), |_| Language::Breton) |
    map!(tag_no_case!("bg"), |_| Language::Bulgarian) |
    map!(tag_no_case!("my"), |_| Language::Burmese) |
    map!(tag_no_case!("ca"), |_| Language::Catalan) |
    map!(tag_no_case!("ch"), |_| Language::Chamorro) |
    map!(tag_no_case!("ce"), |_| Language::Chechen) |
    map!(tag_no_case!("ny"), |_| Language::Chichewa) |
    map!(tag_no_case!("zh"), |_| Language::Chinese) |
    map!(tag_no_case!("cv"), |_| Language::Chuvash) |
    map!(tag_no_case!("kw"), |_| Language::Cornish) |
    map!(tag_no_case!("co"), |_| Language::Corsican) |
    map!(tag_no_case!("cr"), |_| Language::Cree) |
    map!(tag_no_case!("hr"), |_| Language::Croatian) |
    map!(tag_no_case!("cs"), |_| Language::Czech) |
    map!(tag_no_case!("da"), |_| Language::Danish) |
    map!(tag_no_case!("dv"), |_| Language::Divehi) |
    map!(tag_no_case!("nl"), |_| Language::Dutch) |
    map!(tag_no_case!("dz"), |_| Language::Dzongkha) |
    map!(tag_no_case!("en"), |_| Language::English) |
    map!(tag_no_case!("eo"), |_| Language::Esperanto) |
    map!(tag_no_case!("et"), |_| Language::Estonian) |
    map!(tag_no_case!("ee"), |_| Language::Ewe) |
    map!(tag_no_case!("fo"), |_| Language::Faroese) |
    map!(tag_no_case!("fj"), |_| Language::Fijian) |
    map!(tag_no_case!("fi"), |_| Language::Finnish) |
    map!(tag_no_case!("fr"), |_| Language::French) |
    map!(tag_no_case!("ff"), |_| Language::Fulah) |
    map!(tag_no_case!("gl"), |_| Language::Galician) |
    map!(tag_no_case!("ka"), |_| Language::Georgian) |
    map!(tag_no_case!("de"), |_| Language::German) |
    map!(tag_no_case!("el"), |_| Language::Greek) |
    map!(tag_no_case!("gn"), |_| Language::Guarani) |
    map!(tag_no_case!("gu"), |_| Language::Gujarati) |
    map!(tag_no_case!("ht"), |_| Language::Haitian) |
    map!(tag_no_case!("ha"), |_| Language::Hausa) |
    map!(tag_no_case!("he"), |_| Language::Hebrew) |
    map!(tag_no_case!("hz"), |_| Language::Herero) |
    map!(tag_no_case!("hi"), |_| Language::Hindi) |
    map!(tag_no_case!("ho"), |_| Language::HiriMotu) |
    map!(tag_no_case!("hu"), |_| Language::Hungarian) |
    map!(tag_no_case!("ia"), |_| Language::Interlingua) |
    map!(tag_no_case!("id"), |_| Language::	Indonesian) |
    map!(tag_no_case!("ie"), |_| Language::Interlingue) |
    map!(tag_no_case!("ga"), |_| Language::Irish) |
    map!(tag_no_case!("ig"), |_| Language::Igbo) |
    map!(tag_no_case!("ik"), |_| Language::Inupiaq) |
    map!(tag_no_case!("io"), |_| Language::Ido) |
    map!(tag_no_case!("is"), |_| Language::Icelandic) |
    map!(tag_no_case!("it"), |_| Language::Italian) |
    map!(tag_no_case!("iu"), |_| Language::Inuktitut) |
    map!(tag_no_case!("ja"), |_| Language::Japanese) |
    map!(tag_no_case!("jv"), |_| Language::Javanese) |
    map!(tag_no_case!("kl"), |_| Language::Kalaallisut) |
    map!(tag_no_case!("kn"), |_| Language::Kannada) |
    map!(tag_no_case!("kr"), |_| Language::Kanuri) |
    map!(tag_no_case!("ks"), |_| Language::Kashmiri) |
    map!(tag_no_case!("kk"), |_| Language::Kazakh) |
    map!(tag_no_case!("km"), |_| Language::Khmer) |
    map!(tag_no_case!("ki"), |_| Language::Kikuyu) |
    map!(tag_no_case!("rw"), |_| Language::Kinyarwanda) |
    map!(tag_no_case!("ky"), |_| Language::Kirghiz) |
    map!(tag_no_case!("kv"), |_| Language::Komi) |
    map!(tag_no_case!("kg"), |_| Language::Kongo) |
    map!(tag_no_case!("ko"), |_| Language::Korean) |
    map!(tag_no_case!("ku"), |_| Language::Kurdish) |
    map!(tag_no_case!("kj"), |_| Language::Kuanyama) |
    map!(tag_no_case!("la"), |_| Language::Latin) |
    map!(tag_no_case!("lb"), |_| Language::Luxembourgish) |
    map!(tag_no_case!("lg"), |_| Language::Ganda) |
    map!(tag_no_case!("li"), |_| Language::Limburgan) |
    map!(tag_no_case!("ln"), |_| Language::Lingala) |
    map!(tag_no_case!("lo"), |_| Language::Lao) |
    map!(tag_no_case!("lt"), |_| Language::Lithuanian) |
    map!(tag_no_case!("lu"), |_| Language::LubaKatanga) |
    map!(tag_no_case!("lv"), |_| Language::Latvian) |
    map!(tag_no_case!("gv"), |_| Language::Manx) |
    map!(tag_no_case!("mk"), |_| Language::Macedonian) |
    map!(tag_no_case!("mg"), |_| Language::Malagasy) |
    map!(tag_no_case!("ms"), |_| Language::Malay) |
    map!(tag_no_case!("ml"), |_| Language::Malayalam) |
    map!(tag_no_case!("mt"), |_| Language::Maltese) |
    map!(tag_no_case!("mi"), |_| Language::Maori) |
    map!(tag_no_case!("mr"), |_| Language::Marathi) |
    map!(tag_no_case!("mh"), |_| Language::Marshallese) |
    map!(tag_no_case!("mn"), |_| Language::Mongolian) |
    map!(tag_no_case!("na"), |_| Language::Nauru) |
    map!(tag_no_case!("nv"), |_| Language::Navajo) |
    map!(tag_no_case!("nd"), |_| Language::NorthNdebele) |
    map!(tag_no_case!("ne"), |_| Language::Nepali) |
    map!(tag_no_case!("ng"), |_| Language::Ndonga) |
    map!(tag_no_case!("nb"), |_| Language::NorwegianBokmal) |
    map!(tag_no_case!("nn"), |_| Language::NorwegianNynorsk) |
    map!(tag_no_case!("no"), |_| Language::Norwegian) |
    map!(tag_no_case!("ii"), |_| Language::SichuanYi) |
    map!(tag_no_case!("nr"), |_| Language::SouthNdebele) |
    map!(tag_no_case!("oc"), |_| Language::Occitan) |
    map!(tag_no_case!("oj"), |_| Language::Ojibwa) |
    map!(tag_no_case!("cu"), |_| Language::ChurchSlavic) |
    map!(tag_no_case!("om"), |_| Language::Oromo) |
    map!(tag_no_case!("or"), |_| Language::Oriya) |
    map!(tag_no_case!("os"), |_| Language::Ossetian) |
    map!(tag_no_case!("pa"), |_| Language::Punjabi) |
    map!(tag_no_case!("pi"), |_| Language::Pali) |
    map!(tag_no_case!("fa"), |_| Language::Persian) |
    map!(tag_no_case!("pi"), |_| Language::Polish) |
    map!(tag_no_case!("ps"), |_| Language::Pashto) |
    map!(tag_no_case!("pt"), |_| Language::Portuguese) |
    map!(tag_no_case!("qu"), |_| Language::Quechua) |
    map!(tag_no_case!("rm"), |_| Language::Romansh) |
    map!(tag_no_case!("rn"), |_| Language::Rundi) |
    map!(tag_no_case!("ro"), |_| Language::Romanian) |
    map!(tag_no_case!("ru"), |_| Language::Russian) |
    map!(tag_no_case!("sa"), |_| Language::Sanskrit) |
    map!(tag_no_case!("sc"), |_| Language::Sardinian) |
    map!(tag_no_case!("sd"), |_| Language::Sindhi) |
    map!(tag_no_case!("se"), |_| Language::NorthernSami) |
    map!(tag_no_case!("sm"), |_| Language::Samoan) |
    map!(tag_no_case!("sg"), |_| Language::Sango) |
    map!(tag_no_case!("sr"), |_| Language::Serbian) |
    map!(tag_no_case!("gd"), |_| Language::Gaelic) |
    map!(tag_no_case!("sn"), |_| Language::Shona) |
    map!(tag_no_case!("si"), |_| Language::Sinhala) |
    map!(tag_no_case!("sk"), |_| Language::Slovak) |
    map!(tag_no_case!("sl"), |_| Language::Slovenian) |
    map!(tag_no_case!("so"), |_| Language::Somali) |
    map!(tag_no_case!("st"), |_| Language::SouthernSotho) |
    map!(tag_no_case!("es"), |_| Language::Spanish) |
    map!(tag_no_case!("su"), |_| Language::Sundanese) |
    map!(tag_no_case!("sw"), |_| Language::Swahili) |
    map!(tag_no_case!("ss"), |_| Language::Swati) |
    map!(tag_no_case!("sv"), |_| Language::Swedish) |
    map!(tag_no_case!("ta"), |_| Language::Tamil) |
    map!(tag_no_case!("te"), |_| Language::Telugu) |
    map!(tag_no_case!("tg"), |_| Language::Tajik) |
    map!(tag_no_case!("th"), |_| Language::Thai) |
    map!(tag_no_case!("ti"), |_| Language::Tigrinya) |
    map!(tag_no_case!("bo"), |_| Language::Tibetan) |
    map!(tag_no_case!("tk"), |_| Language::Turkmen) |
    map!(tag_no_case!("tl"), |_| Language::Tagalog) |
    map!(tag_no_case!("tn"), |_| Language::Tswana) |
    map!(tag_no_case!("to"), |_| Language::Tonga) |
    map!(tag_no_case!("tr"), |_| Language::Turkish) |
    map!(tag_no_case!("ts"), |_| Language::Tsonga) |
    map!(tag_no_case!("tt"), |_| Language::Tatar) |
    map!(tag_no_case!("tw"), |_| Language::Twi) |
    map!(tag_no_case!("ty"), |_| Language::Tahitian) |
    map!(tag_no_case!("ug"), |_| Language::Uighur) |
    map!(tag_no_case!("uk"), |_| Language::Ukrainian) |
    map!(tag_no_case!("ur"), |_| Language::Urdu) |
    map!(tag_no_case!("uz"), |_| Language::Uzbek) |
    map!(tag_no_case!("ve"), |_| Language::Venda) |
    map!(tag_no_case!("vi"), |_| Language::Vietnamese) |
    map!(tag_no_case!("vo"), |_| Language::Volapuk) |
    map!(tag_no_case!("wa"), |_| Language::Walloon) |
    map!(tag_no_case!("cy"), |_| Language::Welsh) |
    map!(tag_no_case!("wo"), |_| Language::Wolof) |
    map!(tag_no_case!("fy"), |_| Language::WesternFrisian) |
    map!(tag_no_case!("xh"), |_| Language::Xhosa) |
    map!(tag_no_case!("yi"), |_| Language::Yiddish) |
    map!(tag_no_case!("yo"), |_| Language::Yoruba) |
    map!(tag_no_case!("za"), |_| Language::Zhuang) |
    map!(tag_no_case!("zu"), |_| Language::Zulu)
));
