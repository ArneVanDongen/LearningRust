// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_word = Some(String::from("rustlings"));

    let word;
    if let Some(i) = optional_word {
        println!("Matched word: {:?}!", &i);
        word = i;
    }  else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    while let Some(Some(i)) = optional_integers_vec.pop() {
        println!("current value: {}", i);
    }
}
