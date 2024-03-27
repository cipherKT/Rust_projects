use rand::seq::SliceRandom;
use clipboard::{ClipboardContext, ClipboardProvider};

fn generate_password(length: usize, lowercase: Vec<char>, uppercase: Vec<char>, digits: Vec<char>, special: Vec<char>) -> String {
    // This is the password that will be returned
    let mut password = String::with_capacity(length);

    let mut rng = rand::thread_rng();
    password.push(*uppercase.choose(&mut rng).unwrap_or(&'A'));
    password.push(*digits.choose(&mut rng).unwrap_or(&'0'));
    password.push(*special.choose(&mut rng).unwrap_or(&'!'));

    let remaining_len = length - password.len();
    let combined_chars: Vec<char> = lowercase.iter()
        .cloned()
        .chain(uppercase.clone())
        .chain(digits.clone())
        .chain(special.clone())
        .collect();

    let remaining_chars: Vec<char> = combined_chars.choose_multiple(&mut rng, remaining_len).cloned().collect();
    password.push_str(&remaining_chars.iter().collect::<String>());

    let mut password_chars: Vec<char> = password.chars().collect();
    password_chars.shuffle(&mut rng);

    password_chars.iter().collect()
}

fn main() {
    let length = 16;
    let lowercase: Vec<char> = (b'a'..=b'z').map(|c| c as char).collect();
    let uppercase: Vec<char> = (b'A'..=b'Z').map(|c| c as char).collect();
    let digits: Vec<char> = (b'0'..=b'9').map(|c| c as char).collect();
    let special: Vec<char> = (b'!'..=b'/').chain(b':'..=b'@').chain(b'['..=b'`').chain(b'{'..=b'~').map(|c| c as char).collect();

    let password = generate_password(length, lowercase, uppercase, digits, special);
    println!("Generated password: {}", password);


    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(String::from(password)).unwrap();
    println!("Password copied to clipboard!");
}
