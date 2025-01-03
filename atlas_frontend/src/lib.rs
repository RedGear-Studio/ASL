use lexer::AtlasLexer;
use parser::{ast::AbstractSyntaxTree, ParseError};

pub mod lexer;
pub mod parser;

pub fn parse() -> Result<AbstractSyntaxTree, ParseError>{
        //"default()" setup all the systems you asked for, tho you could use "new()" to add them manually
        let mut lex: AtlasLexer = lexer::AtlasLexer::default();
        let tokens = lex
            .set_path("src/main.atlas")
            .set_source(String::from(
                r#"
let main: () -> int = check_perfect_numbers(500)


let check_perfect_numbers: (num: int) -> int = match num
| 0 ~> 42,
\ _ ~> do
    let divisor_sum: int = find_divisors_sum(num, 1, 0);
    print(divisor_sum);
    match divisor_sum
    | num ~> 
        do
            print(num);
            check_perfect_numbers(num - 1, 1, 0);
        end,
    \ _ ~> check_perfect_numbers(num - 1, 1, 0);
end


let find_divisors_sum: (n: int, j: int, divisor_sum: int) -> int = match j
| n ~> divisor_sum,
\ _ ~> match n % j
    | 0 ~> find_divisors_sum(n, j + 1, divisor_sum + 1),
    \ _ ~> find_divisors_sum(n, j + 1, divisor_sum)"#,
            ))
            .tokenize().unwrap();
    let mut parser = parser::SimpleParserV1::new();
    parser.with_tokens(tokens);
    let program = parser.parse();
    return program;
}


