struct User { // Definition of a Struct
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, Structs!");

   

    // DEFINE STRUCT 
    // Whole instance must be mut if it is wanted to be modified
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1 email is {}", user1.email);

    // CHANGE STRUCT INFO
    user1.email = String::from("test@test.com");
    println!("user1 email is {}", user1.email);

    // CREATE STRUCT USING A FUNCTION
    let user_function = build_user(String::from("testing@testing.com"),String::from( "test_user"));
    println!("user_function email is {} and its username is {}", user_function.email, user_function.username);

    // INSTANCIATE NEW OBJECTS WITH OLD STRUCT INFO 
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user2 email is {} and its username is {}", user2.email, user2.username);

    // STRUCTS WITHOUT NAME FIELDS (ONLY TYPE) "TUPLE_STRUCTS"
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Color first parameter is {}", black.0);
    
    

}   

// CREATE STRUCT USING A FUNCTION
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}



