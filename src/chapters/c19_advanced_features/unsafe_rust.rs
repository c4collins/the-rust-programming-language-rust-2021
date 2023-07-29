use std::slice;

pub fn run() {
    dereferenceing_a_raw_pointer();
    unsafe {
        dangerous();
    }
    how_to_use_split_at_mut();
    using_local_split_at_mut();
    foreign_function_interface();
    hello();
    counting();
}

fn dereferenceing_a_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // This is stupid
    let address = 0x012345usize;
    let r = address as *const i32;
}

unsafe fn dangerous() {}

fn how_to_use_split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    println!("{:?} & {:?}", a, b);
}
fn using_local_split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = local_split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    println!("{:?} & {:?}", a, b);
}

fn local_split_at_mut<T>(values: &mut [T], split_index: usize) -> (&mut [T], &mut [T]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(split_index <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, split_index),
            slice::from_raw_parts_mut(ptr.add(split_index), len - split_index),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn foreign_function_interface() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

static HELLO_WORLD: &str = "Hello, world!";

fn hello() {
    println!("Hi? {}", HELLO_WORLD);
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn counting() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Unsafe traits
unsafe trait Foo {}
unsafe impl Foo for i32 {}
