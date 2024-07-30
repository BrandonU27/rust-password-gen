mod password_logic;

use password_logic::PasswordGenerator;

fn main()
{
    // testing functions
    let mut test = PasswordGenerator::new();
    test.print_vecs();
    test.print_password_length();
    test.set_password_length(10);
    test.print_password_length();
}

