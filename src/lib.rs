
pub mod script {
    #[deriving(PartialEq)]
    pub enum OpCode {
        Zero,
        False,
        One,
        True,
        Number(int), // enforce 2-16 in interpreter
        Drop,
        Dup,
        Add,
        Sub,
        Return,
        Verify,
        Equal,
    }

    fn add (x:int, y:int) -> int {
        x + y
    }
    fn sub (x:int, y:int) -> int {
        x - y
    }
    fn run (ops:&[OpCode]) {

        let mut stack = Vec::with_capacity(ops.len());

        for op in ops.iter() {
            match *op {
                Zero|False => stack.push(0i),
                One|True => stack.push(1i),
                Number(num) if num <= 16 && num >= 2 
                    => stack.push(num),
                Add|Sub => {
                    let result = 
                        match (stack.pop(), stack.pop()) {
                            (Some(num1), Some(num2)) =>
                                if *op == Add {add (num1, num2)} else {sub (num1, num2)},
                            (Some(num), None) => // this behavior may
                                // not actually be correct
                                if *op == Add {num} else {-num},
                            _ => 0,
                        };
                    stack.push(result)
                },
                Drop => { stack.pop(); },
                Dup => 
                    match stack.pop () {
                        Some (num) => {
                            stack.push (num);
                            stack.push (num);
                        },
                        None => continue,
                    },
                Equal => {
                    let (x,y) = (stack.pop(),stack.pop());
                    if x == y {stack.push (1i)} else {stack.push (0i)}
                }
                _ => continue,
            }
        }
        println!("{}", stack);
    }

    pub fn hello() -> String {
        "Hello, script!".to_string()    
    }

    pub fn run_sample () {
        run ([Zero, One, Number(6), Add, Sub, Number (7), Equal]);
    }

    
}

