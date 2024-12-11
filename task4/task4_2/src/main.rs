use std::env;

mod lexer;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    lexer::tokenize(&args[1].clone()).iter().for_each(|x| {
        println!("{:?}", x);
    });
}
/*
Token{  type:Int  content:"int"  start:0  end:3  lineno:1  }
Token{  type:Ident("inc")  content:"inc"  start:4  end:7  lineno:1  }
Token{  type:LeftParen  content:"("  start:7  end:8  lineno:1  }
Token{  type:RightParen  content:")"  start:8  end:9  lineno:1  }
Token{  type:LeftBrace  content:"{"  start:0  end:1  lineno:2  }
Token{  type:Int  content:"int"  start:1  end:4  lineno:3  }
Token{  type:Ident("i")  content:"i"  start:5  end:6  lineno:3  }
Token{  type:Semicolon  content:";"  start:6  end:7  lineno:3  }
Token{  type:Ident("i")  content:"i"  start:1  end:2  lineno:4  }
Token{  type:Assign  content:"="  start:3  end:4  lineno:4  }
Token{  type:Ident("i")  content:"i"  start:5  end:6  lineno:4  }
Token{  type:Plus  content:"+"  start:6  end:7  lineno:4  }
Token{  type:Number(1)  content:""  start:8  end:8  lineno:4  }
Token{  type:Semicolon  content:";"  start:8  end:9  lineno:4  }
Token{  type:RightBrace  content:"}"  start:0  end:1  lineno:5  }
 */
