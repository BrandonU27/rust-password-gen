pub struct PasswordGenerator {
    upperLetters: Vec<String>,
    lowerLetters: Vec<String>,
    numbers: Vec<String>,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        PasswordGenerator {
            upperLetters: vec! [
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "E".to_string(),
                "F".to_string(),
                "G".to_string(),
                "H".to_string(),
                "I".to_string(),
                "J".to_string(),
                "K".to_string(),
                "L".to_string(),
                "M".to_string(),
                "N".to_string(),
                "O".to_string(),
                "P".to_string(),
                "Q".to_string(),
                "R".to_string(),
                "S".to_string(),
                "T".to_string(),
                "U".to_string(),
                "V".to_string(),
                "W".to_string(),
                "X".to_string(),
                "Y".to_string(),
                "Z".to_string(),
            ],
            lowerLetters: vec! [
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
                "f".to_string(),
                "g".to_string(),
                "h".to_string(),
                "i".to_string(),
                "j".to_string(),
                "k".to_string(),
                "l".to_string(),
                "m".to_string(),
                "n".to_string(),
                "o".to_string(),
                "p".to_string(),
                "q".to_string(),
                "r".to_string(),
                "s".to_string(),
                "t".to_string(),
                "u".to_string(),
                "v".to_string(),
                "w".to_string(),
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
            ],
            numbers: vec! [
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
            ],
        }
    }

    pub fn print_vecs(&self) {
        println!("Upper: {:?}", self.upperLetters);
        println!("Lower: {:?}", self.lowerLetters);
        println!("Numbers: {:?}", self.numbers);
    }
}
