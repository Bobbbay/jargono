use crate::util::{Node, Type, UnaryOp};



use std::process::exit;

pub fn parse(input: String) -> Vec<Node> {
    /*
    Chic will start panicking if it sees a tab, because although they generally have a
    four-character width, chic thinks \t has a width of one. For the sake of good error message
    reporting, and because we don't care about tab users anyways, we convert all `\t`s to `    `.
    */
    let input = input.replace("\t", "    ");

    let res = jargono::parse(&input);

    let res = match res {
        Ok(res) => res,
        Err(e) => {
            let source = input.lines().nth(e.location.line - 1).unwrap().to_owned();

            let msg = chic::Error::new("An error occurred while parsing:")
                .error(
                    e.location.line,
                    e.location.column - 1,
                    e.location.column,
                    source,
                    format!("Expected: {}", e.expected),
                )
                .to_string();

            eprintln!("{}", msg);
            exit(1)
        }
    };

    res
}

peg::parser! {
  grammar jargono() for str {
    use crate::util::{Node, BinaryOp, Argument};

    rule space() = ['\t' | ' ']+                            // Minimum one space: ` `, `         `, etc.
    rule opt_space() = space()*                             // An optional space: ``, ` `, `         `, etc.
    rule line_comment() = "//" (!"\n" [_])* ("\n" / ![_])   // Single-lined comment: `// abc`, etc.
    rule multiline_comment() = "/*" (!"*/" [_])* "*/"       // Multiline comment: `/*\n\nabc\n\n*/`, etc.
    rule nl() = ['\n' | '\r']                               // Newline: `\n` `\r`
    rule opt_nl() = (nl() / space())*                       // Optional newline or space: ``, ` `, `\n `, etc.
    rule w() = quiet!{ (space() / multiline_comment())* }   // Optional space and/or inline comment: ``, ` `, `         `, `/* abc */`, etc.
    rule wn() = quiet!{ (space() / nl() / line_comment() / multiline_comment())* } // Optional space, newline, inline comment or multiline comment: ``. ` `, `\n`, `// abc`, `/* abc */`, etc.

    rule number() -> i32
      = n:$(['0'..='9']+) {? n.parse().or(Err("Wasn't able to turn `n` into i32.")) }

    rule match_type() -> Type
      = precedence! {
          "bool" { Type::Bool }
          "int"  { Type::Int  }
        }

    rule arithmetic() -> Box<Node> = precedence!{
      x:(@) opt_space() "+" opt_space() y:@ { Box::new(Node::BinaryExpr { op: BinaryOp::Plus, lhs: x, rhs: y }) }
      x:(@) opt_space() "-" opt_space() y:@ { Box::new(Node::BinaryExpr { op: BinaryOp::Minus, lhs: x, rhs: y }) }
      // --
      // x:(@) "*" y:@ { x * y }
      // x:(@) "/" y:@ { x / y }
      // --
      // x:@ "^" y:(@) { x.pow(y as u32) }
      // --
      n:number() { Box::new(Node::Int(n)) }
      "(" e:arithmetic() ")" { e }
    }

    rule ident() -> String
      = n:$(['a'..='z' | 'A'..='Z']+) {? n.parse().or(Err("alphanumeric"))}

    rule arguments() -> Vec<Argument> = { vec![] }

    rule statement() -> Box<Node> = precedence!{
      "let" space() name:ident() space() "=" space() value:statement() { Box::new(Node::Assign(name, value)) }
      --
      "return" space() child:@ { Box::new(Node::UnaryExpr { op: UnaryOp::Return, child }) }
      "-" opt_space() child:@ { Box::new(Node::UnaryExpr { op: UnaryOp::Minus, child }) }
      --
      child:arithmetic() { child }
      "true" { Box::new(Node::Bool(true)) }
      "false" { Box::new(Node::Bool(false)) }
      --
      // If all else fails, it must be a ref
      name:ident() "(" arguments:arguments() ")" { Box::new(Node::FnRef(name, arguments)) }
      child:ident() { Box::new(Node::Ref(child)) }
    }

    rule def_func() -> Node
      = "fn" space() name:ident() opt_space()                               // fn main
        "(" opt_space() arguments:arguments() opt_space() ")" opt_space()  // (a: bool, b: int)
        "->" opt_space() ret_val:match_type() opt_space()                   // -> int
        "{" children:(wn() children:statement() { children } ) ** ";" ";" wn() "}"        // { return 1 + 1; }
        {
            Node::Function {
              name,
              arguments: vec![],
              return_value: ret_val,
              children
            }
        }

    pub rule parse() -> Vec<Node>
      = l:def_func() ** wn() { l }
  }
}
