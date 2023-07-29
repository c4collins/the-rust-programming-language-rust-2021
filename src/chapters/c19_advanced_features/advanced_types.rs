pub fn run() {
    type_aliases();
    using_type_alias_to_make_long_type_easier_to_manage();
}

#[derive(Debug)]
struct Metre(u32); // Thin wrapper around type
type Kilometre = u32; // Aliasing type name

fn type_aliases() {
    let x: u32 = 5;
    let y: Kilometre = 10;
    let z = y + x;
    println!("{} + {} = {}", x, y, z);

    let w: Metre = Metre(450);
    println!(
        "w is {:?} and can't be added to the others because it's a different type",
        w
    );
}

fn using_type_alias_to_make_long_type_easier_to_manage() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    fn takes_long_type(f: Thunk) {
        f();
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hello"))
    }

    let thing = returns_long_type();
    takes_long_type(thing);
}
