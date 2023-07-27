//! Smart pointers are any type that implements DeRef and Drop
//! Box<T> has a known size and point to data allocated on the heap
//! Rc<T>/Weak<T> keeps track of the number of references, allowing for shared ownership
//! RefCell<T> provides interior mutablilty to an otherwise immutable reference

use std::cell::RefCell;
use std::rc::{Rc, Weak};

mod custom_smart_pointer;
mod limit_tracker;
mod list;
mod my_box;
mod tree_graph;

use custom_smart_pointer::CustomSmartPointer;
use list::CycleList::{Cons as CycleCons, Nil as CycleNil};
use list::List::{Cons, Nil};
use list::RefCellRefCountList::{Cons as RefCellRcCons, Nil as RefCellRcNil};
use list::RefCountList::{Cons as RcCons, Nil as RcNil};
use my_box::MyBox;
use tree_graph::Node;

// Runs the code for Chapter 15
pub fn run() {
    boxed();
    cons_list();
    check_references();
    check_boxed_references();
    whats_my_box();
    deref_coercions();
    smart_pointer();
    rc_cons_list();
    refcell_rc_cons_list();
    reference_cycles();
    nodes();
}

// Simple example of Box<T>
fn boxed() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Simple implementation of a cons list
fn cons_list() {
    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    println!("{:?}", list);
}

// Example to show how references work in general
fn check_references() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("x:{}, y:{}", x, y);
}

// Example to show how Box values differ from normal references (and how they don't)
fn check_boxed_references() {
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("x:{}, y:{}", x, y);
}

// Example of implementing Deref on a custom type
fn whats_my_box() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("x:{}, y:{:?}", x, y);
}

// Say hello
fn hello(name: &str) {
    println!("Hello, {}", name);
}

// Example showing how derefs are coerced by Rust
fn deref_coercions() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

// Example to show pointer lifetimes
fn smart_pointer() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("\nCustomSmartPointers created: c: {:?}, d: {:?}", c, d);
    drop(c);
    println!("CustomSmartPointer dropped before the end of this function");
}

// Example showing how Rc<T> can be used to share ownership safely
fn rc_cons_list() {
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("\ncount after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("a:{:?}, b:{:?}", a, b);
}

// Example showing how RefCell<T> can bue used to provide inner mutability
fn refcell_rc_cons_list() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RefCellRcCons(Rc::clone(&value), Rc::new(RefCellRcNil)));

    let b = RefCellRcCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RefCellRcCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("\na after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// Example showing how to create a memory leak
fn reference_cycles() {
    let a = Rc::new(CycleCons(5, RefCell::new(Rc::new(CycleNil))));

    println!("\na initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(CycleCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail()); // This will overflow the stack since there is a circular reference
}

// Example showing how to avoid a memory leak
fn nodes() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "\nleaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
