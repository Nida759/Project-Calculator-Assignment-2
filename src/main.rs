use std::io;
use std::io::prelude::*;


/**********************************************************************
 * LEXER
 */

#[derive(Clone,PartialEq,Eq,Debug)]
enum Token {
  Num(String),
  addition,
  subtraction,
  multiplication,
  division,
  exponents,
//error messages when you do something wrong,
}

struct Lexer {
  buffer: String,
  offset: usize
}

impl Lexer {
  fn new() -> Lexer {
    let mut buffer = String::new();
    let offset = 0;
    io::stdin().read_line(&mut buffer).unwrap_or_else(|_| {
      panic!("Could not initialize lexer:  unable to read line");
    });

    Lexer {
      buffer: buffer,
      offset: offset
    }
  }

  fn advance(&mut self) {
    if self.offset >= self.buffer.len() - 1 {
      self.buffer = String::new();
      io::stdin().read_line(&mut self.buffer).unwrap_or_else(|_| {
        panic!("Could not advance lexer:  unable to read line");
      });
      self.offset = 0;
    } else {
      self.offset += self.current().len_utf8();
    }
  }

  fn current(&self) -> char {
    self.buffer[self.offset..].chars().next()
      .expect("Tried to get a nonsensical character")
  }

  fn get_token(&mut self) -> Token {
    let mut t = String::new();
    let mut c = self.current();

    while c.is_whitespace() {
      self.advance();
      c = self.current();
    }

    while c.is_digit(10) {
      t.push(c);
      self.advance();
      c = self.current();
    }

    if t.len() > 0 {
      return Token::Num(t);
    }

    self.advance();

    match c {
      '+' => Token::addition,
      '-' => Token::subtraction,
      '*' => Token::multiplication,
      '/' => Token::division,
      '^' => Token::exponents,
      x => panic!("unrecognized character: {}", x)
    }
  }
}



/**********************************************************************
 * PARSER
 */

// Abstract syntax tree
#[derive(Clone,Debug)]
enum AST {
  Num(i32),
  addition(Box<AST>, Box<AST>),
  subtraction(Box<AST>, Box<AST>),
  multiplication(Box<AST>, Box<AST>),
  division(Box<AST>, Box<AST>)
}

struct Parser<'a> {
  tok : Token,
  lex : &'a mut Lexer
}

impl<'a> Parser<'a> {
  fn new(lex : &'a mut Lexer) -> Parser {
    Parser {
      tok : lex.get_token(),
      lex : lex
    }
  }

  fn get_token(&mut self) {
    self.tok = self.lex.get_token();
  }

  fn eat(&mut self, t : Token) {
    if self.tok == t {
      self.get_token();
    } else {
      panic!("Syntax error: expected {:?}, found {:?}", t, self.tok);
    }
  }
  
  /********************************************************************
   * GRAMMAR PRODUCTIONS
   * x_ productions are hacks to make the grammar right recursive
   * and therefore suitable for recursive descent parsing
   */
  // Starting production. Use this as entry into the parser.
  fn program(&mut self) -> AST {
    let a = self.exp();
    self.semi(a)
  }

  fn exp(&mut self) -> AST {
    let t = self.term();
    self.exp_(t)
  }

  fn exp_(&mut self, t : AST) -> AST {
    match self.tok {
      Token::addition  => { self.eat(Token::addition);
                        let s = self.term();
                        let rc = AST::addition(Box::new(t), Box::new(s));
                        self.exp_(rc) },
      Token::subtraction => { self.eat(Token::subtraction);
                        let s = self.term();
                        let rc = AST::subtraction(Box::new(t), Box::new(s));
                        self.exp_(rc) },
      _ => { t }
    }
  }

  fn term(&mut self) -> AST {
    let f = self.factor();
    self.term_(f)
  }

  fn term_(&mut self, f : AST) -> AST {
    match self.tok {
      Token::multiplication  => { self.eat(Token::multiplication);
                         let g = self.factor();
                         let rc = AST::multiplication(Box::new(f), Box::new(g));
                         self.term_(rc) },
      Token::division => { self.eat(Token::division);
                         let g = self.factor();
                         let rc = AST::division(Box::new(f), Box::new(g));
                         self.term_(rc) },
      _ => { f }
    }
  }

  fn factor(&mut self) -> AST {
    let tok = self.tok.clone();  // Make the borrow checker stop complaining
    match tok {
      Token::Num(ref x) => { self.get_token();
                             AST::Num(x.parse::<i32>().unwrap()) } ,
      Token::exponents => { self.eat(Token::exponents);
                         let rc = self.exp();
                         self.eat(Token::exponents);
                         rc } ,
      _ => { panic!("Syntax error: expected number or parenthesis") }
    }
  }

  // Terminal production.  Ends parsing.
  fn semi(&mut self, a : AST) -> AST {
    a
  }
}



/**********************************************************************
 * INTERPRETER
 */

// Recursively evaluate the expression tree
fn evaluate(a : AST) -> i32 {
  match a {
    AST::Num(x) => x,
    AST::addition(x, y) => evaluate(*x) + evaluate(*y),
    AST::subtraction(x, y) => evaluate(*x) - evaluate(*y),
    AST::multiplication(x, y) => evaluate(*x) * evaluate(*y),
    AST::division(x, y) => evaluate(*x) / evaluate(*y)
  }
}



/**********************************************************************
 * MAIN
 */
 
fn main() {
  let mut lexer = Lexer::new();
  let mut parser = Parser::new(&mut lexer);

  println!("Enter an arithmetic expression using integers followed by a ;");

  let expression = parser.program();

  //println!("{:?}", expression);
  println!("{}", evaluate(expression));
}