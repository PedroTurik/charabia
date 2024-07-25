/// Default characters categorized as separators.
///
/// This list contains all the characters in below unicode categories:
/// - Pc Connector Punctuation
/// - Pd Dash Punctuation
/// - Pe Close Punctuation
/// - Pf Final Punctuation
/// - Pi Initial Punctuation
/// - Po Other Punctuation
/// - Ps Open Punctuation
/// - Sm Math Symbol
/// - Zl Line Separator
/// - Zp Paragraph Separator
/// - Zs Space Separator
/// plus "\0", ". ", ", " and ។ល។" (៘ decomposition) to categorize them as hard separators
/// and "`" to understand markdown formatted text
#[rustfmt::skip]
pub const DEFAULT_SEPARATORS: &[&str] = &[
    "\0", ". ", ", ", "_", "‿", "⁀", "⁔", "︳", "︴", "﹍", "﹎", "﹏", "＿", "-", "֊", "־", "᐀", "᠆", "‐", "‒", "–",
    "—", "―", "⸗", "⸚", "⸺", "⸻", "⹀", "〜", "〰", "゠", "︱", "︲", "﹘", "﹣", "－", "𐺭", ")",
    "]", "}", "༻", "༽", "᚜", "⁆", "⁾", "₎", "⌉", "⌋", "〉", "❩", "❫", "❭", "❯", "❱", "❳", "❵", "⟆",
    "⟧", "⟩", "⟫", "⟭", "⟯", "⦄", "⦆", "⦈", "⦊", "⦌", "⦎", "⦐", "⦒", "⦔", "⦖", "⦘", "⧙", "⧛", "⧽",
    "⸣", "⸥", "⸧", "⸩", "〉", "》", "」", "』", "】", "〕", "〗", "〙", "〛", "〞", "〟", "﴾",
    "︘", "︶", "︸", "︺", "︼", "︾", "﹀", "﹂", "﹄", "﹈", "﹚", "﹜", "﹞", "）", "］", "｝",
    "｠", "｣", "»", "’", "”", "›", "⸃", "⸅", "⸊", "⸍", "⸝", "⸡", "«", "‘", "‛", "“", "‟", "‹", "⸂",
    "⸄", "⸉", "⸌", "⸜", "⸠", "(", "[", "{", "༺", "༼", "᚛", "‚", "„", "⁅", "⁽", "₍", "⌈", "⌊", "〈",
    "❨", "❪", "❬", "❮", "❰", "❲", "❴", "⟅", "⟦", "⟨", "⟪", "⟬", "⟮", "⦃", "⦅", "⦇", "⦉", "⦋", "⦍",
    "⦏", "⦑", "⦓", "⦕", "⦗", "⧘", "⧚", "⧼", "⸢", "⸤", "⸦", "⸨", "⹂", "〈", "《", "「", "『", "【",
    "〔", "〖", "〘", "〚", "〝", "﴿", "︗", "︵", "︷", "︹", "︻", "︽", "︿", "﹁", "﹃", "﹇",
    "﹙", "﹛", "﹝", "（", "［", "｛", "｟", "｢", "!", "\"", "#", "%", "&", "'", "*", ",", ".",
    "/", ":", ";", "?", "@", "\\", "¡", "§", "¶", "·", "¿", ";", "·", "՚", "՛", "՜", "՝", "՞", "՟",
    "։", "׀", "׃", "׆", "׳", "״", "؉", "؊", "،", "؍", "؛", "؞", "؟", "٪", "٭", "۔", "܀", "܁", "܂",
    "܃", "܄", "܅", "܆", "܇", "܈", "܉", "܊", "܋", "܌", "܍", "߷", "߸", "߹", "࠰", "࠱", "࠲", "࠳", "࠴",
    "࠵", "࠶", "࠷", "࠸", "࠹", "࠺", "࠻", "࠼", "࠽", "࠾", "࡞", "।", "॥", "॰", "৽", "੶", "૰", "౷", "಄",
    "෴", "๏", "๚", "๛", "༄", "༅", "༆", "༇", "༈", "༉", "༊", "་", "།", "༎", "༏", "༐", "༑", "༒", "༔",
    "྅", "࿐", "࿑", "࿒", "࿓", "࿔", "࿙", "࿚", "၊", "။", "၌", "၍", "၎", "၏", "჻", "፠", "፡", "።", "፣",
    "፤", "፥", "፦", "፧", "፨", "᙮", "᛫", "᛬", "᛭", "᜵", "᜶", "។ល។", "។", "៕", "៖", "៘", "៙", "៚", "᠀", "᠁",
    "᠂", "᠃", "᠄", "᠅", "᠇", "᠈", "᠉", "᠊", "᥄", "᥅", "᨞", "᨟", "᪠", "᪡", "᪢", "᪣", "᪤", "᪥", "᪦",
    "᪨", "᪩", "᪪", "᪫", "᪬", "᪭", "᭚", "᭛", "᭜", "᭝", "᭞", "᭟", "᭠", "᯼", "᯽", "᯾", "᯿", "᰻", "᰼",
    "᰽", "᰾", "᰿", "᱾", "᱿", "᳀", "᳁", "᳂", "᳃", "᳄", "᳅", "᳆", "᳇", "᳓", "‖", "‗", "†", "‡", "•",
    "‣", "․", "‥", "…", "‧", "‰", "‱", "′", "″", "‴", "‵", "‶", "‷", "‸", "※", "‼", "‽", "‾", "⁁",
    "⁂", "⁃", "⁇", "⁈", "⁉", "⁊", "⁋", "⁌", "⁍", "⁎", "⁏", "⁐", "⁑", "⁓", "⁕", "⁖", "⁗", "⁘", "⁙",
    "⁚", "⁛", "⁜", "⁝", "⁞", "⳹", "⳺", "⳻", "⳼", "⳾", "⳿", "⵰", "⸀", "⸁", "⸆", "⸇", "⸈", "⸋", "⸎",
    "⸏", "⸐", "⸑", "⸒", "⸓", "⸔", "⸕", "⸖", "⸘", "⸙", "⸛", "⸞", "⸟", "⸪", "⸫", "⸬", "⸭", "⸮", "⸰",
    "⸱", "⸲", "⸳", "⸴", "⸵", "⸶", "⸷", "⸸", "⸹", "⸼", "⸽", "⸾", "⸿", "⹁", "⹃", "⹄", "⹅", "⹆", "⹇",
    "⹈", "⹉", "⹊", "⹋", "⹌", "⹍", "⹎", "⹏", "⹒", "、", "。", "〃", "〽", "・", "꓾", "꓿", "꘍", "꘎",
    "꘏", "꙳", "꙾", "꛲", "꛳", "꛴", "꛵", "꛶", "꛷", "꡴", "꡵", "꡶", "꡷", "꣎", "꣏", "꣸", "꣹", "꣺", "꣼",
    "꤮", "꤯", "꥟", "꧁", "꧂", "꧃", "꧄", "꧅", "꧆", "꧇", "꧈", "꧉", "꧊", "꧋", "꧌", "꧍", "꧞", "꧟", "꩜",
    "꩝", "꩞", "꩟", "꫞", "꫟", "꫰", "꫱", "꯫", "︐", "︑", "︒", "︓", "︔", "︕", "︖", "︙", "︰",
    "﹅", "﹆", "﹉", "﹊", "﹋", "﹌", "﹐", "﹑", "﹒", "﹔", "﹕", "﹖", "﹗", "﹟", "﹠", "﹡",
    "﹨", "﹪", "﹫", "！", "＂", "＃", "％", "＆", "＇", "＊", "，", "．", "／", "：", "；", "？",
    "＠", "＼", "｡", "､", "･", "𐄀", "𐄁", "𐄂", "𐎟", "𐏐", "𐕯", "𐡗", "𐤟", "𐤿", "𐩐", "𐩑", "𐩒", "𐩓",
    "𐩔", "𐩕", "𐩖", "𐩗", "𐩘", "𐩿", "𐫰", "𐫱", "𐫲", "𐫳", "𐫴", "𐫵", "𐫶", "𐬹", "𐬺", "𐬻", "𐬼", "𐬽", "𐬾",
    "𐬿", "𐮙", "𐮚", "𐮛", "𐮜", "𐽕", "𐽖", "𐽗", "𐽘", "𐽙", "𑁇", "𑁈", "𑁉", "𑁊", "𑁋", "𑁌", "𑁍", "𑂻", "𑂼",
    "𑂾", "𑂿", "𑃀", "𑃁", "𑅀", "𑅁", "𑅂", "𑅃", "𑅴", "𑅵", "𑇅", "𑇆", "𑇇", "𑇈", "𑇍", "𑇛", "𑇝", "𑇞", "𑇟",
    "𑈸", "𑈹", "𑈺", "𑈻", "𑈼", "𑈽", "𑊩", "𑑋", "𑑌", "𑑍", "𑑎", "𑑏", "𑑚", "𑑛", "𑑝", "𑓆", "𑗁", "𑗂", "𑗃",
    "𑗄", "𑗅", "𑗆", "𑗇", "𑗈", "𑗉", "𑗊", "𑗋", "𑗌", "𑗍", "𑗎", "𑗏", "𑗐", "𑗑", "𑗒", "𑗓", "𑗔", "𑗕", "𑗖",
    "𑗗", "𑙁", "𑙂", "𑙃", "𑙠", "𑙡", "𑙢", "𑙣", "𑙤", "𑙥", "𑙦", "𑙧", "𑙨", "𑙩", "𑙪", "𑙫", "𑙬", "𑜼", "𑜽",
    "𑜾", "𑠻", "𑥄", "𑥅", "𑥆", "𑧢", "𑨿", "𑩀", "𑩁", "𑩂", "𑩃", "𑩄", "𑩅", "𑩆", "𑪚", "𑪛", "𑪜", "𑪞", "𑪟",
    "𑪠", "𑪡", "𑪢", "𑱁", "𑱂", "𑱃", "𑱄", "𑱅", "𑱰", "𑱱", "𑻷", "𑻸", "𑿿", "𒑰", "𒑱", "𒑲", "𒑳", "𒑴", "𖩮",
    "𖩯", "𖫵", "𖬷", "𖬸", "𖬹", "𖬺", "𖬻", "𖭄", "𖺗", "𖺘", "𖺙", "𖺚", "𖿢", "𛲟", "𝪇", "𝪈", "𝪉", "𝪊", "𝪋",
    "𞥞", "𞥟", "\n", "\r", "\u{2029}", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", "　", "`", "\t", "+", "-", "±", "×", "÷", "−", "∓", "∔", "∕", "∖", "∗", "∘", "∙", "√", "∛",
    "∜", "∝", "∞", "∟", "∠", "∡", "∢", "∣", "∤", "∥", "∧", "∨", "∩", "∪", "∫", "∬", "∭", "∮", "∯",
    "∰", "∱", "∲", "∳", "∴", "∵", "∶", "∷", "∸", "∹", "∺", "∻", "∼", "∽", "∾", "∿", "≀", "≁", "≂",
    "≃", "≄", "≅", "≆", "≇", "≈", "≉", "≊", "≋", "≌", "≍", "≎", "≏", "≐", "≑", "≒", "≓", "≔", "≕",
    "≖", "≗", "≘", "≙", "≚", "≛", "≜", "≝", "≞", "≟", "≠", "≡", "≢", "≣", "≤", "≥", "≦", "≧", "≨",
    "≩", "≪", "≫", "≬", "≭", "≮", "≯", "≰", "≱", "≲", "≳", "≴", "≵", "≶", "≷", "≸", "≹", "≺",
    "≻", "≼", "≽", "≾", "≿", "⊀", "⊁", "⊂", "⊃", "⊄", "⊅", "⊆", "⊇", "⊈", "⊉", "⊊", "⊋", "⊌", "⊍",
    "⊏", "⊐", "⊑", "⊒", "⊓", "⊔", "⊕", "⊖", "⊗", "⊘", "⊙", "⊚", "⊛", "⊜", "⊝", "⊞", "⊟", "⊠", "⊡",
    "⊢", "⊣", "⊤", "⊥", "⊦", "⊧", "⊨", "⊩", "⊪", "⊫", "⊬", "⊭", "⊮", "⊯", "⊰", "⊱", "⊲", "⊳", "⊴",
    "⊵", "⊶", "⊷", "⊸", "⊹", "⊺", "⊻", "⊼", "⊽", "⊾", "⊿", "⋀", "⋁", "⋂", "⋃", "⋄", "⋅", "⋆", "⋇",
    "⋈", "⋉", "⋊", "⋋", "⋌", "⋍", "⋎", "⋏", "⋐", "⋑", "⋒", "⋓", "⋔", "⋕", "⋖", "⋗", "⋘", "⋙", "⋚",
    "⋛", "⋜", "⋝", "⋞", "⋟", "⋠", "⋡", "⋢", "⋣", "⋤", "⋥", "⋦", "⋧", "⋨", "⋩", "⋪", "⋫", "⋬", "⋭",
    "⋮", "⋯", "⋰", "⋱", "⋲", "⋳", "⋴", "⋵", "⋶", "⋷", "⋸", "⋹", "⋺", "⋻", "⋼", "⋽", "⋾", "⋿"
];

#[rustfmt::skip]
pub const CONTEXT_SEPARATORS: &[&str] = &[
    "\0", // Null byte, can be used as artificial separator
    "᠆", // Mongolian Todo Soft Hyphen, mark the end of a paragraph.
    "᚛", "᚜", // Oghams, mark start and end of text
    "!", ". ", ", ", ";", "?", "¡", "§", "¶", "¿", ";", // Latin
    "՜", // Armenian exclamation mark
    "՝", // Armenian comma
    "՞", // Armenian question mark
    "։", // Armenian full stop or period, used to indicate the end of a sentence
    "׃", // Used to signify the end of a passage, such as a verse from Scripture
    "،", // the Arabic comma punctuation mark
    "؛", // The Arabic semicolon punctuation mark
    "؟", // Reversed question mark.
    "۔", // A punctuation mark equivalent to a (.) period
    "܀", "܁", "܂", // Syriac Full Stops
    "܃", "܄", "܅", "܆", "܇", "܈", "܉", // Syriac semicolon/colon
    "߷", // The gbakurunen, used in the NKo script to indicate the end of a major section of text
    "߸", // Nko Comma
    "߹", // Nko Exclamation Mark
    "࠰", "࠱", "࠲", "࠳", "࠴", "࠵", "࠸", "࠹", "࠺", "࠻", "࠼", "࠽",
    "࠾", // samaritan marks (https://www.unicode.org/L2/L2008/08024-n3377-samaritan.pdf)
    "࡞", // Mandaic
    "।", "॥", // Dévanâgarî (http://hapax.qc.ca/Tableaux-15.0/U0900.pdf)
    "෴", // Sinhala
    "๚", "๛", // Thai
    "༄", "༅", "༆", "༇", "༈", "༉", "༊", "།", "༎", "༏", "༐", "༑", "༒", "༔", "࿐", "࿑", "࿒", "࿓", "࿔", // Tibetan (http://hapax.qc.ca/Tableaux-15.0/U0F00.pdf)
    "၊", "။", // Myanmar
    "჻", // Georgian paragraph separator
    "።", "፣", "፤", "፥", "፦", "፧", "፨", // Ethiopian
    "᛫", "᛬", "᛭", // Runic
    "᜵", "᜶", // Philippine
    "។", "៕", "៖", "៘", "។ល។", "៚", // Khmer
    "᠀", "᠁", "᠂", "᠃", "᠄", "᠅", "᠈", "᠉", // Mongolian
    "᥄", "᥅", // Limbu
    "᨟", // Buginese stop
    "᪨", "᪩", "᪪", "᪫", "᪬", "᪭", // Tai
    "᭚", "᭛", "᭜", "᭝", "᭞", "᭟", "᭠", // Balinese
    "᯼", "᯽", "᯾", "᯿", // Batak
    "᰻", "᰼", "᰽", "᰾", "᰿", // Leptcha
    "᱾", "᱿", // Ol tchiki
    "᳀", "᳁", "᳂", "᳃", "᳄", "᳅", "᳆", "᳇", // Sundanese
    "․", "‥", "…", "※", "‼", "‽", "⁇", "⁈", "⁉", "⁏", "⁖", "⁚", // general
    "⳹", "⳺", "⳻", "⳼", "⳾", // Nubian
    "⸘", "⸮", "⹁", // general part 2
    "、", "。", // CJK
    "꓾", "꓿", // Lisu
    "꘍", "꘎", "꘏", // Vai
    "꛲", "꛳", "꛴", "꛵", "꛶", "꛷", // Bamum
    "꡶", "꡷", // Phags-Pa
    "꣎", "꣏", // Saurashtra
    "꤯", // Kayah Li
    "꥟", // Rejang
    "꧃", "꧄", "꧅", "꧆", "꧇", "꧈", "꧉", "꧊", "꧋", "꧌", "꧍", // Javanese
    "꩜", "꩝", "꩞", "꩟", // Tcham
    "꫟", // Tai Viet
    "꫰", "꫱", "꯫", // Meetei Mayek
    "︐", "︑", "︒", "︓", "︔", "︕", "︖", "︙", "︰", "﹅", "﹐", "﹑", "﹒", "﹔", "﹕", "﹖", "﹗", "！", "，", "．", "：", "；", "？", "｡", "､", // general part 3
    "𐡗", // Imperial Aramaic Section Sign
    "𐩐", "𐩑", "𐩒", "𐩓", "𐩔", "𐩕", "𐩖", "𐩗", "𐩘", // Kharoshthi
    "𐫰", "𐫱", "𐫲", "𐫳", "𐫴", "𐫵", // Manichaean
    "𐬺", "𐬻", "𐬼", "𐬽", "𐬾", "𐬿", // Avestan
    "𐮙", "𐮚", "𐮛", "𐮜", // Psalter Pahlavi
    "𐽕", "𐽖", "𐽗", "𐽘", "𐽙", // Sogdian
    "𑁇", "𑁈", "𑁉", "𑁊", "𑁋", "𑁌", "𑁍", // Brahmi
    "𑂾", "𑂿", "𑃀", "𑃁", // Kaithi
    "𑅀", "𑅁", "𑅂", "𑅃", // Chakma
    "𑅵", // Mahajani
    "𑇅", "𑇆", "𑇍", "𑇞", "𑇟", // Sharada
    "𑈸", "𑈹", "𑈻", "𑈼", // Khojki
    "𑊩", // Multani
    "𑑋", "𑑌", "𑑍", "𑑚", // Newa
    "𑗂", "𑗃", "𑗉", "𑗊", "𑗋", "𑗌", "𑗍", "𑗎", "𑗏", "𑗐", "𑗑", "𑗒", "𑗓", "𑗔", "𑗕", "𑗖", "𑗗", // Siddham
    "𑙁", "𑙂", // Modi
    "𑙠", "𑙡", "𑙢", "𑙣", "𑙤", "𑙥", "𑙦", "𑙧", "𑙨", "𑙩", "𑙪", "𑙫", "𑙬", // Mongolian ornements
    "𑜼", "𑜽", "𑜾", // Ahom
    "𑥄", "𑥆", // Dives Akuru
    "𑨿", "𑩀", "𑩁", "𑩂", "𑩃", "𑩄", "𑩅", "𑩆", // Zanabazar
    "𑪚", "𑪛", "𑪜", "𑪞", "𑪟", "𑪠", "𑪡", "𑪢", // Soyombo
    "𑱁", "𑱂", // Bhaiksuki
    "𑱰", "𑱱", // Marchen
    "𑻷", "𑻸", // Makasar
    "𑿿", // Tamil
    "𒑱", "𒑲", "𒑳", "𒑴", // Cuneiform
    "𖩮", "𖩯", // Mro
    "𖫵", // Bassa Vah
    "𖬷", "𖬸", "𖬹", "𖭄", // Pahawh Hmong
    "𖺗", "𖺘", "𖺙", "𖺚", // Medefaidrin
    "𛲟", // Duployan
    "𝪇", "𝪈", "𝪉", "𝪊", // Signwriting
    "𞥞", "𞥟", // Adlam
];
