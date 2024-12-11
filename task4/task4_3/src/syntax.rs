use std::{fmt::Debug, usize};

use crate::{
    lexer::{error, read_file},
    token::{Token, TokenType},
};

pub fn parse(tokens: Vec<Token>, path: &str) -> Vec<Node> {
    let mut nodes = vec![];
    let len = tokens.len();
    let mut parser = Parser::new(tokens, path);
    while parser.pos != len {
        nodes.push(parser.comp_unit());
    }
    nodes
}

//Nodes
#[derive(Clone)]
pub enum NodeType {
    Nil,
    //Expressions
    ///Value
    Number(i32),

    ///#### Value declaration
    ///Type,Name,Dimensions,Init list,Scope
    Declare(
        BasicType,
        String,
        Option<Vec<Node>>,
        Option<Vec<Node>>,
        Scope,
    ),
    ///#### Initialization list
    ///List of `Expr` or `InitList`
    InitList(Vec<Node>),

    ///#### Value access
    ///Array name,Indexes,Decl
    Access(String, Option<Vec<Node>>, Box<Node>),

    ///#### Binary operation
    ///Op,Lhs,Rhs
    BinOp(TokenType, Box<Node>, Box<Node>),

    ///#### Function call
    ///Name,Args,Func_decl
    Call(String, Vec<Node>, Box<Node>),

    //Statements
    ///#### Declaration sequence
    ///List of `Declare`
    DeclStmt(Vec<Node>),

    ///Name,Indexes,Expr,Lhs_decl
    Assign(String, Option<Vec<Node>>, Box<Node>, Box<Node>),

    ///Expression
    ExprStmt(Box<Node>),

    ///List of statement
    Block(Vec<Node>),

    ///Condition,If block,Else block
    If(Box<Node>, Box<Node>, Option<Box<Node>>),

    ///Condition,Body
    While(Box<Node>, Box<Node>),
    Break,
    Continue,
    ///Return value
    Return(Option<Box<Node>>),
    ///#### Function definition
    ///Return type,Name,Args(List of declares),Body
    Func(BasicType, String, Vec<Node>, Box<Node>),
}

impl Debug for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Nil => write!(f, "Nil"),
            NodeType::Number(num) => write!(f, "Number({})", num),
            NodeType::Declare(_, name, _, _, _) => write!(f, "Declare({})", name),
            NodeType::InitList(_) => write!(f, "InitList"),
            NodeType::Access(name, _, _) => write!(f, "Access({})", name),
            NodeType::BinOp(op, _, _) => write!(f, "BinOp({:?})", op),
            NodeType::Call(name, _, _) => write!(f, "Call({})", name),
            NodeType::DeclStmt(_) => write!(f, "DeclStmt"),
            NodeType::Assign(name, _, _, _) => write!(f, "Assign({})", name),
            NodeType::ExprStmt(_) => write!(f, "ExprStmt"),
            NodeType::Block(_) => write!(f, "Block"),
            NodeType::If(_, _, _) => write!(f, "If"),
            NodeType::While(_, _) => write!(f, "While"),
            NodeType::Break => write!(f, "Break"),
            NodeType::Continue => write!(f, "Continue"),
            NodeType::Return(_) => write!(f, "Return"),
            NodeType::Func(_, name, _, _) => write!(f, "Func({})", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BasicType {
    Nil,
    Int,
    Const,
    Void,
    IntArray(Vec<usize>),
    ConstArray(Vec<usize>),
    Func(Box<BasicType>), //return type
}
#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Local,
    Global,
    Param,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub ntype: NodeType,
    pub btype: BasicType,

    //for error messages
    pub start: usize,
    pub end: usize,
}

impl Node {
    pub fn new(ntype: NodeType) -> Self {
        Node {
            start: 0,
            end: 0,
            ntype,
            btype: BasicType::Nil,
        }
    }

    fn new_binop(ttype: TokenType, lhs: Node, rhs: Node) -> Self {
        Node::new(NodeType::BinOp(ttype, Box::new(lhs), Box::new(rhs)))
    }

    fn num_zero() -> Self {
        Node::new(NodeType::Number(0))
    }

    fn set_range(mut self, start: usize, end: usize) -> Self {
        self.start = start;
        self.end = end;
        self
    }

    pub fn show(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!("{}{:?}", indent_str, self.ntype);
        match &self.ntype {
            NodeType::Declare(_, _, Some(dims), Some(init), _) => {
                for dim in dims {
                    dim.show(indent + 2);
                }
                for val in init {
                    val.show(indent + 2);
                }
            }
            NodeType::InitList(vals) => {
                for val in vals {
                    val.show(indent + 2);
                }
            }
            NodeType::Access(_, Some(indexes), _) => {
                for index in indexes {
                    index.show(indent + 2);
                }
            }
            NodeType::BinOp(_, lhs, rhs) => {
                lhs.show(indent + 2);
                rhs.show(indent + 2);
            }
            NodeType::Call(_, args, _) => {
                for arg in args {
                    arg.show(indent + 2);
                }
            }
            NodeType::DeclStmt(decls) => {
                for decl in decls {
                    decl.show(indent + 2);
                }
            }
            NodeType::Assign(_, Some(indexes), expr, _) => {
                for index in indexes {
                    index.show(indent + 2);
                }
                expr.show(indent + 2);
            }
            NodeType::ExprStmt(expr) => {
                expr.show(indent + 2);
            }
            NodeType::Block(stmts) => {
                for stmt in stmts {
                    stmt.show(indent + 2);
                }
            }
            NodeType::If(cond, on_true, Some(on_false)) => {
                cond.show(indent + 2);
                on_true.show(indent + 2);
                on_false.show(indent + 2);
            }
            NodeType::If(cond, on_true, None) => {
                cond.show(indent + 2);
                on_true.show(indent + 2);
            }
            NodeType::While(cond, body) => {
                cond.show(indent + 2);
                body.show(indent + 2);
            }
            NodeType::Return(Some(expr)) => {
                expr.show(indent + 2);
            }
            NodeType::Func(_, _, args, body) => {
                for arg in args {
                    arg.show(indent + 2);
                }
                body.show(indent + 2);
            }
            _ => {}
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    path: String,
    code: Vec<char>,
}

impl Parser {
    fn new(tokens: Vec<Token>, path: &str) -> Self {
        Parser {
            tokens,
            pos: 0,
            path: path.to_string(),
            code: read_file(path),
        }
    }

    //Tool functions
    fn expect(&mut self, ttype: TokenType) {
        let t = self.get_current_token();
        if t.kind != ttype {
            error(
                &self.path,
                &self.code,
                t.range.start,
                t.line,
                t.range.start,
                &format!("expect {:?}", ttype),
                "Unexpected token",
            );
        }
        self.pos += 1;
    }

    fn seek(&mut self, ttype: TokenType) -> bool {
        let t = self.get_current_token();
        if t.kind != ttype {
            return false;
        }
        self.pos += 1;
        true
    }

    fn get_current_token(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    fn start_count(&self) -> usize {
        self.tokens[self.pos].range.start
    }

    fn stop_count(&self) -> usize {
        self.tokens[self.pos - 1].range.end
    }

    //Expressions (priority high to low)
    fn primary_expr(&mut self, is_cond: bool) -> Node {
        let t = self.get_current_token();
        let startpos = t.range.start;
        self.pos += 1;
        let result = match &t.kind {
            TokenType::LeftParen => {
                let exp = self.const_expr(is_cond);
                self.expect(TokenType::RightParen);
                Some(exp)
            }
            TokenType::Number(num) => Some(Node::new(NodeType::Number(*num))),
            TokenType::Ident(id) => {
                //Function call
                if self.seek(TokenType::LeftParen) {
                    let mut args = vec![];
                    if !self.seek(TokenType::RightParen) {
                        args.push(self.const_expr(is_cond));
                        while self.seek(TokenType::Comma) {
                            args.push(self.const_expr(is_cond));
                        }
                        self.expect(TokenType::RightParen);
                        Some(Node::new(NodeType::Call(
                            id.clone(),
                            args,
                            Box::new(Node::num_zero()),
                        )))
                    } else {
                        Some(Node::new(NodeType::Call(
                            id.clone(),
                            args,
                            Box::new(Node::num_zero()),
                        )))
                    }
                }
                //Array access
                else {
                    Some(Node::new(NodeType::Access(
                        id.to_string(),
                        self.seek_array(false),
                        Box::new(Node::num_zero()),
                    )))
                }
            }
            _ => {
                error(
                    &self.path,
                    &self.code,
                    t.range.start,
                    t.line,
                    t.range.start,
                    &format!("expect {:?}", "expression"),
                    "Unexpected token",
                );
                None
            }
        };
        let endpos = self.stop_count();
        result
            .expect("Wrong expression")
            .set_range(startpos, endpos)
    }

    fn unary_expr(&mut self, is_cond: bool) -> Node {
        let startpos = self.start_count();
        loop {
            if self.seek(TokenType::Plus) {
                continue;
            } else if self.seek(TokenType::Minus) {
                let mut rhs = Node::new_binop(
                    TokenType::Minus,
                    Node::num_zero(),
                    self.primary_expr(is_cond),
                );
                let endpos = self.stop_count();
                rhs = rhs.set_range(startpos, endpos);
                return rhs;
            } else if is_cond && self.seek(TokenType::Not) {
                let mut rhs = Node::new_binop(
                    TokenType::Equal,
                    self.primary_expr(is_cond),
                    Node::num_zero(),
                );
                let endpos = self.stop_count();
                rhs = rhs.set_range(startpos, endpos);
                return rhs;
            } else {
                break;
            }
        }

        self.primary_expr(is_cond)
    }

    fn mul_expr(&mut self, is_cond: bool) -> Node {
        let startpos = self.start_count();
        let mut lhs = self.unary_expr(is_cond);

        loop {
            if self.seek(TokenType::Multiply) {
                lhs = Node::new_binop(TokenType::Multiply, lhs, self.unary_expr(is_cond));
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else if self.seek(TokenType::Divide) {
                lhs = Node::new_binop(TokenType::Divide, lhs, self.unary_expr(is_cond));
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else if self.seek(TokenType::Modulus) {
                lhs = Node::new_binop(TokenType::Modulus, lhs, self.unary_expr(is_cond));
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else {
                return lhs;
            }
        }
    }

    fn add_expr(&mut self, is_cond: bool) -> Node {
        let startpos = self.start_count();
        let mut lhs = self.mul_expr(is_cond);

        loop {
            if self.seek(TokenType::Plus) {
                lhs = Node::new_binop(TokenType::Plus, lhs, self.mul_expr(is_cond));
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else if self.seek(TokenType::Minus) {
                lhs = Node::new_binop(TokenType::Minus, lhs, self.mul_expr(is_cond));
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else {
                return lhs;
            }
        }
    }

    fn const_expr(&mut self, is_cond: bool) -> Node {
        self.add_expr(is_cond)
    }

    fn rel_expr(&mut self) -> Node {
        let startpos = self.start_count();
        let mut lhs = self.add_expr(true);
        loop {
            if self.seek(TokenType::LesserThan) {
                lhs = Node::new_binop(TokenType::LesserThan, lhs, self.add_expr(true));
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else if self.seek(TokenType::GreaterThan) {
                lhs = Node::new_binop(TokenType::GreaterThan, lhs, self.add_expr(true));
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else if self.seek(TokenType::LesserEqual) {
                lhs = Node::new_binop(TokenType::LesserEqual, lhs, self.add_expr(true));
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else if self.seek(TokenType::GreaterEqual) {
                lhs = Node::new_binop(TokenType::GreaterEqual, lhs, self.add_expr(true));
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else {
                return lhs;
            }
        }
    }

    fn eq_expr(&mut self) -> Node {
        let startpos = self.start_count();
        let mut lhs = self.rel_expr();
        loop {
            if self.seek(TokenType::Equal) {
                lhs = Node::new_binop(TokenType::Equal, lhs, self.rel_expr());
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else if self.seek(TokenType::NotEqual) {
                lhs = Node::new_binop(TokenType::NotEqual, lhs, self.rel_expr());
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else {
                return lhs;
            }
        }
    }

    fn l_and_expr(&mut self) -> Node {
        let startpos = self.start_count();
        let mut lhs = self.eq_expr();
        loop {
            if self.seek(TokenType::And) {
                lhs = Node::new_binop(TokenType::And, lhs, self.eq_expr());
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else {
                return lhs;
            }
        }
    }

    fn l_or_expr(&mut self) -> Node {
        let startpos = self.start_count();
        let mut lhs = self.l_and_expr();
        loop {
            if self.seek(TokenType::Or) {
                lhs = Node::new_binop(TokenType::Or, lhs, self.l_and_expr());
                let endpos = self.stop_count();
                lhs = lhs.set_range(startpos, endpos);
            } else {
                return lhs;
            }
        }
    }

    //Statements
    fn stmt(&mut self) -> Node {
        let startpos = self.start_count();
        let t = self.get_current_token();
        self.pos += 1;
        match t.kind {
            TokenType::Ident(id) => {
                let pos = self.pos;
                let index = self.seek_array(false);
                if self.seek(TokenType::Assign) {
                    let expr = self.add_expr(false);
                    self.expect(TokenType::Semicolon);
                    let endpos = self.stop_count();
                    Node::new(NodeType::Assign(
                        id,
                        index,
                        Box::new(expr),
                        Box::new(Node::num_zero()),
                    ))
                    .set_range(startpos, endpos)
                } else {
                    self.pos = pos - 1;
                    let expr = self.add_expr(false);
                    self.expect(TokenType::Semicolon);
                    let endpos = self.stop_count();
                    Node::new(NodeType::ExprStmt(Box::new(expr))).set_range(startpos, endpos)
                }
            }
            TokenType::Int | TokenType::Const => {
                self.pos -= 1;
                self.decl_stmt(Scope::Local)
            }
            TokenType::LeftBrace => {
                self.pos -= 1;
                self.block()
            }
            TokenType::If => {
                let on_false: Option<Box<Node>>;
                self.expect(TokenType::LeftParen);
                let cond = self.l_or_expr();
                self.expect(TokenType::RightParen);
                let on_true = self.stmt();
                if self.seek(TokenType::Else) {
                    on_false = Some(Box::new(self.stmt()));
                } else {
                    on_false = None;
                }
                let endpos = self.stop_count();
                Node::new(NodeType::If(Box::new(cond), Box::new(on_true), on_false))
                    .set_range(startpos, endpos)
            }
            TokenType::While => {
                self.expect(TokenType::LeftParen);
                let cond = self.l_or_expr();
                self.expect(TokenType::RightParen);
                let body = self.stmt();
                let endpos = self.stop_count();
                Node::new(NodeType::While(Box::new(cond), Box::new(body)))
                    .set_range(startpos, endpos)
            }
            TokenType::Break => {
                self.expect(TokenType::Semicolon);
                let endpos = self.stop_count();
                Node::new(NodeType::Break).set_range(startpos, endpos)
            }
            TokenType::Continue => {
                self.expect(TokenType::Semicolon);
                let endpos = self.stop_count();
                Node::new(NodeType::Continue).set_range(startpos, endpos)
            }
            TokenType::Return => {
                let ret: Option<Box<Node>>;
                if self.seek(TokenType::Semicolon) {
                    ret = None;
                } else {
                    ret = Some(Box::new(self.add_expr(false)));
                    self.expect(TokenType::Semicolon);
                }
                let endpos = self.stop_count();
                Node::new(NodeType::Return(ret)).set_range(startpos, endpos)
            }
            _ => {
                let expr = self.add_expr(false);
                self.expect(TokenType::Semicolon);
                let endpos = self.stop_count();
                Node::new(NodeType::ExprStmt(Box::new(expr))).set_range(startpos, endpos)
            }
        }
    }

    fn init_val(&mut self) -> Vec<Node> {
        let mut init = vec![];
        let mut first = true;
        self.expect(TokenType::LeftBrace);
        while !self.seek(TokenType::RightBrace) {
            if first {
                first = false;
            } else {
                self.expect(TokenType::Comma);
            }
            let startpos = self.start_count();
            match self.get_current_token().kind {
                TokenType::LeftBrace => {
                    let n = Node::new(NodeType::InitList(self.init_val()));
                    let endpos = self.stop_count();
                    init.push(n.set_range(startpos, endpos));
                }
                TokenType::Ident(_) | TokenType::Number(_) | TokenType::LeftParen => {
                    init.push(self.add_expr(false));
                }
                _ => {
                    let t = self.get_current_token();
                    error(
                        &self.path,
                        &self.code,
                        t.range.start,
                        t.line,
                        t.range.start,
                        &format!("expect {:?}", "expression or initlist"),
                        "Unexpected token",
                    );
                }
            }
        }
        init
    }

    ///Declaration
    fn decl_stmt(&mut self, scope: Scope) -> Node {
        let startpos = self.start_count();
        let t = self.get_current_token();
        self.pos += 1;
        let btype = match t.kind {
            TokenType::Const => {
                self.expect(TokenType::Int);
                Some(BasicType::Const)
            }
            TokenType::Int => Some(BasicType::Int),
            _ => {
                error(
                    &self.path,
                    &self.code,
                    t.range.start,
                    t.line,
                    t.range.start,
                    &format!("expect {:?}", "type define"),
                    "Unexpected token",
                );
                None
            }
        }
        .expect("Expect type define");
        let mut first = true;
        let mut decl_list = vec![];
        while !self.seek(TokenType::Semicolon) {
            if first {
                first = false;
            } else {
                self.expect(TokenType::Comma);
            }
            let startpos = self.start_count();
            let name = self.ident();
            let dims = self.seek_array(false);
            let init: Option<Vec<Node>>;
            if self.seek(TokenType::Assign) {
                if dims.is_none() {
                    init = Some(vec![self.add_expr(false)]);
                } else {
                    init = Some(self.init_val());
                }
            } else if btype == BasicType::Const {
                let t = self.get_current_token();
                error(
                    &self.path,
                    &self.code,
                    t.range.start,
                    t.line,
                    t.range.start,
                    &format!("expect {:?}", "assign in const declaration"),
                    "Unexpected token",
                );
                unreachable!();
            } else {
                init = None;
            }
            let endpos = self.stop_count();
            decl_list.push(
                Node::new(NodeType::Declare(
                    btype.clone(),
                    name,
                    dims,
                    init,
                    scope.clone(),
                ))
                .set_range(startpos, endpos),
            );
        }
        let endpos = self.stop_count();
        Node::new(NodeType::DeclStmt(decl_list)).set_range(startpos, endpos)
    }

    fn block(&mut self) -> Node {
        let startpos = self.start_count();
        let mut stmts = vec![];
        self.expect(TokenType::LeftBrace);
        while !self.seek(TokenType::RightBrace) {
            stmts.push(self.stmt());
        }
        let endpos = self.stop_count();
        Node::new(NodeType::Block(stmts)).set_range(startpos, endpos)
    }

    fn basic_type(&mut self) -> BasicType {
        let t = self.get_current_token();
        self.pos += 1;
        let result = match t.kind {
            TokenType::Void => Some(BasicType::Void),
            TokenType::Int => Some(BasicType::Int),
            TokenType::Const => {
                self.expect(TokenType::Int);
                Some(BasicType::Const)
            }
            _ => {
                error(
                    &self.path,
                    &self.code,
                    t.range.start,
                    t.line,
                    t.range.start,
                    &format!("expect {:?}", "type declaration"),
                    "Unexpected token",
                );
                None
            }
        };
        result.expect("Typename required")
    }

    fn ident(&mut self) -> String {
        let name: String;
        if let TokenType::Ident(id) = &self.get_current_token().kind {
            self.pos += 1;
            name = id.clone();
        } else {
            let t = self.get_current_token();
            error(
                &self.path,
                &self.code,
                t.range.start,
                t.line,
                t.range.start,
                &format!("expect {:?}", "function or value name"),
                "Unexpected token",
            );
            return "".to_string();
        }
        name
    }

    fn seek_array(&mut self, is_param: bool) -> Option<Vec<Node>> {
        let mut v = vec![];
        let mut allow_empty = is_param;
        while self.seek(TokenType::LeftBracket) {
            let startpos = self.start_count();
            if allow_empty {
                allow_empty = false;
                while !self.seek(TokenType::RightBracket) {
                    self.pos += 1;
                }
                let endpos = self.stop_count();
                v.push(Node::new(NodeType::Nil).set_range(startpos, endpos));
                continue;
            }

            let len = self.const_expr(false);
            v.push(len);
            self.expect(TokenType::RightBracket);
        }

        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    }

    fn func_f_param(&mut self) -> Node {
        let startpos = self.start_count();
        self.expect(TokenType::Int);
        let name = self.ident();
        let dim = self.seek_array(true);
        let btype: BasicType;
        if dim.is_none() {
            btype = BasicType::Int;
        } else {
            btype = BasicType::IntArray(vec![0]);
        }
        let endpos = self.stop_count();
        Node::new(NodeType::Declare(btype, name, dim, None, Scope::Param))
            .set_range(startpos, endpos)
    }

    fn comp_unit(&mut self) -> Node {
        let startpos = self.start_count();
        let pos = self.pos;
        let btype = self.basic_type();
        let name = self.ident();

        if self.seek(TokenType::LeftParen) {
            let mut args = vec![];
            if !self.seek(TokenType::RightParen) {
                args.push(self.func_f_param());
                while self.seek(TokenType::Comma) {
                    args.push(self.func_f_param());
                }
                self.expect(TokenType::RightParen);
            }
            let body = self.block();
            let endpos = self.stop_count();
            return Node::new(NodeType::Func(btype, name, args, Box::new(body)))
                .set_range(startpos, endpos);
        }

        self.pos = pos;
        self.decl_stmt(Scope::Global)
    }
}
