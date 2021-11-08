// type A = Box<i32>;

// struct Point {
//     x: A,
//     y: A
// }

fn main() {
    println!("Hello World");


    // let a = Some(Box::new(12));
    // let b = a;
    // let c = a;

    // let mut a = Point { x: Box::new(10), y: Box::new(10) };
    // let b = a.x;
    // a.x = b;

    // let mut a = Point { x: Box::new(10), y: Box::new(10) };
    // let b = &mut a;
    // a.x = Box::new(10);
    // b.y = Box::new(12);

    // let a = Point { x: Box::new(10), y: Box::new(10) };
    // let b = a.x;
    // let c = a;

    // let mut a = Point { x: Box::new(10), y: Box::new(10) };
    // let b = &mut a;
    // let c = a;
    // b.x = Box::new(11);
    
    // let a = Point { x: Box::new(10), y: Box::new(10) };
    // let b = a.x;
    // let c = &a;

    // let a = Point { x: Box::new(10), y: Box::new(10) };
    // let b = &a;
    // let c = a.x;
    // if b.x == Box::new(10) {
    //     ()
    // }

    // let s = String::from("Hello");
    // let a = Box::new(10);
    // let b;
    // {
    //     if s == String::from("Hello1") {
    //         b = a;
    //     }
    // }
    // let c = a;

    // let mut a = Point { x: Box::new(10), y: Box::new(10) };
    // let b: &Point = &a;
    // a.x = Box::new(11);
    // let c = b;

    // let a = Point { x: Box::new(10), y: Box::new(10) };
    // {
    //     let b = a;
    // }
    // let c = a;

    // let a = Point { x: Box::new(10), y: Box::new(10) };
    // let b = &a;
    // let c = a;

    // This code compiles
    // let mut a = Point { x: Box::new(10), y: Box::new(10) };

    // let b = &mut a;
    
    // let c = &mut *b;

    // let d = &mut *c;

    // // let e = c;
    // let e = &c;

    // // let e = &mut *c;

    // // e.x = Box::new(8);

    // d.x = Box::new(9);
    
    // // c.x = Box::new(11);
    // // c.x = Box::new(13);
    
    // b.x = Box::new(12);
    // b.x = Box::new(14);

    // a.x = Box::new(12);


    // let mut a = Point { x: Box::new(10), y: Box::new(10) };
    // let b = a.x;
    // a.x = Box::new(11);
    // let c = b;

// let a = Box::new(10);
// let b = a; // The ownership of Box::new(10) belongs to b
// let c = a; // Results in ownership error

// let a = (Box::new(10), Box::new(20));
// let b = a.0; // The ownership of Box::new(10) belongs to b
// let d = a.0; // Ownership error: Box::new(10) does not belong to a anymore
// // Now: The ownership of Box::new(20) still belongs to a
// let c = a.1; // Thus, this change of ownership from a.1 to c is valid

// let mut a = (Box::new(10), Box::new(20));
// let b = a.0; // The ownership of Box::new(10) belongs to b
// a.0 = Box::new(30); // The ownership of Box::new(30) belongs to a.0
// let c = a.0; // Thus, this change of ownership from a.0 to c is valid

// let mut a = 10;
// let b = &mut a;

// let mut a = Box::new(10);
// let b = &mut a;
// let c = a;

// let mut a = Box::new(10); // The ownership of Box::new(10) belongs to a
// let b = &mut a; // The ownership of "&mut a" belongs to b

// let mut a = Box::new(10);
// let b = &mut a; // &mut Box<i32> belongs to b
// let c = b;      // &mut Box<i32> belongs to c

// let mut a = (Box::new(10), Box::new(20));
// let b = a.0; // Ownership of Box::new(10) belongs to b
// let c = &mut a.0; // Ownership error: Box::new(10) does not belong to a.0
// // a.0 cannot borrow Box::new(10) to c
// let d = &mut a.1; // Ownership of Box::new(20) belongs to a.1
// // a.1 borrows Box::new(20) to d
// let e = &mut a.1; // a.1 stop borrow Box::new(20) to d
// // a.1 borrows Box::new(20) to e

// let mut a = Box::new(10);
// let mut b = &mut a;
// let c = &mut b;

// // *a = 20;
// // **b = 30;
// ***c = 40;

// let mut a = Box::new(10);
// let mut b = &mut a; // &mut a belongs to b
// let c = &mut b; // &mut b belongs to c
// *b = Box::new(30);
// **c = Box::new(20); // a = Box


// let mut a = Box::new(10);
// let mut b = &mut a; // &mut a belongs to b
// let c = &mut b; // &mut b belongs to c
// *b = Box::new(10);
// let d = c;
// **c = Box::new(10); // Dereference twice and mutate
// *b = Box::new(10); // Dereference once and mutate
// // Note that swapping the last two lines result in borrow error

// let mut a = Box::new(10);
// let b = &mut a;
// let c = &mut *b;
// let d = &mut *c;
// // a = Box::new(10);
// *d = Box::new(20);
// *c = Box::new(30);
// *b = Box::new(40);

// let mut a = 10;
// let b = &mut a;
// {
//     let c = &mut a; // Borrow error: Cannot borrow more than once
//     *c = 20;
// }
// *b = 30;

// add_2(a);
// // Is transformed by the compiler to mean
// add_2(&mut *a);

// let a = Box::new(10);
// let b = &a;
// let c = &a;

// let mut a = (Box::new(10), Box::new(20));
// let b = &mut a;
// let c = b.0; // Ownership error
// b.0 = Box::new(0);

// using std::mem::take;
// let mut a = (Box::new(10), Box::new(20));
// let b = &mut a;
// let c = take(&mut b.0);

// let a = Box::new(10);
// let b = &a; // b is an immutable reference of Box::new(10)
// let mut c = a; // a changes ownership to c is valid
// c = Box::new(20); // c mutates data, invalidating b
// let d = b; // Borrow error: b is invalidated

// let a = (10, 20);
// let b = a.1;
// drop(a);
// println!("b: {}", b);

// let mut a = (10, 20);
// let b = take(&mut a.1);
// drop(a);
// println!("b: {}", b);

// let a = Box::new(10);
// let b = a.deref();
// let c = a.as_ref();

// for i in 0..5 {
//     println!("NUM: {}", i);
// }

}

// fn main() {
//     let mut x = 10;
//     let a = &mut x;
//     add_2(a); // Mutable reference gets copied in?
//     *a += 3; // Mutates some more
//     println!("Answer: {}", x);
// }

// fn add_2(x: &mut i32) {
//     *x += 1; // Mutates using local mutable reference
//     add_1(x);
// }

// fn add_1(x: &mut i32) {
//     *x += 1; // Mutates using local mutable reference
// }