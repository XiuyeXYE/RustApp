use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::marker::Copy;
use std::marker::Send;
use std::mem;
use std::option::Option;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

//8 byte align : 16
//bbbb bbbb
//bbbb bbbb
//16
#[repr(align(8))]
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

trait AI {
    fn f(&self);
}

impl AI for A {
    fn f(&self) {
        println!("A:AI .f()");
    }
}

// auto trait BI{}

// impl !BI for A{}

fn main() {
    println!("Hello, world!");
    let a = A { i: 100, j: 99 };
    let b = A { j: 88, i: 77 };
    println!("{}", a);
    println!("{}", b);
    a.f();
    b.f();

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

    let p: i32 = s6.as_ptr() as i32;
    println!("{:p}", &p);
    println!("{:x}", p);

    let op = Option::Some(2);
    println!("{:?}", op);

    if let Some(i) = op {
        println!("{}", i);
    }

    println!("Thread run...==================");

    let parked_thread = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("Thread unparked");
        })
        .unwrap();

    // Let some time pass for the thread to be spawned.
    thread::sleep(Duration::from_millis(10));

    println!("Unpark the thread");
    parked_thread.thread().unpark();

    parked_thread.join().unwrap();
    let other_thread = thread::spawn(|| thread::current().id());

    let other_thread_id = other_thread.join().unwrap();
    println!("{:?} {:?}", thread::current().id(), other_thread_id);

    trait Demo {
        fn clone2(&self) -> Self;
    }
    struct B {
        z: i32,
        k: A,
    }
    impl Copy for A {}
    impl Copy for B {}
    impl Clone for A {
        fn clone(&self) -> Self {
            *self
        }

        fn clone_from(&mut self, source: &Self) {
            *self = *source
        }
    }
    impl Clone for B {
        fn clone(&self) -> Self {
            *self
        }

        fn clone_from(&mut self, source: &Self) {
            *self = *source
        }
    }
    impl Demo for B {
        fn clone2(&self) -> Self {
            *self
        }
    }
    impl Display for B {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "({},{})", self.z, self.k)
        }
    }
    let c = B {
        z: 100,
        k: A { i: 100, j: 100 },
    };
    let d = c.clone2();
    println!("{}", c);
    println!("{}", d);
    println!("{:p}", &c);
    println!("{:p}", &d);
    println!("{}", mem::size_of::<B>());
    thread::spawn(move || println!("{}", a)).join().unwrap();
    thread::spawn(move || println!("{}", a)).join().unwrap();
    struct C {
        i: i32,
        j: i32,
    }
    impl Display for C {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "({},{})", self.i, self.j)
        }
    }
    unsafe impl Send for C {}
    unsafe impl Sync for C {}

    let e = C { i: 100, j: 100 };
    //内不可变性！
    //引用计数
    //智能指针！
    let arc = Arc::new(e);
    let arc2 = arc.clone();
    let arc3 = arc.clone();
    thread::spawn(move || {
        println!("thread1:{}", arc2);
        println!("thread1:{}", Arc::weak_count(&arc2));
        println!("thread1:{}", Arc::strong_count(&arc2));
    })
    .join()
    .unwrap();
    println!("main:{}", Arc::weak_count(&arc));
    thread::spawn(move || {
        println!("thread2:{}", arc3);
        println!("thread2:{}", Arc::weak_count(&arc3));
        println!("thread2:{}", Arc::strong_count(&arc3));         
        // Clones inner data

    })
    .join()
    .unwrap();
    println!("main:{}", Arc::weak_count(&arc));
    println!("main:{}", Arc::strong_count(&arc));
    println!("main:{}", arc);
}
