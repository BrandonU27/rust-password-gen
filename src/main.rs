mod password_logic;

use password_logic::PasswordGenerator;

fn main()
{
    let test = PasswordGenerator::new();
    test.print_vecs();
}
