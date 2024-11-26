use rules::SYSY;

mod rules;
mod symbols;

pub fn show() {
    let sysy = SYSY.lock().unwrap();
    println!("{}", sysy);
}
