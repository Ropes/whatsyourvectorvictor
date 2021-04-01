use phf::phf_map;

static ALPHABET: phf::Map<&'static str, &'static str> = phf_map! {
    "a" => "alpha",
    "b"=> "bravo",
    "c"=> "charlie",
    "d"=> "delta",
    "e"=> "echo",
    "f"=> "foxtrot",
    "g"=> "golf",
    "h"=> "hotel",
    "i"=> "india",
    "j"=> "juliet",
    "k"=> "kilo",
    "l"=> "lima",
    "m"=> "mike",
    "n"=> "november",
    "o"=> "oscar",
    "p"=> "papa" ,
    "q"=> "quebec",
    "r"=> "romeo",
    "s"=> "sierra",
    "t"=> "tango",
    "u"=> "uniform",
    "v"=> "victor",
    "w"=> "whiskey",
    "x"=> "xray",
    "y"=> "yankee",
    "z"=> "zulu",
};

/* 
static DIGETS: phf::Map<, &'static str> = phf_map! {
    0 => "zero",
    1 => "one",
};

lazy_static!{
    static ref MAP: HashMap<char, &'static str> = [
    ("a", "alpha"),
    ("b", "bravo"),
    ("c", "charlie"),
    ("d", "delta"),
    ("e", "echo"),
    ("f", "foxtrot"),
    ("g", "golf"),
    ("h", "hotel"),
    ("i", "india"),
    ("j", "juliet"),
    ("k", "kilo"),
    ("l", "lima"),
    ("m", "mike"),
    ("n", "november"),
    ("o", "oscar"),
    ("p","papa" ),
    ("q", "quebec"),
    ("r", "romeo"),
    ("s", "sierra"),
    ("t", "tango"),
    ("u", "uniform"),
    ("v", "victor"),
    ("w", "whiskey"),
    ("x", "xray"),
    ("y", "yankee"),
    ("z", "zulu"),
    ].iter().copied().collect();
}
*/