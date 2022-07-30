// how rust allocate memory in heap and stack 
// how rust allocate and release memory

// one variable one owner
// owner can be trasnferred cannot borrow
// use reference instead

pub fn main(){

    // variable scoping
    // rust automaticly clean the out of scope memory if used heap 
    // {
    //     let mut s = String::from("hello");

    //     s.push_str(", world!"); // push_str() appends a literal to a String
    
    //     println!("{}", s); // This will print `hello, world!`
    // }

    // s2 copy the pointer of the s1 which is String class 
    // let s1 = String::from("hello");
    // let s2 = s1;


    // invalid because s1 already invalidate by rust
    // this will move the pointer of String::from("hello") to s2
    // only one var has one owner
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
    println!("{}", s);

}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.