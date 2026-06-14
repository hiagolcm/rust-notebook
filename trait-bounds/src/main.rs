fn duplicate<T: Clone>(item: T) -> (T, T) {
    (item.clone(), item.clone())
}

struct NotClonable {}

fn main() {
    let not_clonable = NotClonable{};
    let my_string = String::from("hey");
    let clones = duplicate(my_string);
    let other_clones = duplicate(not_clonable);
    dbg!(clones);
}
