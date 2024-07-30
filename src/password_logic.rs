pub struct PasswordGenerator {
    password_len: usize,
    upper_letters: Vec<char>,
    lower_letters: Vec<char>,
    numbers: Vec<char>,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        PasswordGenerator {
            password_len: 20,
            upper_letters: ('A'..='Z').collect(),
            lower_letters: ('a'..='z').collect(),
            numbers: ('0'..='9').collect(),
        }
    }

    pub fn set_password_length(&mut self, len: usize) {
        self.password_len = len;
    }

    pub fn print_vecs(&self) {
        println!("Upper: {:?}", self.upper_letters);
        println!("Lower: {:?}", self.lower_letters);
        println!("Numbers: {:?}", self.numbers);
    }

    pub fn print_password_length(&self) {
        println!("Password Length: {}", self.password_len);
    }
}
