
pub mod script {
    pub enum OpCode {
        Zero,
        False,
        One,
        True,
        Two,
        Sixteen,
        Drop,
        Dup,
        Add,
        Sub,
    }

    fn run (ops:&[OpCode]) {
        for op in ops.iter() {
            match *op {
                Zero|False => println!("sup falsey"),
                One|True => println!("sup truthy"),
                _ => println!("sup everything else")
            }
        }
    }

    pub fn hello() -> String {
        "Hello, script!".to_string ()    
    }

    pub fn run_sample () {
        run ([Zero, One, Dup,]);
    }

    
}

