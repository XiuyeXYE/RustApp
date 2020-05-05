
use std::cell::Cell;
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Debug)]
struct A{
    i:i32,
    j:i32
}

trait AI{}

// impl AI for *{

// }

pub trait BI:'static{
    fn f(&self);
}
extern "Rust"{

}

impl <T:'static> BI for T{
    fn f(&self){
        println!("f");
    }
}


// #[derive[Debug]]

impl Debug for dyn BI{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> 
    { 
        write!(f,"BI")
    }
}

impl dyn BI{
    fn g(&self){
        println!("g{:?}",self);
    }
}


// impl Sized for BI{}

fn h(a : &dyn BI){
    a.g();
}

trait Descriptive {
    fn describe(&self) -> String {
        String::from("[Object]")
    }
}

struct Person {
    name: String,
    age: u8
}

impl Descriptive for Person {
    //real achievement , or use default!
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}
impl Descriptive for &Person {
    //real achievement , or use default!
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}
fn output(object: impl Descriptive) {
    println!("{}", object.describe());
}
//<=>
// fn output<T: Descriptive>(object: T) {
//     println!("{}", object.describe());
// }
fn  output2<'a>(object: &'a dyn Descriptive) {
    println!("{}", object.describe());
}

// impl BI for Descriptive{
//     fn f(&self){
//         println!("f ds");
//     }
// }
impl BI for dyn Descriptive{
    fn f(&self){
        println!("f ds");
    }
}
// impl BI for impl Descriptive{
//     fn f(&self){
//         println!("f ds");
//     }
// }


fn main() {
    
    //inner variant!
    let cell = RefCell::new(A{i:100,j:100});
    println!("{:?}",cell);

    cell.f();
    h(&Box::new(cell));
    // BI::g(&Box::new(cell));
    // cell.g();

    // let cell = Cell::new(A{i:100,j:100});
    // println!("{:?}",cell);

   
    union D{
        i:i32,
    }
    let cali = Person {
        name: String::from("Cali"),
        age: 24
    };
    println!("{}", cali.describe());
    output(&cali);
    output2(&cali);
    output2(&cali);
    output2(&cali);
    
    

}
