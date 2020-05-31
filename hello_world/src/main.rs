#![allow(ellipsis_inclusive_range_patterns)]
#![allow(non_upper_case_globals)]
#![allow(overlapping_patterns)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::mem;

extern crate rand;
use rand::Rng;

extern crate phrases;
use phrases::greetings::english;
use phrases::greetings::french;

static mut GLOBALVAR:isize = 0;

fn datatypes_and_mutability() {

    let a:u8 = 123;           // 8 bits unsigned, values from 0...255.  Immutable.  Cannot be changed.
    println!("a = {}", a);

    let mut b:i8 = 42;        // b-bits signed, values from -127 to 128. Mutable. Can be changed.
    println!("b = {}", b);
    b = 23;
    println!("b = {}", b);
    b += 1;
    println!("b = {}", b);

    let c = 1234567890;   // let compiler figure out size.  Signed by default.
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    // i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 123;        // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS",
      z, size_of_z, size_of_z * 8);

    // let d:char = 'x';
    let d = 'x';  // same
    println!("d = {}, takes up {} bytes", d, mem::size_of_val(&d));

    //let e:f64 = 2.5;  // double-precision, 8 bytes or 64 bits, type is f64 
    let e = 2.5;        // same
    println!("e = {}, takes up {} bytes", e, mem::size_of_val(&e));

    // true false
    let g = false;
    println!("g = {}, takes up {} bytes", g, mem::size_of_val(&g));

    // threads are automatically safe from mutating a writable value accessible by more than 
    // one potential thread unless you explicitly state that you are doing it on purpose:
    unsafe {
        println!("GLOBALVAR = {}", GLOBALVAR);
    }
}


/* Creating on the heap, looping
*/
fn heap_and_looping() {
    struct Point {
        x: f64,
        y: f64
    }

    fn origin() -> Point {
        Point{x:0.0, y:0.0}
    }

    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("Point({},{})", p3.x, p3.y);

    // while
    let mut x = 1;
    while x <= 1024 {
        println!("{}", x);
        x <<= 1;
    }

    // loop
    loop {
        println!("{}", x);
        x >>= 1;
        if x <= 0 { break; }
    }

    // for
    for (index, y) in (30..41).enumerate() {
        println!("index: {}, y: {}", index, y);
    }
}


/* Match statement
 *
 */
fn match_statement() {
    // assume range should only be 0 - 999
    let mut country_code:usize;

    country_code = if true {44} else {999};
    country_code += if true {0} else {1};

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1...1000 => "unknown",   // 1...999 (deprecated) 1 999
        _ => "invalid",
    };

    println!("{}", country);

    // if and match are expressions and can yield values
}


fn structures() {
    struct Point {
        x: f64,
        y: f64
    }

    struct Line {
        start: Point,
        end: Point
    }

    let p1: Point = Point{x: 3.0, y: 4.0};
    let p2: Point = Point{x: 5.0, y: 6.0};

    let myline = Line{start: p1, end: p2};
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), // tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8},     // struct
}

fn enums() {

    let mut c: Color = Color::Red;
    c = Color::RgbColor(0,0,0);
    c = Color::RgbColor(1,2,3);
    c = Color::CmykColor{cyan:1,magenta:2,yellow:3,black:255};

    match c {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::RgbColor(0,0,0) | Color::CmykColor{cyan:_,magenta:_,yellow:_,black:255} 
            => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r, g, b),
        _ => ()
    };
}

fn options() {

    // Option<T>
    let x = 3.0;
    let y = 2.0;

    // Some(z) None

    let result:Option<f64> = if y != 0.0 { Some(x/y) } else { None };

    println!("{:?}", result);

    match result {
        Some(z) => println!("{} / {} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y),
    };

    if let Some(z) = result { println!("{}", z) }
}


fn arrays() {

    let mut a: [i32;5] = [1,2,3,4,5];

    println!("a has {} elements and the first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("a[0] = {}", a[0]);

    // debug format output
    println!("{:?}", a);

    if a == [321,2,3,4,5] {
        println!("match");
    }

    let b = [1u64; 10];    // b.len = 10
    println!("{:?}", b);

    println!("size of b = {}", mem::size_of_val(&b));

    let board: [[f32;3];2] = [[0.0, 0.1, 0.2], [1.0, 1.1, 1.2]];
    println!("{:?}", board);

    let board = [[0.0;3];2];
    println!("{:?}", board);

    let board = [[0.0, 0.1, 0.2], [1.0, 1.1, 1.2]];
    println!("{:?}", board);
}


fn vectors() {

    let mut a = Vec::new();

    a.push(1);
    a.push(1);
    a.push(1);
    println!("{:?}", a);

    a.push(44);
    println!("{:?}", a);

    // usize isize

    let idx:usize = 0;  // can't address memory by anything other than native machine pointer size and not signed
    println!("{:?}", a[idx]);

    // if we try to index out of bounds we crash
    // so we use get() which returns an Option
    match a.get(6) {
        Some(z) => println!("a[6] = {}", z),
        None => println!("a[6] is out of bounds")
    };
    a.push(3);
    a.push(4);
    a.push(5);
    match a.get(6) {
        Some(z) => println!("a[6] = {}", z),
        None => println!("a[6] is out of bounds")
    };

    // use & to go through the elements in a
    for x in &a { println!("{}", x); }

    let elem = a.pop();
    println!("a.pop() = {:?}, a = {:?}", elem, a);

    while let Some(x) = a.pop() {
        println!("{:?}", x);
    }
    println!("{:?}", a);

    if let Some(x) = a.pop() { println!("x = {:?}", x) };   // is not executed since a is empty
}


fn use_slice(slice: &mut [i32]) {
    println!("first elemen = {}, len = {}", slice[0], slice.len());
    slice[1] = 90;
}
fn slices() {

    let mut data:[i32;7] = [1,2,3,4,5,6,7];

    use_slice(&mut data[3..5]);
    println!("data = {:?}, len = {}", data, data.len());
}

fn strings() {
    
    // utf-8
    let s: &'static str = "hello there!";   // &str = string slice
    // s = "abc";
    // let h = s[0];    can't do these

    for c in s.chars().rev() {
        println!("{}", c);
    }
    // only way to get to chars in static string:
    if let Some(c) = s.chars().nth(0) { println!("{}", c); }

    // Totally different type of string - used on heap
    // String

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        if (a as char) != 'z' {
            letters.push_str(",");
            letters.push(' ');
        }
        a += 1;
    }
    println!("{:?}", letters);

    // &str and String
    // this is legal:
    let u:&str = &letters;


    // concatentation
    // String + str   - ok
    // let z = letters + &letters;

    // String from &str
    let mut abc = String::from("hello world");
    // -or-
    let mut abc = "hello world".to_string();

    abc.remove(0);
    abc.push_str("!!");
    println!("{}", abc.replace("ello", "goodbye"));

    struct Test {
        x:f64,
        y:f64
    }

    let mut a = Test{ x: 1.0, y: 2.0 };
}



fn sum_and_product(x:i32, y:i32) -> (i32,i32) {
    return (x+y,x*y);
}

fn tuples() {

    let x = 3;
    let y = 4;
    let tuple = sum_and_product(x,y);

    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, tuple.0, tuple.1);

    // or 'destructuring':
    let (a,b) = tuple;
    println!("{}, {}", a, b);

    // since a single element in parenthesis would be interpreted as an expression,
    // use a trailing comma in order to create a tuple with one element:
    let a = (1,);
}



fn how_many_oranges(x:i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9...12 => "lots of",    // named range
        _ if (x%2==0) => {"an even number of"},
        _ => "a few"
    }
}


fn match_point(mut point:(i32,i32)) {
    match point {
        (0,0) => println!("origin"),
        
        (0,y) => println!("x axis, y = {}", y),                         // y made on-the-fly

        (ref mut x,0) => println!("y axis, x = {} (forced)", x = 1),    // x made on-the-fly and is a mutable reference instead of a value
        
        (_,y) => println!("(?,{})", y),             // same as _ in this case since any two values will match this tuple
        //_ => ()
    };
}

fn pattern_matching() {

    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many_oranges(x));
    }

    match_point((3,4));
    match_point((0,4));
    match_point((3,0));

    enum Color {
        Red,
        Green,
        Blue,
        RgbColor(u8,u8,u8), // tuple
        CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8},     // struct
    }

    let mut c: Color = Color::CmykColor{cyan:1, magenta:2, yellow:3, black:255};

    match c {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),

        // Color::RgbColor(0,0,0) | Color::CmykColor{cyan:_,magenta:_,yellow:_,black:255} => println!("black")
        // The line above can be put this way: (meaning we care about nothing but the black:
        // parameter)
        Color::RgbColor(0,0,0) | Color::CmykColor{black:255,..} => println!("black"),
        
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r, g, b),
        _ => ()
    };
}


fn generics() {

    struct Point<T,V> {
        x:T,
        y:V
    }

    let a = Point { x: 0, y: 0 };
    let a: Point<f64,f64> = Point { x: 0.0, y: 0.0 };
    let a: Point<&'static str, &'static str> = Point { x: "hello", y: "world" };
    let a: Point<&'static str,String> = Point { x: "hello", y: "world".to_string() };
    let a = Point{x: 1.0, y: 2.0};
    let a = Point{x: 1, y: "hello world"};

    struct Line<T> {
        start: Point<T,T>,
        end: Point<T,T>
    }


    let a = Point{x: 1, y: 2};
    let b = Point{x: 3, y: 4};
    let c = Line{ start: a, end: b};

    let a = Line{start: Point{ x: "hello", y: "world" }, end: Point{ x: "some", y: "string" } };
}



// 'functions' are functions defined at the 'mod' level.
// That is to say, "not inside a class struct tuple, or enum",
// or "not a method"
fn functions() {
    fn func<T>(x: T) -> T {
        return x;
    }

    fn increase(x: &mut i32) {
        *x += 1;
    }

    fn product(x: i32, y:i32) -> i32 {
        x * y
    }

    let a = func(1);
    let a = func(1.0);
    let a = func("hello");

    let ref a = (1,2);
    println!( "{:?}", a);
    let ref a = &(1,2);
    println!( "{:?}", a);

    let mut z = 1;
    increase(&mut z);
    println!( "z = {:?}", z);

    let a = 3;
    let b = 4;
    let p = product(a,b);
    println!( "p = {:?}", p);
}


fn methods() {
    
    struct Point<T,V> {
        x:T,
        y:V
    }

    struct Line<T> {
        start: Point<T,T>,
        end: Point<T,T>
    }

    impl Line<f64> {
        fn len(&self) -> f64 {
           let dx = self.start.x - self.end.x; 
           let dy = self.start.y - self.end.y; 

           (dx*dx+dy*dy).sqrt()
        }
    }

    let p1 = Point{ x: 1.1, y: 2.2 };
    let p2 = Point{ x: 3.3, y: 4.4 };

    let a = Line{start: p1, end: p2};
    let b = a.len();
    println!("distance from Point({}, {}) to Point({}, {}) is {}",
        a.start.x, a.start.y, a.end.x, a.end.y, b);
}



// neither functions or methods,
// closures are stand alone (like lambdas I think* - yes they are)
//
fn closures() {

    fn say_hello() { println!("hello"); }
    let a = say_hello;
    a();

    let plus_one = |x: i32| -> i32 { x + 1 };
    let a = 1;
    let b = plus_one(a);
    println!("{} + 1 = {:?}", a, b);

    let two = 2;
    let plus_two = |x|
    {
        let mut z = x;
        z += two;
        z
    };

    println!("{} + 2 = {}", 3, plus_two(3));

    // Borrowing
    // if we use the above statement but use a variable instead of a &'static i32:
    // let borrow_two = &mut two;

    // we would get an error because the code above 'borrowed' and owns the variable two
    // in it's context.  We need to make it release it's reference.  So if we had the same code
    // except inside a pair of braces it would be released:

    let mut two = 2;
    {
        let plus_two = |x|
        {
            let mut z = x;
            z += two;
            z
        };
    
        println!("{} + 2 = {}", two, plus_two(two));
    }

    // now it is ok to use two
    let borrow_two = &mut two;
    println!("borrow_two = {}", borrow_two);

    // we can do better by taking a reference instead so it's value isn't captured by reference:
    //
    // T: by value
    // T&
    // &mut &
    let plus_three = |x:&mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("{} + 3 = {}", 12, f);
}



fn higher_order_functions() {

    let is_even = |ref x| { (x % 2) == 0 };

    let limit = 500;
    let mut sum = 0;

    for i in 0.. {  // NOTE: no end of range
        let isq = i*i;

        if isq > limit { break; } 
        else if is_even(isq) { sum += isq; }
    }
    println!("sum = {}", sum);

    // now do all that in a single line!:

    let sum2 = (0..)
        .map(|x| x*x)
        .filter(|x| is_even(*x))
        .take_while(|&x| x < limit)
        .fold(0, |sum, x| sum + x);

    println!("sum2 = {:?}", sum2);
}


fn traits() {

    trait Animal {
        fn create(name: &'static str) -> Self;
        fn name(&self) -> &'static str;

        fn talk(&self) {
            println!("{} cannot talk", self.name());
        }
    }

    struct Human {
        name: &'static str
    }
    impl Animal for Human {
        fn create(name: &'static str) -> Human {
            Human{name: name}
        }
        fn name(&self) -> &'static str { self.name }
        fn talk(&self) { println!("{} says hello", self.name); }
    }

    struct Cat {
        name: &'static str
    }
    impl Animal for Cat {
        fn create(name: &'static str) -> Cat {
            Cat{name: name}
        }
        fn name(&self) -> &'static str { self.name }
        fn talk(&self) { println!("{} says meow", self.name); }
    }

    //let h = Human{name: "John"};
    let h = Human::create("John");
    h.talk();

    //let c = Cat{name: "Tom"};
    let c = Cat::create("Tom");
    c.talk();

    let z: Human = Animal::create("Jane");
    z.talk();


    // You can add traits and behavior to structs you don't even
    // own or have the code
    let a = vec![1,2,3];

    // we want to run: 
    //   let s = a.sum();
    //
    //   but Vec::sum() -> i32 doesn't exist so we make it:
    trait Summable<T> {
        fn sum(&self) -> T;
    }

    impl Summable<i32> for Vec<i32> {
        fn sum(&self) -> i32 {
            let mut result:i32 = 0;
            for x in self { result += *x; }
            return result;
        }
    }

    let s = a.sum();
    println!("a.sum() = {}", s);
}


fn lifetime() {

    let v = vec![1, 2, 3];  

    // the variable v 'owns' the memory it is bound to.
    // the v structure itself lives on the stack.
    // now if we say:
    let v2 = v;
    // that DID NOT copy the value of v, it copied a pointer to it.
    // the same single values 1, 2, and 3 are still on the heap but
    // two stack variables now point to them.
    // 
    // Rust sees and figures out that these two pointers create a potential race condition!
    // This violates Rust's safety guarantee so it chooses which pointer is bound to the resource.
    // 
    // Right now as the code is, the code:
    //
    //   println!("{:?}", v);
    //
    //   would give a compile error:
    //   "borrow of moved value 'v'

//  println!("{:?}", v);
    
    // when v2 was assigned the pointer value of v, v was invalidated and marked as 'moved'.
    //
    // This is okay:
    let u = 42;
    let u2 = u;     // the value is copied

    // This is not okay:
    let u = Box::new(42);   // i32 on heap
    let u2 = u;

    // println!("{}", *u);  // would not compile for same reason as above:
    //
    // "borrow of moved value 'u'
    // to make your own types get copied by value like primitives
    // you need to implement the trait 'Copy'
    //

    struct Point<T> {
        x: T,
        y: T
    }
    let p1 = Point{x: 0.0, y: 0.0};
    let p2 = p1;
    //println!("Point({}, {})", p1.x, p1.y);

    // this is okay:

    let v = vec![1, 2, 3];
    let foo = |x:Vec<i32>| -> Vec<i32> {
        x
    };
    let v = foo(v);
    println!("{:?}", v);

    // Still, having to return the value and have a let v = ... 
    // is tedious.  
    // So we have Borrowing which basically means pass by reference only
    // and control ownership using scoping and never let two mutable references
    // to the same resource exist at the same time.

    let mut z = vec![3, 2, 1];
    for i in &mut z {
        println!("{}", i);
        // z.push(5);       // Not allowed because our controlling loop would never see the end of
                            // the vector!  A race condition which Rust sees and will not allow to
                            // even compile.
    }
}


/**
 * https://crates.io
 *
 * Like cpan.org for perl or pipi.org for python
 * Rust calls the packages 'Crates'.  Crates are made of modules.
 *
 * Example Cargo.toml for our project:
 *
 * [package]
 * name = "odds_ends"
 * version = "0.1.0"
 * author = ["Trent Wyatt trent@trentwyatt.com"]
 *
 * [dependencies]
 * rand = "0.3.12"                      // the package from crates.io we need in our package
 * phrases = { path = "../phrases" }    // include our own crate also
 * 
 * Now at the command line we execute:
 *
 *   $ cargo build
 *
 * and this will compile our odss_ends crate.
 * or do: 
 *
 * cargo run
 *
 * which will build and then run
 *
 */

fn random() {
    let mut rng = rand::thread_rng();
    let mut v:Vec<u32> = vec![];

    println!();

    for _ in 0..11 {
        let mut r: u32 = rng.gen();
        r %= 100;
        v.push(r);
    }

    println!("{:?}", v);
}


fn main() {

    //random();

    println!();

    println!("English: {} {}", english::hello(), english::goodbye());
    println!(" French: {} {}",  french::hello(),  french::goodbye());
}








