pub fn execute() {
    let s = gives_ownership();
    {
        let num_chrs = number_of_chars(&s);
        println!("{} has {} characters", s, num_chrs)
    }
    println!(
        "{}
",
        s
    );
}

fn gives_ownership() -> String {
    String::from("Ownership")
}

fn number_of_chars(some_string: &String) -> usize {
    some_string.len()
}
