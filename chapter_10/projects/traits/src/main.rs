pub trait Summary { // If something is a Summary, must have implemented each of the trait functions
    fn summarize(&self) -> String;
}

/*
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
*/

// Implementing a Trait on a Type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Diff struct with same Trait implementation
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
 */


fn main() {
    println!("Hello, Traits!");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // ---- TRAITS AS PARAMETERS -----
    notify(&tweet);
}

// ---- TRAITS AS PARAMETERS -----
pub fn notify(item: &impl Summary) { // Takes item that has implemented Summary Trait
    println!("Breaking news! {}", item.summarize());
}

// Other Syntax implementations
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// Specifying Multiple Trait Bounds with the + Syntax
// pub fn notify(item: &(impl Summary + Display)) { // 2 Traits implementations
// pub fn notify<T: Summary + Display>(item: &T) { // Same with generic types <T>

// Clearer Trait Bounds with where Clauses
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// or
/*  fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
*/

// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary { // Returns a Type that implementes Trait Summary
                                            // can only be used if returns only 1 implemented trait
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
