use std::usize;

use crate::lexer::{Token, TokenType};

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut nodes = vec![];
    let len = tokens.len();
    let mut parser = Parser::new(tokens);
    while parser.pos != len {
        nodes.push(parser.comp_unit());
    }
    nodes
}

//Nodes
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum BasicType {
    Nil,
    Int,
    Float,
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

    // pub fn to_dot(&self) -> String {
    //     let mut dot = String::new();
    //     dot.push_str("digraph G {\n");
    //     self.to_dot_helper(&mut dot, 0);
    //     dot.push_str("}\n");
    //     dot
    // }

    // fn to_dot_helper(&self, dot: &mut String, parent_id: usize) -> usize {
    //     let current_id = parent_id + 1;
    //     dot.push_str(&format!(
    //         "  node{} [label=\"{:?}\"];\n",
    //         current_id, self.ntype
    //     ));
    //     if parent_id > 0 {
    //         dot.push_str(&format!("  node{} -> node{};\n", parent_id, current_id));
    //     }
    //     match &self.ntype {
    //         NodeType::BinOp(_, lhs, rhs) => {
    //             let lhs_id = lhs.to_dot_helper(dot, current_id);
    //             rhs.to_dot_helper(dot, lhs_id);
    //         }
    //         NodeType::Call(_, args, _) => {
    //             for arg in args {
    //                 arg.to_dot_helper(dot, current_id);
    //             }
    //         }
    //         NodeType::Access(_, indexes, _) => {
    //             if let Some(indexes) = indexes {
    //                 for index in indexes {
    //                     index.to_dot_helper(dot, current_id);
    //                 }
    //             }
    //         }
    //         NodeType::Declare(_, _, dims, init, _) => {
    //             if let Some(dims) = dims {
    //                 for dim in dims {
    //                     dim.to_dot_helper(dot, current_id);
    //                 }
    //             }
    //             if let Some(init) = init {
    //                 for init_node in init {
    //                     init_node.to_dot_helper(dot, current_id);
    //                 }
    //             }
    //         }
    //         NodeType::InitList(nodes) | NodeType::DeclStmt(nodes) | NodeType::Block(nodes) => {
    //             for node in nodes {
    //                 node.to_dot_helper(dot, current_id);
    //             }
    //         }
    //         NodeType::If(cond, if_block, else_block) => {
    //             let cond_id = cond.to_dot_helper(dot, current_id);
    //             let if_block_id = if_block.to_dot_helper(dot, cond_id);
    //             if let Some(else_block) = else_block {
    //                 else_block.to_dot_helper(dot, if_block_id);
    //             }
    //         }
    //         NodeType::While(cond, body) => {
    //             let cond_id = cond.to_dot_helper(dot, current_id);
    //             body.to_dot_helper(dot, cond_id);
    //         }
    //         NodeType::Assign(_, indexes, expr, _) => {
    //             if let Some(indexes) = indexes {
    //                 for index in indexes {
    //                     index.to_dot_helper(dot, current_id);
    //                 }
    //             }
    //             expr.to_dot_helper(dot, current_id);
    //         }
    //         NodeType::ExprStmt(expr) | NodeType::Return(Some(expr)) => {
    //             expr.to_dot_helper(dot, current_id);
    //         }
    //         NodeType::Func(_, _, args, body) => {
    //             for arg in args {
    //                 arg.to_dot_helper(dot, current_id);
    //             }
    //             body.to_dot_helper(dot, current_id);
    //         }
    //         _ => {}
    //     }
    //     current_id
    // }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    //Tool functions
    fn expect(&mut self, ttype: TokenType) {
        let t = self.get_current_token();
        if t.ttype != ttype {
            t.wrong_token(format!("{:?}", ttype));
        }
        self.pos += 1;
    }

    fn seek(&mut self, ttype: TokenType) -> bool {
        let t = self.get_current_token();
        if t.ttype != ttype {
            return false;
        }
        self.pos += 1;
        true
    }

    fn get_current_token(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    fn start_count(&self) -> usize {
        self.tokens[self.pos].start
    }

    fn stop_count(&self) -> usize {
        self.tokens[self.pos - 1].end
    }

    //Expressions (priority high to low)
    fn primary_expr(&mut self, is_cond: bool) -> Node {
        let t = self.get_current_token();
        let startpos = t.start;
        self.pos += 1;
        let result = match &t.ttype {
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
                t.wrong_token("expression".into());
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
        match t.ttype {
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
            TokenType::Int | TokenType::Float | TokenType::Const => {
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
            match self.get_current_token().ttype {
                TokenType::LeftBrace => {
                    let n = Node::new(NodeType::InitList(self.init_val()));
                    let endpos = self.stop_count();
                    init.push(n.set_range(startpos, endpos));
                }
                TokenType::Ident(_) | TokenType::Number(_) | TokenType::LeftParen => {
                    init.push(self.add_expr(false));
                }
                _ => {
                    self.get_current_token()
                        .wrong_token("expression or initlist".into());
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
        let btype = match t.ttype {
            TokenType::Const => {
                self.expect(TokenType::Int);
                Some(BasicType::Const)
            }
            TokenType::Int => Some(BasicType::Int),
            _ => {
                t.wrong_token("type define".into());
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
                self.get_current_token()
                    .wrong_token("assign in const declaration".into());
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
        let result = match t.ttype {
            TokenType::Void => Some(BasicType::Void),
            TokenType::Int => Some(BasicType::Int),
            TokenType::Const => {
                self.expect(TokenType::Int);
                Some(BasicType::Const)
            }
            _ => {
                t.wrong_token("type declaration".into());
                None
            }
        };
        result.expect("Typename required")
    }

    fn ident(&mut self) -> String {
        let name: String;
        if let TokenType::Ident(id) = &self.get_current_token().ttype {
            self.pos += 1;
            name = id.clone();
        } else {
            self.get_current_token()
                .wrong_token("function or value name".into());
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
