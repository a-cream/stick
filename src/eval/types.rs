#[derive(Debug)]
pub enum Token {
    Number(f64),

    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Ast {
    Number(f64),

    Plus(Box<Ast>, Box<Ast>),
    Minus(Box<Ast>, Box<Ast>),
    Multiply(Box<Ast>, Box<Ast>),
    Divide(Box<Ast>, Box<Ast>),
}
