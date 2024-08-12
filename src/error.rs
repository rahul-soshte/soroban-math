#[derive(Debug)]
pub enum ArithmeticError {
    Overflow,
    Underflow,
    DivisionByZero,
    InvalidInput,
    DomainError,
}
