//
// COMMENTS ARE A WORK IN PROGRESS
//
/*
 * Password Logic Object
 *
 * Description:
 *  The 'PasswordGenerator' class is designed to generate random passwords composed of uppercase
 *  letters, lowercase letters, numeric characters, and symbols. This class provides methods to customize
 *  the password length and generate the actual password.
 */
use rand::Rng;

/*
 * Structure
 *
 * Fields:
 *  'password_len: usize': Specifies the length of the password
 *  'upper_letters: Vec<char>': Stores a vector of uppercase letters.
 *  'lower_letters: Vec<char>': Stores a vector of lowercase letters.
 *  'numbers: Vec<char>': Stores a vector of numeric characters.
 *  'symbols: Vec<char>': Stores a vector of symbols.
 *  'include_upper: bool': Flag to include uppercase letters.
 *  'include_lower: bool': Flag to include lowercase letters.
 *  'include_numbers: bool': Flag to include numeric characters.
 *  'include_symbols: bool': Flag to include symbols.
 */
pub struct PasswordGenerator {
    password_len: usize,
    upper_letters: Vec<char>,
    lower_letters: Vec<char>,
    numbers: Vec<char>,
    symbols: Vec<char>,
    include_upper: bool,
    include_lower: bool,
    include_numbers: bool,
    include_symbols: bool,
}

impl PasswordGenerator {
    /*
    * Constructor
    *
    * Output:
    *  New PasswordGenerator Object with a default password length.
    *
    * Comment:
    *  The default password length is set to 20.
    *  Uppercase letters, lowercase letters, and numeric characters are collected into their
    *  respective vectors.
    */
    pub fn new() -> Self {
        PasswordGenerator {
            password_len: 20,
            upper_letters: ('A'..='Z').collect(),
            lower_letters: ('a'..='z').collect(),
            numbers: ('0'..='9').collect(),
            symbols: vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+'],
            include_upper: true,
            include_lower: true,
            include_numbers: true,
            include_symbols: true,
        }
    }

    /*
     * set_password_length()
     *
     * Input:
     *  'len: usize': New password length.
     * 
     * Comment:
     *  This method allows the user to set a custom password length by updating the 'password_len'
     *  field.
     */
    pub fn set_password_length(&mut self, len: usize) {
        self.password_len = len;
    }

    /*
     * set_xxxx_flag()
     * 
     * Input:
     *  'flag: bool': Flag state.
     * 
     * Comment:
     *  This method enables users to configure the current state of each flag, controlling the character sets used to generate passwords.
     *  Specifically, it determines the inclusion of characters, numbers, and symbols in teh generated password.
     */
    pub fn set_upper_flag(&mut self, flag: bool) {
        self.include_upper = flag;
    }
    pub fn set_lower_flag(&mut self, flag: bool) {
        self.include_lower = flag;
    }
    pub fn set_numbers_flag(&mut self, flag: bool) {
        self.include_numbers = flag;
    }
    pub fn set_symbols_flag(&mut self, flag: bool) {
        self.include_symbols = flag;
    }

    /*
     * generate_password()
     *
     * Input:
     *  None
     *
     * Output:
     *  String: User generated password.
     *
     * Comment:
     *  This method generates a random password.
     *      - It combines all possible characters into a single vector.
     *      - A thread-local random number generator is used to select random characters from the
     *      combined vector.
     *      - The selected characters are collect into a String to form the password.
     */
    pub fn generate_password(&self) -> String {
        // Create a vector containing all possible characters for the password.
        // Checks to see which vecotors to include in the all_characters vector.
        let mut all_characters: Vec<char> = Vec::new();
        if self.include_upper {
            all_characters.extend(&self.upper_letters);
        }
        if self.include_lower {
            all_characters.extend(&self.lower_letters);
        }
        if self.include_numbers {
            all_characters.extend(&self.numbers);
        }
        if self.include_symbols {
            all_characters.extend(&self.symbols);
        }

        // Create a thread-local random number generator.
        let mut rng = rand::thread_rng();

        // Generate the password by randomly selecting characters
        (0..self.password_len)
            .map(|_| all_characters[rng.gen_range(0..all_characters.len())])
            .collect()
    }

    pub fn print_vecs(&self) {
        println!("Upper: {:?}", self.upper_letters);
        println!("Lower: {:?}", self.lower_letters);
        println!("Numbers: {:?}", self.numbers);
        println!("Symbols: {:?}", self.symbols);
    }

    pub fn print_password_length(&self) {
        println!("Password Length: {}", self.password_len);
    }
}

/*
 * Unit Tests
 * WORK IN PROGRESS
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_default_generator() {
        let generator = PasswordGenerator::new();

        assert_eq!(generator.password_len, 20);
        assert_eq!(generator.include_upper, true);
        assert_eq!(generator.include_lower, true);
        assert_eq!(generator.include_numbers, true);
        assert_eq!(generator.include_symbols, true);
    }

    #[test]
    fn test_generate_password() {
        let generator = PasswordGenerator::new();
        let password = generator.generate_password();

        assert!(!password.is_empty(), "Generated password should not be NULL");
        assert_eq!(password.len(), 20);
    }

    #[test]
    fn test_flag_setting() {
        let mut generator = PasswordGenerator::new();
        generator.set_upper_flag(false);
        generator.set_lower_flag(true);
        generator.set_numbers_flag(false);

        assert_eq!(generator.include_upper, false);
        assert_eq!(generator.include_lower, true);
        assert_eq!(generator.include_numbers, false);
    }
}