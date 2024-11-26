use crate::grammar::Grammar;
use crate::symbol::Symbol;
use crate::sysy::symbols::*;
use std::sync::LazyLock;

#[rustfmt::skip]
lazy_static::lazy_static! {
    pub static ref SYSY: std::sync::Mutex<Grammar> = {
        let mut sys_y_grammar = Grammar::new();
        let mut make = |lhs: LazyLock<Symbol>, rhs: Vec<&LazyLock<Symbol>>|  {
            sys_y_grammar.add_rule(*lhs, *rhs);
        };

        // CompUnit -> [ CompUnit ] ( Decl | FuncDef )
        // make(&CompUnit, vec![&Decl]);
        // make(&CompUnit, vec![&FuncDef]);
        // make(&CompUnit, vec![&CompUnit, &Decl]);
        // make(&CompUnit, vec![&CompUnit, &FuncDef]);

        // // Decl -> ConstDecl | VarDecl
        // make(&Decl, vec![&ConstDecl]);
        // make(&Decl, vec![&VarDecl]);

        // // ConstDecl -> 'const' BType ConstDef { ',' ConstDef } ';'
        // make(&ConstDecl,  vec![&Const, &BType, &ConstDef, &ConstDecl_, &Semicolon]);
        // make(&ConstDecl_, vec![&Epsilon]);
        // make(&ConstDecl_, vec![&ConstDecl_, &Commo, &ConstDef]);

        // // BType -> 'int' | 'float'
        // make(&BType, vec![&Int]);
        // make(&BType, vec![&Float]);

        // // ConstDef -> Ident { '[' ConstExp ']' } '=' ConstInitVal
        // make(&ConstDef,  vec![&Ident, &ConstDef_, &Equal, &ConstInitVal]);
        // make(&ConstDef_, vec![&Epsilon]);
        // make(&ConstDef_, vec![&ConstDef_, &LBracket, &ConstExp, &RBracket]);

        // // ConstInitVal  ConstExp | '{' [ ConstInitVal { ',' ConstInitVal } ] '}'
        // make(&ConstInitVal,   vec![&ConstExp]);
        // make(&ConstInitVal,   vec![&LBrace, &ConstInitVal_, &RBrace]);
        // make(&ConstInitVal_,  vec![&Epsilon]);
        // make(&ConstInitVal_,  vec![&ConstInitVal, &ConstInitVal_2]);
        // make(&ConstInitVal_2, vec![&Epsilon]);
        // make(&ConstInitVal_2, vec![&ConstInitVal_2, &Commo, &ConstInitVal]);

        // // VarDecl ->  BType VarDef { ',' VarDef } ';'
        // make(&VarDecl,  vec![&BType, &VarDef, &VarDecl_, &Semicolon]);
        // make(&VarDecl_, vec![&Epsilon]);
        // make(&VarDecl_, vec![&VarDecl_, &Commo, &VarDef]);

        // // VarDef -> Ident { '[' ConstExp ']' } | Ident { '[' ConstExp ']' } '=' InitVal
        // make(&VarDef,   vec![&Ident, &VarDef_, &VarDef_2]);
        // make(&VarDef_,  vec![&Epsilon]);
        // make(&VarDef_,  vec![&VarDef_, &LBracket, &ConstExp, &RBracket]);
        // make(&VarDef_2, vec![&Epsilon]);
        // make(&VarDef_2, vec![&Equal, &InitVal]);

        // // InitVal -> ConstExp | '{' [ InitVal { ',' InitVal } ] '}'
        // make(&InitVal,   vec![&ConstExp]);
        // make(&InitVal,   vec![&LBrace, &InitVal_, &RBrace]);
        // make(&InitVal_,  vec![&Epsilon]);
        // make(&InitVal_,  vec![&InitVal, &InitVal_2]);
        // make(&InitVal_2, vec![&Epsilon]);
        // make(&InitVal_2, vec![&InitVal_2, &Commo, &InitVal]);

        // // FuncDef -> FuncType Ident '(' [FuncFParams] ')' Block
        // make(&FuncDef,  vec![&FuncType, &Ident, &LParen, &FuncDef_, &RParen, &Block]);
        // make(&FuncDef_, vec![&Epsilon]);
        // make(&FuncDef_, vec![&FuncFParams]);

        // // FuncType -> 'void' | 'int' | 'float'
        // make(&FuncType, vec![&Void]);
        // make(&FuncType, vec![&Int]);
        // make(&FuncType, vec![&Float]);

        // // FuncFParams -> FuncFParam { ',' FuncFParam }
        // make(&FuncFParams, vec![&FuncFParam]);
        // make(&FuncFParams, vec![&FuncFParams, &Commo, &FuncFParam]);

        // // FuncFParam -> BType Ident ['[' ']' { '[' Exp ']' }]
        // make(&FuncFParam,   vec![&BType, &Ident, &FuncFParam_]);
        // make(&FuncFParam_,  vec![&Epsilon]);
        // make(&FuncFParam_,  vec![&LBracket, &RBracket, &FuncFParam_2]);
        // make(&FuncFParam_2, vec![&Epsilon]);
        // make(&FuncFParam_2, vec![&FuncFParam_2, &LBracket, &Exp, &RBracket]);

        // // Block -> '{' { BlockItem } '}'
        // make(&Block,  vec![&LBrace, &Block_, &RBrace]);
        // make(&Block_, vec![&Epsilon]);
        // make(&Block_, vec![&Block_, &BlockItem]);

        // // BlockItem -> Decl | Stmt
        // make(&BlockItem, vec![&Decl]);
        // make(&BlockItem, vec![&Stmt]);

        // // Stmt -> LVal '=' Exp ';' | [Exp] ';' | Block | 'if' '( Cond ')' Stmt [ 'else' Stmt ] |
        // // 'while' '(' Cond ')' Stmt | 'break' ';' | 'continue' ';' | 'return' [Exp] ';'
        // make(&Stmt, vec![&LVal, &Equal, &Exp, &Semicolon]);
        // make(&Stmt, vec![&Epsilon, &Semicolon]);
        // make(&Stmt, vec![&Exp, &Semicolon]);
        // make(&Stmt, vec![&Block]);
        // make(&Stmt, vec![&If, &LParen, &Cond, &RParen, &Stmt, &Epsilon]);
        // make(&Stmt, vec![&If, &LParen, &Cond, &RParen, &Stmt, &Else, &Stmt]);
        // make(&Stmt, vec![&While, &LParen, &Cond, &RParen, &Stmt]);
        // make(&Stmt, vec![&Break, &Semicolon]);
        // make(&Stmt, vec![&Continue, &Semicolon]);
        // make(&Stmt, vec![&Return, &Epsilon, &Semicolon]);
        // make(&Stmt, vec![&Return, &Exp, &Semicolon]);

        // // Exp -> AddExp
        // make(&Exp, vec![&AddExp]);

        // // Cond -> LOrExp
        // make(&Cond, vec![&LOrExp]);

        // // LVal -> Ident {'[' Exp ']'}
        // make(&LVal,  vec![&Ident, &LVal_]);
        // make(&LVal_, vec![&Epsilon]);
        // make(&LVal_, vec![&LVal_, &LBracket, &Exp, &RBracket]);

        // // PrimaryExp -> '(' Exp ')' | LVal | Number
        // make(&PrimaryExp, vec![&LParen, &Exp, &RParen]);
        // make(&PrimaryExp, vec![&LVal]);
        // make(&PrimaryExp, vec![&Number]);

        // // Number -> IntConst | FloatConst
        // make(&Number, vec![&IntConst]);
        // make(&Number, vec![&FloatConst]);

        // // UnaryExp -> PrimaryExp | Ident '(' [FuncRParams] ')' | UnaryOp UnaryExp
        // make(&UnaryExp,  vec![&PrimaryExp]);
        // make(&UnaryExp,  vec![&Ident, &LParen, &UnaryExp_, &RParen]);
        // make(&UnaryExp_, vec![&Epsilon]);
        // make(&UnaryExp_, vec![&FuncRParams]);
        // make(&UnaryExp,  vec![&UnaryOp, &UnaryExp]);

        // // UnaryOp -> '+' | '-' | '!'
        // make(&UnaryOp, vec![&Plus]);
        // make(&UnaryOp, vec![&Minus]);
        // make(&UnaryOp, vec![&Not]);

        // // FuncRParams -> Exp { ',' Exp }
        // make(&FuncRParams, vec![&Exp]);
        // make(&FuncRParams, vec![&FuncRParams, &Commo, &Exp]);

        // // MulExp -> UnaryExp | MulExp ('*' | '/' | '%') UnaryExp
        // make(&MulExp, vec![&UnaryExp]);
        // make(&MulExp, vec![&MulExp, &MulOp, &UnaryExp]);
        // make(&MulOp,  vec![&Star]);
        // make(&MulOp,  vec![&Div]);
        // make(&MulOp,  vec![&Mod]);

        // // AddExp -> MulExp | AddExp ('+' | '-') MulExp
        // make(&AddExp, vec![&MulExp]);
        // make(&AddExp, vec![&AddExp, &AddOp, &MulExp]);
        // make(&AddOp,  vec![&Plus]);
        // make(&AddOp,  vec![&Minus]);

        // // RelExp -> AddExp | RelExp ('<' | '>' | '<=' | '>=') AddExp
        // make(&RelExp, vec![&AddExp]);
        // make(&RelExp, vec![&RelExp, &RelOp, &AddExp]);
        // make(&RelOp,  vec![&Lt]);
        // make(&RelOp,  vec![&Gt]);
        // make(&RelOp,  vec![&Le]);
        // make(&RelOp,  vec![&Ge]);

        // // EqExp -> RelExp | EqExp ('==' | '!=') RelExp
        // make(&EqExp, vec![&RelExp]);
        // make(&EqExp, vec![&EqExp, &EqOp, &RelExp]);
        // make(&EqOp,  vec![&Eq]);
        // make(&EqOp,  vec![&Ne]);

        // // LAndExp -> EqExp | LAndExp '&&' EqExp
        // make(&LAndExp, vec![&EqExp]);
        // make(&LAndExp, vec![&LAndExp, &And, &EqExp]);

        // // LOrExp -> LAndExp | LOrExp '||' LAndExp
        // make(&LOrExp, vec![&LAndExp]);
        // make(&LOrExp, vec![&LOrExp, &Or, &LAndExp]);

        // // ConstExp -> AddExp
        // make(&ConstExp, vec![&AddExp]);

        // // Ident -> identifier-nondigit | identifier identifier-nondigit | identifier digit
        // make(&Ident, vec![&letter_A]);
        // make(&Ident, vec![&letter_B]);
        // make(&Ident, vec![&letter_C]);
        // make(&Ident, vec![&letter_D]);
        // make(&Ident, vec![&letter_E]);
        // make(&Ident, vec![&letter_F]);
        // make(&Ident, vec![&letter_G]);
        // make(&Ident, vec![&letter_H]);
        // make(&Ident, vec![&letter_I]);
        // make(&Ident, vec![&letter_J]);
        // make(&Ident, vec![&letter_K]);
        // make(&Ident, vec![&letter_L]);
        // make(&Ident, vec![&letter_M]);
        // make(&Ident, vec![&letter_N]);
        // make(&Ident, vec![&letter_O]);
        // make(&Ident, vec![&letter_P]);
        // make(&Ident, vec![&letter_Q]);
        // make(&Ident, vec![&letter_R]);
        // make(&Ident, vec![&letter_S]);
        // make(&Ident, vec![&letter_T]);
        // make(&Ident, vec![&letter_U]);
        // make(&Ident, vec![&letter_V]);
        // make(&Ident, vec![&letter_W]);
        // make(&Ident, vec![&letter_X]);
        // make(&Ident, vec![&letter_Y]);
        // make(&Ident, vec![&letter_Z]);
        // make(&Ident, vec![&letter_a]);
        // make(&Ident, vec![&letter_b]);
        // make(&Ident, vec![&letter_c]);
        // make(&Ident, vec![&letter_d]);
        // make(&Ident, vec![&letter_e]);
        // make(&Ident, vec![&letter_f]);
        // make(&Ident, vec![&letter_g]);
        // make(&Ident, vec![&letter_h]);
        // make(&Ident, vec![&letter_i]);
        // make(&Ident, vec![&letter_j]);
        // make(&Ident, vec![&letter_k]);
        // make(&Ident, vec![&letter_l]);
        // make(&Ident, vec![&letter_m]);
        // make(&Ident, vec![&letter_n]);
        // make(&Ident, vec![&letter_o]);
        // make(&Ident, vec![&letter_p]);
        // make(&Ident, vec![&letter_q]);
        // make(&Ident, vec![&letter_r]);
        // make(&Ident, vec![&letter_s]);
        // make(&Ident, vec![&letter_t]);
        // make(&Ident, vec![&letter_u]);
        // make(&Ident, vec![&letter_v]);
        // make(&Ident, vec![&letter_w]);
        // make(&Ident, vec![&letter_x]);
        // make(&Ident, vec![&letter_y]);
        // make(&Ident, vec![&letter_z]);
        // make(&Ident, vec![&digit_0]);
        // make(&Ident, vec![&digit_1]);
        // make(&Ident, vec![&digit_2]);
        // make(&Ident, vec![&digit_3]);
        // make(&Ident, vec![&digit_4]);
        // make(&Ident, vec![&digit_5]);
        // make(&Ident, vec![&digit_6]);
        // make(&Ident, vec![&digit_7]);
        // make(&Ident, vec![&digit_8]);
        // make(&Ident, vec![&digit_9]);

        sys_y_grammar.set_start_symbol(CompUnit.clone());

        std::sync::Mutex::new(sys_y_grammar)
    };
}
