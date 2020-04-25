use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::mem;

struct A {
    i: i32,
    j: i32,
}

impl Display for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // todo!();
        // Result::Ok("(".to_owned() + &self.i.to_string() + &"," + &self.j.to_string() + &")")
        // write!(f,"({},{})",self.i,self.j) // OK
        // write!(f,"({},{})",&self.i,&self.j) // OK
        write!(f, "({},{})", *&self.i, *&self.j) // OK intentional ref deref!
                                                 /*
                                                  *   primitives copy clone,not move
                                                  *   struct self  move
                                                  */
    }
}


fn main() {
    println!("Hello, world!");
    let a = A { i: 100, j: 99 };
    let b = A { j: 88, i: 77 };
    println!("{}", a);
    println!("{}", b);

    let i: fn() -> i32 = move || 10;
    println!("{}", i());

    println!("size of &str: {}", mem::size_of::<&str>());
    println!("size of &[u8]: {}", mem::size_of::<&[u8]>());

    let str_slice = "Hello world";
    let mem: [usize; 2] = unsafe { mem::transmute(str_slice) };
    println!("&str ptr: 0x{:x}, len: {}", mem[0], mem[1]);

    let u8_arr: [u8; 5] = [b'H', b'e', b'l', b'l', b'o'];
    let u8_slice: &[u8] = &u8_arr[..5];
    let mem: [usize; 2] = unsafe { mem::transmute(u8_slice) };
    println!("&[u8] ptr: 0x{:x}, len: {}", mem[0], mem[1]);
    println!("size of String: {}", mem::size_of::<String>());

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    s.push_str("OK");
    println!("{}", s);

    let s2 = String::from("goo");
    s.push_str(&s2);
    println!("{}", s);

    let s3 = String::from("Hello");
    let s5 = String::from("World");

    let s6 = "123".to_owned() + &s3 + &s5;
    println!("{}", s6);
    println!("& :? {:?}", &s6.as_ptr());
    println!(":? {:?}", s6.as_ptr());
    println!("& :p {:p}", &s6.as_ptr());
    println!("*& :p {:p}", *&s6.as_ptr());
    println!(":p {:p}", s6.as_ptr());

    println!("{}", s6.len());

    let p:i32 = s6.as_ptr() as i32;
    println!("{:p}",&p);
    println!("{:x}",p);

    
}
