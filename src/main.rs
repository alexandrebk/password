use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use cli_clipboard;

fn main() {
    let mut rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let borrowed_string: &str = "!";
    rand_string.push_str(borrowed_string);

    cli_clipboard::set_contents(rand_string.to_owned()).unwrap();

    println!("A new password was succesfully copy to your clipboard");
    println!("{}", rand_string);
}
