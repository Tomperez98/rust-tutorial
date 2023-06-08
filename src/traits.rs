trait Summarize {
    fn summarize(&self) -> String;
}

struct Article {
    author: String,
    headline: String,
}
struct Tweet {
    username: String,
    content: String,
}
impl Summarize for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
impl Summarize for Tweet {
    fn summarize(&self) -> String {
        format!("{} wrote {}", self.username, self.content)
    }
}

fn notify<T: Summarize>(summarizable: &T) {
    println!("{}", summarizable.summarize());
}

pub fn execute() {
    let art = Article {
        author: "Tomas".to_owned(),
        headline: "Rust is great".to_owned(),
    };
    let tweet = Tweet {
        username: "tomaperez98".to_owned(),
        content: "Rust is great!".to_owned(),
    };

    notify(&art);
    notify(&tweet);
}
