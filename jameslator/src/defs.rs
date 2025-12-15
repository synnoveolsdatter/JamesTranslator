pub const CONSONANTS: [char; 20] = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'];
pub const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
pub const JCONSANTS: [char; 3] = ['j', 'm', 's'];
pub const JVOWELS: [char; 2] = ['a', 'e'];

#[derive(PartialEq)]
pub enum TranslationType {
    JtoE,
    EtoJ,
    QUIT,
    NONE,// NOTE - NONE is never used since it's just a temporary value to use in velg()
}

// dont ask
pub struct Jimser {
    jc_i: u8,// James Consonant Index
    jv_i: u8,// James Vowel Index
}

impl Jimser {
    pub fn new() -> Jimser {
        Jimser {
            jc_i: 0,
            jv_i: 0
        }
    }

    // converts a char to the current jamesish value held in the struct (gotten by indexing into
    // the jamesish consonants / vowels depending on which it is)
    pub fn char_to_james(&mut self, c: &char) -> String {
        if Jimser::is_consonant(c) != -1 {// must be consonant if it's not -1
            let idx = Jimser::is_consonant(c) as usize;// need to index james consonants later
            let mut ret = String::new();
            for _ in 0..idx {
                ret.push(JCONSANTS[self.jc_i as usize]);
            }
            self.plus_c();
            return ret;
        }
        // we havent returned yet so it must be a vowel
        let idx = Jimser::is_vowel(c) as usize;
        let mut ret = String::new();
        for _ in 0..idx {
            ret.push(JVOWELS[self.jv_i as usize]);
        }
        self.plus_v();
        ret
    }

    // assumes character occurs in the james alphabet,
    // string should only be the *one character* repeating
    pub fn james_to_char(jam: char, length: usize) -> char {
        if Jimser::is_jconsonant(jam) != -1 {// must be a consonant
            return CONSONANTS[length - 1];
        }
        return VOWELS[length - 1]// it's nearly 01:00 as im writing this i really hope it works
    }

    // increment consonant idx, wraps if needed
    pub fn plus_c(&mut self) {
        self.jc_i = (self.jc_i + 1) % 3;// wrap after 's' (index 2)
    }

    // increment vowel idx, wrap if needed (which is often lol)
    pub fn plus_v(&mut self) {
        self.jv_i = (self.jv_i + 1) % 2;
    }

    // housekeeping etc, methods to check js to make it easier down the line
    pub fn is_consonant(c: &char) -> i8 {
        for idx in 0..20 {
            if *c == CONSONANTS[idx] {
                return idx.try_into().unwrap();
            }
        }
        -1
    }

    // literally the same as the previous one but w diff range and variable
    pub fn is_vowel(c: &char) -> i8 {
        for idx in 0..6 {
            if *c == VOWELS[idx] {
                return idx.try_into().unwrap();

            }
        }
        -1
    }

    // quicker than is_consonant bc only uses the 3 consonants,
    // should only be used if char is guaranteed jamish
    pub fn is_jconsonant(c: char) -> i8 {
        for idx in 0..3 {
            if c == JCONSANTS[idx] {
                return idx.try_into().unwrap();
            }
        }
        -1
    }
}
