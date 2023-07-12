fn main() {
    println!("Hello, Rust Bootcamp by VBI Academy!");
   exercise5();
}


#[derive(Debug)]
struct Foo {
    str_val: String,
    int_val: i32,
}

impl Foo {
    fn clone(&self) -> Foo {
        let other = Foo{str_val: self.str_val.clone(), int_val: self.int_val.clone()};
        other
    }
}

fn exercise5() {
    let mut foos = Vec::new();
    foos.push(Foo {
        str_val: "ten".to_string(),
        int_val: 10,
    });
    foos.push(Foo {
        str_val: "twenty".to_string(),
        int_val: 20,
    });

    
    let moved = foos[0].clone();

    
    let moved_field = foos[0].str_val.clone();
}
