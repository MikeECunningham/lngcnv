// LNGCNV VERSION 1.6.0-BETA.6 / MIT LICENSE © 2022 PIOTR BAJDEK

// MODULE MODENG

// CLIPPY LINTS

#![deny(clippy::no_effect_replace)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::single_char_pattern, clippy::too_many_lines, clippy::unicode_not_nfc)]
#![allow(clippy::string_lit_as_bytes)] // must be as_bytes() because non-ASCII characters are included

// IMPORTS

use std::fs::OpenOptions;
use std::io::Write;

// SIMPLIFY INTERPUNCTION

fn engpncbeg(lowercase: &str) -> String {
    let pncbeg = &lowercase.replace(';', ".").replace(':', ",").replace('!', ".").replace('?', ".").replace("--", " – ").replace("),", ",").replace(").", ".").replace(')', ",").replace('(', "∣ .");
    pncbeg.to_string()
}

// REMOVE INTERPUNCTION

fn engpncend(strmod: &str) -> String {
    let result = &strmod.replace(',', " ∣").replace(". ", " ∥ ").replace('.', "").replace(" - ", " ∣ ").replace(" – ", " ∣ ").replace("∣ ∥", "∥");
    result.to_string()
}

// ENGLISH: IPA

pub fn engaucanberra(original_text: &str, usefile: &str, outputfile: &str) {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";
    let yellow = "\x1b[93m";

    let dotend = original_text.to_owned() + "."; // mark word ending
    let dotbeg = ".".to_owned() + &dotend; // word beginning
    let lowercase = &dotbeg.to_lowercase();
    let pncbeg = engpncbeg(lowercase);

    let strmod = &pncbeg
        .replace("i'd", "ɑ̝ed")
        .replace("i’d", "ɑ̝ed")
        .replace("i'll", "ɑ̝el")
        .replace("i’ll", "ɑ̝el")
        .replace("i'm", "ɑ̝em")
        .replace("i’m", "ɑ̝em")
        .replace("'", "")
        .replace('’', "")
        .replace("etc.", "etˈsɛ̝t͡ʃɹə.")
        .replace("eah", "ɶ̜ə")
        .replace("answ", "ɐ̞ːns")
        .replace("ballet", "bɐlei^")
        .replace("caramel", "ˈkʰɶ̜ɹməɫ")
        .replace("challeng", "t͡ʃɶ̜l̥ind͡ʒ")
        .replace("christmas", "krismas")
        .replace("debris", "dibɹiː")
        .replace("entrepreneur", "ˌɔ̝nt̠ɹ̥əpɹəˈnɜː")
        .replace("garage", "ˈɡɐɹəd͡ʒ")
        .replace("listen", "lisᵊn")
        .replace("often", "ɔ̝fᵊn")
        .replace("perhaps", "peːhɶ̜ps")
        .replace("comm", "kʰɔ̝mm")
        .replace("ionaire", "io̝ːnə")
        .replace("alumi", "ælʲʉ̟ːmi")
        .replace("nium", "nʲəm")
        .replace("subtl", "sɐtl")
        .replace("debt", "det")
        .replace("wedn", "wɛn")
        .replace("yed", "yd̥")
        .replace("bb", "b")
        .replace("cce", "kse")
        .replace("cci", "ksi")
        .replace("musc", "mɐsc")
        .replace("psycho", "sɑ̝eko")
        .replace("sca", "s.ca")
        .replace("sc", "s")
        .replace("cen", "sen")
        .replace("dd", "d̥")
        .replace("choir", "kwɑ̝eə")
        .replace("ghost", "gɞ̜ʉ̞st")
        .replace("gue", "ge")
        .replace("kn", "n")
        .replace("qu", "kw")
        .replace("rh", "ɹ")
        .replace("ttem", "tʰem")
        .replace("tten", "tʰen")
        .replace("wr", "ɹ")
        .replace("wha", "wɔ̝")
        .replace("dess", "diss")
        .replace("sse", "ze")
        .replace("pad", "pɶ̜d")
        .replace("ve read", "ve ɹeːd")
        .replace("has read", "has ɹeːd")
        .replace("nt read", "nt ɹeːd")
        .replace("ered", "əd̥")
        .replace("bare", "beː")
        .replace("bari", "beːɹi")
        .replace("care", "kʰeː")
        .replace("cari", "kʰeːɹi")
        .replace("dare", "deː")
        .replace("dari", "deːɹi")
        .replace("mare", "meː")
        .replace("mari", "meːɹi")
        .replace("share", "ʃeː")
        .replace("shari", "ʃeːɹi")
        .replace("tare", "teː")
        .replace("tari", "teːɹi")
        .replace("ware", "weː")
        .replace("esta", "ista")
        .replace("est ", "ist ")
        .replace("est.", "ist.")
        .replace("est,", "ist,")
        .replace("bist", "best")
        .replace("rist", "rest")
        .replace("uist", "est")
        .replace("wist", "west")
        .replace("sword", "sɞːd̥")
        .replace("ord", "ə̟ːd")
        .replace("orld", "ə̟ːld")
        .replace("ork", "ə̟ːk")
        .replace("urn", "ə̟ːn")
        .replace("orth", "o̝ːθ")
        .replace("ore̞st", "o̝ɹest")
        .replace("ore", "o̝ː")
        .replace("ory", "ɔ̝ːɾy")
        .replace("or", "ɔ̝ː")
        .replace("air", "eː")
        .replace("there", "ðeː")
        .replace("were", "weː")
        .replace("circ", "sə̟ːc")
        .replace("ira", "ɑ̝era")
        .replace("iro", "ɑ̝ero")
        .replace("irr", "iɹ")
        .replace("ir", "ə̟ː")
        .replace("urr", "ɐɹ")
        .replace("urse", "ə̟ːs")
        .replace("isl", "ɑ̝el")
        .replace("exp", "ixp")
        .replace("rious", "ɹiəs")
        .replace("cious", "ʃiəs")
        .replace("tious", "ʃiəs")
        .replace("ous", "əs")
        .replace("bour", "bə")
        .replace("iour", "j|ə")
        .replace("nour", "nə")
        .replace("olour", "ɔ̝lə")
        .replace("youre", "j|ʉ̟ːə")
        .replace("your", "j|ʉ̟ː")
        .replace("ourse", "o̝ːs")
        .replace("four", "fo̝ː")
        .replace("our", "æ̞ɔ̞ə")
        .replace("easure", "eːʒə")
        .replace("eisure", "ɜʒə")
        .replace("sure", "ʃʲɔ̝ː")
        .replace("cture", "ct͡ʃə")
        .replace("ure", "ʲʉ̟ːə")
        .replace("ounce", "æɔ̞̃nce")
        .replace("ounci", "æɔ̞̃nsi")
        .replace("ound", "æɔ̞̃nd̥")
        .replace("ceipt", "ceit")
        .replace("ow", "æ̞ɔ̞")
        .replace("wea", "weˑ")
        .replace("eady", "edy")
        .replace("eason", "ᵊiːz̥ᵊn")
        .replace("ear ", "iə ")
        .replace("ear", "eː")
        .replace("eali", "ᵊiːəli")
        .replace("ea", "ᵊiː")
        .replace("who ", "hʉ̟ː ")
        .replace("who", "ho")
        .replace("wh", "w")
        .replace("aunt", "ɐ̞ːnt")
        .replace("daughter", "dɔ̝tə")
        .replace("au", "ɔ̝")
        .replace("any", "æ̃ni")
        .replace("archi", "ɐːkʰi")
        .replace("arm", "õ̝ːm")
        .replace("arn", "õ̝ːn")
        .replace("harr", "hɶ̜ɹ")
        .replace("arr", "əɹ")
        .replace("are ", "ɐ̞ː ")
        .replace("eyre", "eː")
        .replace("re ", "ə ")
        .replace(" ar", " əɹ")
        .replace(".ar", ".əɹ")
        .replace("liar", "lɑ̝eə")
        .replace("ara", "ɐ̞ːɹa")
        .replace("ar a", "ɐ̞ːɹ a")
        .replace("ar e", "ɐ̞ːɹ e")
        .replace("ar o", "ɐ̞ːɹ o")
        .replace("ar", "ɐ̞ː")
        .replace("ust", "ɐst")
        .replace("tri", "t͡ʃɹi")
        .replace("str", "sṯɹ̥")
        .replace("spr", "spɹ")
        .replace("pr", "pɹ̥")
        .replace("thought", "θought")
        .replace("outh", "æ̞ɔ̞θ")
        .replace("out", "æ̞ɔ̞t")
        .replace("obi", "ɞ̜ʉ̞bi")
        .replace("ock", "ɔ̝ck")
        .replace("uke", "ʲʉ̟ːk")
        .replace("ude", "ʲʉ̟ːd")
        .replace("ee", "ᵊiː")
        .replace(" ev", " iv")
        .replace(".ev", ".iv")
        .replace("cially", "ʃl̥ᵊiː")
        .replace("dia", "dɑ̝e")
        .replace("ia", "iːə")
        .replace("ile", "ɑ̝eɫ")
        .replace("ance", "æ̃nse")
        .replace("anci", "æ̃nsi")
        .replace("astle", "ɶ̜sᵊl")
        .replace("stle", "sᵊl")
        .replace("trast", "trɶ̜st")
        .replace("and", "æ̃nd")
        .replace("vel", "vɫ")
        .replace("ain", "ɶ̜in")
        .replace("behav", "bihɶ̜iv")
        .replace("hav", "hɶ̜v")
        .replace("ave", "ɶ̜iv")
        .replace("avi", "ɶ̜ivi")
        .replace("ame", "ɶ̜ĩm")
        .replace("ami", "ɶ̜ĩmi")
        .replace("ache", "ɶ̜ik̥ʰ")
        .replace("ake", "ɶ̜ik̥e")
        .replace("aki", "ɶ̜ik̥i")
        .replace("ace", "ɶ̜is")
        .replace("aci", "ɶ̜isi")
        .replace("ade", "ɶ̜id̥")
        .replace("adi", "ɶ̜id̥i")
        .replace("ase", "ɶ̜ise")
        .replace("asi", "ɶ̜isi")
        .replace("ice", "ɑ̝es")
        .replace("pɹ̥ove", "pɹ̥ʉ̟ːve")
        .replace("pɹ̥ovid", "pɹ̥əvid")
        .replace("pɹ̥ovi", "pɹ̥ʉ̟ːvi")
        .replace("ide", "ɑ̝ed̥e")
        .replace("idi", "ɑ̝ed̥i")
        .replace("water", "wɒðə")
        .replace("man", "mɶ̜n")
        .replace("ssage", "sid͡ʒ")
        .replace("uage", "wid͡ʒ")
        .replace("nage", "nid͡ʒ")
        .replace("age", "ɶ̜id͡ʒe")
        .replace("magi", "mɶ̜d͡ʒi")
        .replace("agi", "ɶ̜id͡ʒi")
        .replace("ale", "ɶ̜ile")
        .replace("ike", "ɑ̝eke")
        .replace("iki", "ɑ̝eki")
        .replace("ife", "ɑ̝ef")
        .replace("ime", "ɑ̝eme")
        .replace("machine", "məʃin")
        .replace("line", "lɑ̝en")
        .replace("mine", "mɑ̝en")
        .replace("iny", "ɑ̝eny")
        .replace("ini", "ɑ̝eni")
        .replace("ison", "isᵊn")
        .replace("itely", "ʔl̥ᵊiː")
        .replace("ite", "ɑ̝ete")
        .replace("ate", "ɶ̜ite")
        .replace("ati", "ɶ̜iti")
        .replace("iti", "ɑ̝eti")
        .replace("ita", "ɑ̝eta")
        .replace("ɑ̝etc", "itec")
        .replace("ise", "ɑ̝ese")
        .replace("isi", "ɑ̝esi")
        .replace("ize", "ɑ̝ese")
        .replace("izi", "ɑ̝esi")
        .replace("ype", "ɑ̝epe")
        .replace("ypi", "ɑ̝epi")
        .replace("ay", "ɶ̜i^")
        .replace("ey ", "y ")
        .replace("ey,", "y,")
        .replace("ey.", "y.")
        .replace("ey", "ɑ̝e^")
        .replace("aw", "ɔ̝")
        .replace("aste", "ɶ̜is̺t")
        .replace("asthma", "ɶ̜smə")
        .replace("ast", "ɐ̞ːst")
        .replace("etable", "tbɫ")
        .replace("ortable", "tbɫ")
        .replace("shal", "ʃʲɶ̜l")
        .replace("gra", "ɡɹ̥ɶ̜")
        .replace("sylla", "silə")
        .replace("berra", "brra")
        .replace("able", "ɶ̜ibɫ")
        .replace("cæi", "kʰɶ̜i")
        .replace("cæĩ̞", "kʰɶ̜ĩ̞")
        .replace("tæi", "tʰɶ̜i")
        .replace("tæĩ̞", "tʰɶ̜ĩ̞")
        .replace("imme", "ə̃mᵊiː")
        .replace("was", "wɔ̝z̥")
        .replace("obli", "əblɑ̝e")
        .replace("ob", "əb")
        .replace("oubl", "ɐbl")
        .replace("ble ", "bɫ ")
        .replace("ble.", "bɫ.")
        .replace("ble,", "bɫ,")
        .replace("igh", "ɑ̝e^")
        .replace("die", "d̥ɑ̝e^")
        .replace("lie", "lɑ̝e^")
        .replace("tie", "tɑ̝e^")
        .replace("pie", "pɑ̝e^")
        .replace("iet", "ɑ̝eət")
        .replace("by", "bɑ̝e^")
        .replace("my", "mɑ̝e^")
        .replace("cry", "kɹ̥ɑ̝e^")
        .replace("dry", "dɹ̥ɑ̝e^")
        .replace("fly", "flɑ̝e^")
        .replace("shy", "ʃɑ̝e^")
        .replace("sky", "skɑ̝e^")
        .replace("try", "trɑ̝e^")
        .replace("why", "wɑ̝e^")
        .replace("iche", "iːʃ")
        .replace("uel", "ʲʉ̟ːᵊl")
        .replace("oa", "ɞ̜ʉ̞")
        .replace("cɞ̜ʉ̞", "kʰɞ̜ʉ̞")
        .replace("ɞ̜ʉ̞l", "ɔ̝o̝ːɫ")
        .replace("ole", "ɔ̝o̝ːɫ")
        .replace("ool", "uːɫ")
        .replace("oll", "ɔ̝ɫ")
        .replace("old", "ɔ̝o̝ːd")
        .replace("dont", "dɞ̜ʉ̞nt̚ ")
        .replace("ond", "ənd̥")
        .replace("oes", "ɐs")
        .replace("cous", "kʰɐs")
        .replace("touch", "tɐt͡ʃ")
        .replace("ouch", "æ̞ɔ̞t͡ʃ")
        .replace("oud", "æ̞ɔ̞d")
        .replace("oubt", "æ̞ɔ̞t")
        .replace("ought", "ɔ̝t")
        .replace("eno", "ino")
        .replace("ough", "ɐf")
        .replace("could", "kʰʊd")
        .replace("ouse", "ɑ̝ese")
        .replace("ou", "ʉ̟ː")
        .replace("ost", "æ̞ɔ̞st")
        .replace("ooe", "ʉ̟ː")
        .replace("ood", "ɐd")
        .replace("oo", "ʊ")
        .replace("ose", "ɞ̜ʉ̞s")
        .replace("use", "ʲʉ̟ːz")
        .replace("usi", "ʲʉ̟ːzi")
        .replace("actu", "ɶ̜kʃʉ̟ː")
        .replace("act", "ɶ̜kt")
        .replace("alf", "ɐ̞ːf")
        .replace("alk", "ɒːk")
        .replace("alm", "ɐ̞̃ːm")
        .replace("eith", "iːð")
        .replace("urth", "e̞ːð")
        .replace("ura", "ʲʉ̟ːra")
        .replace("ewe", "ʲʉ̟ːe")
        .replace("ew", "ʲʉ̟ː")
        .replace("une", "ʲʉ̟̃ːn")
        .replace("uni", "ʲʉ̟̃ːni")
        .replace("tʲʉ̟̃ːn", "t͡ʃʉ̟̃ːn")
        .replace("dʲʉ̟̃ːn", "d͡ʒʉ̟̃ːn")
        .replace("ume", "ʲʉ̃ːm")
        .replace("umi", "ʲʉ̃ːmi")
        .replace("assʲʉ̃ːm", "əʃʉ̃ːm")
        .replace("upi", "ʲʉ̟ːpi")
        .replace("syd", "sid")
        .replace("ute", "ʲʉ̟ːte")
        .replace("uti", "ʲʉ̟ːti")
        .replace("move", "mʉ̟ːv")
        .replace("movi", "mʉ̟ːvi")
        .replace("love", "lɔ̜ve")
        .replace("lovi", "lɔ̜vi")
        .replace("to dove", "to dɞ̜ʉ̞v")
        .replace("dove", "dɔ̜v")
        .replace("ove", "ɞ̜ʉ̞ve")
        .replace("uck", "ɐk")
        .replace("some", "sɐm")
        .replace("ome", "ɞ̜ʉ̞m")
        .replace("opef", "ɞ̜ʉ̞pf")
        .replace("opel", "ɞ̜ʉ̞pl")
        .replace("ope", "ɞ̜ʉ̞pe")
        .replace("opi", "ɞ̜ʉ̞pi")
        .replace("ode", "ɞ̜ʉ̞d̥e")
        .replace("ote", "ɞ̜ʉ̞t")
        .replace("oti", "ɞ̜ʉ̞ti")
        .replace("oto", "ɞ̜ʉ̞to")
        .replace("uild", "ild")
        .replace("uilt", "ilt")
        .replace("uin", "uːin")
        .replace("cuit", "kʰit")
        .replace("guit", "git")
        .replace("uit", "uːt")
        .replace("off", "ɔ̝f")
        .replace("of", "ɔ̝v")
        .replace("sson", "sᵊn")
        .replace("con", "kə̃n")
        .replace("con", "kə̃n")
        .replace("gone", "gɔ̝̃n")
        .replace("phon", "fɔ̝̃n")
        .replace("ton", "tɔ̝̃n")
        .replace("zon", "zɔ̝̃n")
        .replace("once", "wɐ̃ns")
        .replace("one", "wɐ̃n")
        .replace("engl", "ingl")
        .replace("ng", "ŋ")
        .replace("aŋ", "ɶ̜̃ŋ")
        .replace("tiŋ", "ɾĩŋ")
        .replace("i^iŋ", "iŋ")
        .replace("iŋ", "ĩŋ")
        .replace("oŋ", "ɔ̝̃ŋ")
        .replace("ent ", "ənt ")
        .replace("ent.", "ənt.")
        .replace("ent,", "ənt,")
        .replace("ene", "iːn")
        .replace("en", "ẽn̪")
        .replace("in", "ĩn")
        .replace("im", "ĩm")
        .replace("un", "ɐ̃n")
        .replace("up", "ɐp")
        .replace("but ", "bət ")
        .replace("but,", "bət,")
        .replace("ak", "æk")
        .replace("ai", "ɶ̜i^")
        .replace("ik", "ikʰ")
        .replace("this", "ðis^")
        .replace("is ", "iz̥ ")
        .replace("is.", "iz̥.")
        .replace("is,", "iz̥,")
        .replace("ẽ̞n̪th", "ẽn̪θ")
        .replace("othes", "ɞ̜ʉ̞ðz")
        .replace("oth", "ɐð")
        .replace("uth", "ʉθ")
        .replace("ythm", "iðᵊm")
        .replace("yme", "ɑ̝eme")
        .replace("gh", "g")
        .replace("orth", "oːθ")
        .replace("that", "ðət")
        .replace("than", "ðə̃n̪")
        .replace("tha", "ðæ")
        .replace("thẽn̪", "ðẽn̪")
        .replace("these", "ðiːz")
        .replace("the ", "ðə ")
        .replace("thi", "θi")
        .replace("th", "ð")
        .replace("two", "tʰʉ̟ː")
        .replace("pt", "p̚t")
        .replace("spu", "spʲʉ̟ː")
        .replace("pu", "pʰu")
        .replace("pʊ", "pʰʊ")
        .replace("tʊ", "tʰʊ")
        .replace("dge", "d͡ʒe")
        .replace("cant", "kʰɐ̞̃ːnt")
        .replace("can", "kʰæ̃ːn")
        .replace("ken", "kʰɛ̃ːn")
        .replace("we ", "wᵊiː ")
        .replace("we.", "wᵊiː.")
        .replace("we,", "wᵊiː,")
        .replace("she ", "ʃᵊiː ")
        .replace("she.", "ʃᵊiː.")
        .replace("she,", "ʃᵊiː,")
        .replace("shes ", "ʃᵊiːz ")
        .replace("he ", "hᵊiː ")
        .replace("he.", "hᵊiː.")
        .replace("he,", "hᵊiː,")
        .replace("hes ", "hᵊiːz ")
        .replace("hi", "hɑ̝e^")
        .replace("sh", "ʃ")
        .replace("sto", "stɔ̝")
        .replace("nf", "ɱf")
        .replace("oɱ", "ə̃ɱ")
        .replace("om", "ə̃m")
        .replace("bad", "bɶ̜d")
        .replace("hab", "hɶ̜b")
        .replace("had", "hɶ̜d")
        .replace("suk", "sək")
        .replace("urg", "ɜːg")
        .replace("app", "ɶ̜p")
        .replace("bit", "bət")
        .replace("rot", "ɹət")
        .replace("ot", "ɔ̝t")
        .replace("ert", "eːt")
        .replace("ere ", "iə ")
        .replace("ere.", "iə.")
        .replace("ere,", "iə,")
        .replace("mber", "mer")
        .replace("er a", "əɹ a")
        .replace("er e", "əɹ e")
        .replace("er o", "əɹ o")
        .replace("er ", "ə ")
        .replace("er.", "ə.")
        .replace("er,", "ə,")
        .replace("ses", "zəz̥")
        .replace("sas", "zəz̥")
        .replace("as ", "əz̥ ")
        .replace("as.", "əz̥.")
        .replace("as,", "əz̥,")
        .replace("ped ", "p̚t ")
        .replace("ped.", "p̚t.")
        .replace("ped,", "p̚t,")
        .replace("d͡ʒed", "d͡ʒd̚")
        .replace("d͡ʒd", "d͡ʒd̚")
        .replace("ked", "kt")
        .replace("k̥ed", "kt")
        .replace("ed ", "ᵊd̥ ")
        .replace("ed.", "ᵊd̥.")
        .replace("ed,", "ᵊd̥,")
        .replace("bəd̥", "bed̥")
        .replace("bad̥", "bæd̥")
        .replace("bod", "bɔ̝d")
        .replace("be ", "bi^ ")
        .replace("be.", "bi.")
        .replace("be,", "bi,")
        .replace(" be", " bi")
        .replace(".be", ".bi")
        .replace(" me ", " mi^ ")
        .replace(" me.", " mi.")
        .replace(" me,", " mi,")
        .replace(".me,", ".mi,")
        .replace("nal", "nᵊl")
        .replace("ral", "ɹᵊl")
        .replace(" all ", " ɔ̝ːl ")
        .replace(".all ", ".ɔ̝ːl ")
        .replace(" all", " əl")
        .replace(".all", ".əl")
        .replace("all", "ɔ̝ːl")
        .replace(" al", " ɔ̝ːl")
        .replace(".al", ".ɔ̝ːl")
        .replace(" re", " ɹi")
        .replace(".re", ".ɹi")
        .replace("ation", "ɶ̜iʃŋ")
        .replace("tion", "ʃŋ")
        .replace("sion", "ʃŋ")
        .replace("zion", "ʒᵊŋ")
        .replace("cky", "kiː")
        .replace("ctly", "t͡ʃl̥ᵊiː")
        .replace("tely", "ʔl̥ᵊiː")
        .replace("llery", "l̥ɹᵊiː")
        .replace("ily", "l̥ᵊiː")
        .replace("ly", "l̥ᵊiː")
        .replace("ical", "ikᵊl")
        .replace("ic ", "ikʰ ")
        .replace("ic.", "ikʰ.")
        .replace("ic,", "ikʰ,")
        .replace("ce ", "s ")
        .replace("ce.", "s.")
        .replace("ce,", "s,")
        .replace(" ive ", " ɑ̝ev ")
        .replace(".ive ", ".ɑ̝ev ")
        .replace("five", "fɑ̝ev")
        .replace("e ", " ")
        .replace("e.", ".")
        .replace("e,", ",")
        .replace(" do ", " dʊ ")
        .replace(" do.", " dʊ.")
        .replace(" do,", " dʊ,")
        .replace(".do ", ".dʊ ")
        .replace("to", "tə")
        .replace("o ", "ɞ̜ʉ̞ ")
        .replace("o.", "ɞ̜ʉ̞.")
        .replace("o,", "ɞ̜ʉ̞,")
        .replace("cy ", "sᵊiː ")
        .replace("cy.", "sᵊiː.")
        .replace("cy,", "sᵊiː,")
        .replace("logy", "ləd͡ʒᵊiː")
        .replace("y ", "ᵊiː ")
        .replace("y.", "ᵊiː.")
        .replace("y,", "ᵊiː,")
        .replace("ci", "si")
        .replace("eful", "fʊl")
        .replace("ful", "fʊl")
        .replace("diff", "dif")
        .replace("mach", "məkʰ")
        .replace("tch", "t͡ʃ")
        .replace("ch ", "t͡ʃ ")
        .replace("ch.", "t͡ʃ.")
        .replace("ch,", "t͡ʃ,")
        .replace("chɶ̜", "t͡ʃɶ̜")
        .replace("chʊ", "t͡ʃʊ")
        .replace("chec", "t͡ʃec")
        .replace("cho", "t͡ʃo")
        .replace("ch", "k")
        .replace("cc", "k")
        .replace("ck", "k")
        .replace("c", "k")
        .replace("kɶ̜", "kʰɶ̜")
        .replace("kl", "kl̥")
        .replace("gr", "ɡɹ̥")
        .replace("tr", "ṯɹ̥")
        .replace("um", "ɐ̃m")
        .replace("pʰut", "pʰʊt")
        .replace("ut", "ɐt")
        .replace("ud", "ɐd")
        .replace("ʊk", "ʊkʰ")
        .replace("ɶ̜k", "ɶ̜kʰ")
        .replace("r", "ɹ")
        .replace("ck", "k")
        .replace("dɹ", "d͡ʒɹ̥")
        .replace("dst", "d.st")
        .replace("ds", "d͡z")
        .replace("d̥s", "d͡z")
        .replace("ns ", "nz̥ ")
        .replace("ns.", "nz̥.")
        .replace("ns,", "nz̥,")
        .replace("ŋs", "ŋs̬")
        .replace("ɶ̜is", "ɶ̜iz̥")
        .replace("x", "ks")
        .replace("ss", "s")
        .replace("s", "s̻")
        .replace("i ", "ɑ̝e ")
        .replace("ph", "f")
        .replace("ff", "f")
        .replace("pp", "p")
        .replace("pf", "f")
        .replace("h ", " ")
        .replace("h.", ".")
        .replace("h,", ",")
        .replace("mm", "m")
        .replace("mb ", "m ")
        .replace("mb.", "m.")
        .replace("mb,", "m,")
        .replace(" am ", " ɶ̜̃m ")
        .replace(" am.", " ɶ̜̃m.")
        .replace(" am,", " ɶ̜̃m,")
        .replace(".am ", ".ɶ̜̃m ")
        .replace("damn", "dɶ̜̃m")
        .replace("amn ", "ɶ̜̃m ")
        .replace("amn.", "ɶ̜̃m.")
        .replace("amn,", "ɶ̜̃m,")
        .replace("ɶ̜̃n", "æ̃n)")
        .replace("umn", "ᵊm")
        .replace("ymn", "im")
        .replace("nn", "n")
        .replace("s̻nt", "zᵊnt̚")
        .replace("nt ", "nt̚  ")
        .replace("nt.", "nt̚ .")
        .replace("nt,", "nt̚ ,")
        .replace("j", "d͡ʒ")
        .replace("d͡ʒ|", "j")
        .replace("oy", "o̝ɪ")
        .replace("at an", "aɹ an")
        .replace("y", "j")
        .replace("a", "ə")
        .replace("ə̹i", "ɑ̝e")
        .replace("e-", "iː")
        .replace("e", "e̞")
        .replace("i", "i̞")
        .replace("i̞ː", "iː")
        .replace("ĩ̞", "ɪ̃")
        .replace("ət", "ət̪")
        .replace("tt", "ð")
        .replace("rr", "ɹ")
        .replace("r", "ɹ")
        .replace("ɹɹ", "ɹ")
        .replace("ll", "l")
        .replace("l ", "ɫ ")
        .replace("l.", "ɫ.")
        .replace("l,", "ɫ,")
        .replace("ɑ̝e̞ɫ", "ɑ̝e̞ᵊɫ")
        .replace("iːɫ", "iːᵊɫ")
        .replace("ʉ̟ːld", "ʉ̟ːd̥")
        .replace("ld", "l̥d")
        .replace("lw", "l̥w")
        .replace("t̪ ð", "t̪̚ ð")
        .replace("d ð", "d̪̚ ð")
        .replace("d ", "d̥ ")
        .replace("d.", "d̥.")
        .replace("d,", "d̥,")
        .replace("ðə ə", "ðiː ə")
        .replace("ðə ɐ", "ðiː ɐ")
        .replace("i̞ i̞", "i̞")
        .replace("n m", "m m")
        .replace("n n", " n")
        .replace("g", "ɡ")
        .replace('^', "");

    let result = engpncend(strmod);

    if usefile == "new" {
        let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
        file.write_all("CANBERRA, ACT, AU: [STILL IN ALPHA STAGE]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "old" {
        let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
        file.write_all("CANBERRA, ACT, AU: [STILL IN ALPHA STAGE]".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("Canberra, ACT, AU: {}", red.to_owned() + "[still in alpha stage]" + reset);
        println!();
        print!("{}", yellow);
        println!("{}", result);
        print!("{}", reset);
    }
}

//   ++++++++++   ++++++++++   ++++++++++

// ENGLISH: ORTHOGRAPHY

pub fn ortuseng(original_text: &str, usefile: &str, outputfile: &str) {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";
    let yellow = "\x1b[93m";

    let space = original_text.to_owned() + " "; // mark word ending
    let result = &space
        .replace("celling", "celing")
        .replace("delling", "deling")
        .replace("velling", "veling")
        .replace("bour", "bor")
        .replace("iour", "ior")
        .replace("nour", "nor")
        .replace("olour", "olor")
        .replace("centre", "center")
        .replace("metre", "meter")
        .replace("theatre", "theater")
        .replace("fence", "fense")
        .replace("lise", "lize")
        .replace("lyse", "lyze")
        .replace("sation", "zation")
        .replace("aestetic", "estetic")
        .replace("Aestetic", "Estetic")
        .replace("faec", "fec")
        .replace("Faec", "Fec")
        .replace("palaeo", "paleo")
        .replace("Palaeo", "Paleo")
        .replace("manoeuvre", "maneuver")
        .replace("aluminium", "aluminum")
        .replace("Aluminium", "Aluminum")
        .replace(" axe ", " ax ")
        .replace(" axe,", " ax,")
        .replace(" axe.", " ax.")
        .replace("cheque", "check")
        .replace("cumquat", "kumquat")
        .replace("fulfil", "fulfill")
        .replace("fulfilll", "fulfill")
        .replace("Fulfil", "Fulfill")
        .replace("Fulfilll", "Fulfill")
        .replace("grey", "gray")
        .replace("Grey", "Gray")
        .replace("kerb", "curb")
        .replace("moustache", "mustache")
        .replace("mum ", "mom ")
        .replace("mum,", "mom,")
        .replace("mum.", "mom.")
        .replace("Mum,", "Mom,")
        .replace("programme", "program")
        .replace("Programme", "Program")
        .replace("sceptical", "skeptical")
        .replace("Sceptical", "Skeptical")
        .replace("CELLING", "CELING")
        .replace("DELLING", "DELING")
        .replace("VELLING", "VELING")
        .replace("BOUR", "BOR")
        .replace("IOUR", "IOR")
        .replace("NOUR", "NOR")
        .replace("OLOUR", "OLOR")
        .replace("CENTRE", "CENTER")
        .replace("METRE", "METER")
        .replace("THEATRE", "THEATER")
        .replace("FENCE", "FENSE")
        .replace("LISE", "LIZE")
        .replace("LYSE", "LYZE")
        .replace("SATION", "ZATION")
        .replace("AESTETIC", "ESTETIC")
        .replace("FAEC", "FEC")
        .replace("PALAEO", "PALEO")
        .replace("MANOEUVRE", "MANEUVER")
        .replace("ALUMINIUM", "ALUMINUM")
        .replace(" AXE ", " AX ")
        .replace(" AXE,", " AX,")
        .replace(" AXE.", " AX.")
        .replace("CHEQUE", "CHECK")
        .replace("CUMQUAT", "KUMQUAT")
        .replace("FULFIL", "FULFILL")
        .replace("FULFILLL", "FULFILL")
        .replace("GREY", "GRAY")
        .replace("KERB", "CURB")
        .replace("MOUSTACHE", "MUSTACHE")
        .replace("MUM ", "MOM ")
        .replace("MUM,", "MOM,")
        .replace("MUM.", "MOM.")
        .replace("PROGRAMME", "PROGRAM")
        .replace("SCEPTICAL", "SKEPTICAL")
        .replace("liquorice", "licorice")
        .replace("LIQUORICE", "LICORICE")
        .replace("tyre", "tire")
        .replace("TYRE", "TIRE")
        .replace("wellery", "welry")
        .replace("WELLERY", "WELRY");

    if usefile == "new" {
        let mut file = std::fs::File::create(outputfile).expect(&(red.to_owned() + "The output file could not be created!" + reset));
        file.write_all("AMERICAN ENGLISH:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "old" {
        let mut file = OpenOptions::new().append(true).open(outputfile).expect(&(red.to_owned() + "cannot open file" + reset));
        file.write_all("AMERICAN ENGLISH:".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all(result.as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
        file.write_all("\n".as_bytes()).expect(&(red.to_owned() + "write failed" + reset));
    }
    if usefile == "terminal" {
        println!();
        println!("American English:");
        println!();
        print!("{}", yellow);
        println!("{}", result);
        print!("{}", reset);
    }
}
