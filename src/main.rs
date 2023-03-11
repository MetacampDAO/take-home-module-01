// Define a User struct which contains 2 fields:
struct User {
    // name (string e.g "John")
    name: String,
    // balance (tuple e.g (100.00, "SGD"))
    balance: (f32, String),
}

// Define a User method (using impl)
impl User {
    // called print_user_detail, which simply prints the name, balance and currency of the user.
    fn print_user_detail(&self) {
        println!(
            "{} has balance of {} {}",
            self.name, self.balance.0, self.balance.1
        );
    }
}

// Define an accrue_interest function, which takes in a user and interest percentage as 2 separate parameters.
fn accrue_interest(mut user: User, interest: f32) {
    // Within the function, increase the users' balance by the interest percentage, and print out the user details by calling the print_user_detail method.
    user.balance.0 = user.balance.0 + (user.balance.0 * interest / 100.0);
    user.print_user_detail();
}

fn main() {
    // In the main function, create a user variable of type User, populating the field values of name, and balance and currency.
    let user = User {
        name: "John".to_owned(),
        balance: (100.0, "SGD".to_owned()),
    };

    // Then, call the accrue_interest function.
    accrue_interest(user, 10.0);
    
    // Bonus: After the call to accrue_interest, call it multiple times so that the user may benefit from compounding interest.
}
