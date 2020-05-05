use std::fmt::Display;
use std::thread;
use std::borrow::BorrowMut;
struct A {
    i: i32,
    j: i64,
}

impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({},{})", self.i, self.j)
    }
}

trait DI{
    fn f(& mut self)->&mut Self{
        self
    }
    fn g(&self)->&mut Self{
        unsafe {  & mut * ( self as * const Self as * mut Self)}
    }
}

impl DI for A{
    
}

fn main() {
    // println!("Hello, world!");
    println!("{}", A { i: 1, j: 2 });
    let mut a = A { i: 1, j: 2 };
    let b = a.borrow_mut();
    b.i=100;
    println!("{}",b);

    let c = a.borrow_mut();
    println!("{}",c);
    c.i = 88;
    println!("{}",a);
    thread::spawn(move || {
        println!("{}",a);
    }).join().unwrap();
    let mut a = A{i:99,j:99};
    {
        let b = &mut a;
        b.i = 100;
    }
    println!("{}",a);
    {
        let c = &mut a;    
        c.j = 100;
    }
    println!("{}",a);
    
    let d = A{i:987,j:999};
    let g = d.g();
    g.i=777;
    println!("{}",d);

}
