mod lang;
use std::{collections::HashMap, io::stdin};
use std::error::Error;
use lang::code_analysis::syntax::lexer::Lexer;
use crate::lang::code_analysis::syntax::parser::Parser;

fn main() {

    let lexer= Lexer::new(r"fun abc(test:int,alpha:float)
    {
        let a= 5.6+c*9+abc(a,b);
        test(45,7+9);
        return 5;
        while(a>b)
        {
            a=a+b;
            b=a-b;
        }
        if a+b
        {
            return;
        }
        else if a+b
        {
            return 5;
        }
        else
        {
            return 2;
        }

    }".to_string());
    let mut parser=Parser::new(lexer);
    let ast=parser.parse();
    match ast {
        Ok(ast) =>
            println!("{:?}",ast),

        Err(e) => println!("error: {:?}",e.description()),
    }
}

