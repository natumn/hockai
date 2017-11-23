use std::env;
use std::process;
use std::io::{self, Write};

struct Token {
    kind:               Kind,
    text[TEXT_SIZE+1]:  char,
    intVal:             int,
}

enum Kind {
    Lparen,
    Rparen,
    Plus,
    Minus,
    Multi,
    Divi,
    Equal,
    NotEq,
    Less,
    LessEq,
    Great,
    GreatEq,
    SngQ,
    DblQ,
    Assgin,
    Semicolon,
    StringTkn,
    Letter,
    Digit,
    NulKind,
    EofTkn,
    Others,
    END_list,
}

trait lexer {
    fn initChTyp(&self) -> vec<_>;
    fn nextTkn(&self) -> Token;
    fn nextCh(&self) -> i32;
    fn is_ope2(&self, c1: i32, c2: i32 -> i32;
    fn set_kind(&self) -> Token;
    fn err_exit<F>(f: F) -> Reult<i32>;
}

impl lexer for Token {
    fn nextTkn() -> Token {
        let mut tkn:Token  = {NulKind, "", 0};
        
        while (isspace(ch)) { ch = nextCh(); }
        if (ch == EOF) {
            tkn.kind = EofTkn;
            return tkn;
        }
        match ctyp[ch] {
            Letter => {
            
            }
        }
    }
}

fn main() {
    let args: Vec<_>  = env::args.collect();
    
    process::exit(match run_compile() {
        Ok(_) => 0,
        Error(err) => {
            writeln!(io::stderr(),"error: {:?}", err).unwrap();
            1
        }
    });
}

fn run_compile() -> Result<sucess, err> {
    if let Err(err) = initChTyp() {
        return err
    };

    Ok(())
}

fn initChTyp() {
    for i in 0..256 { ctyp[i] = Others; } 
    for i in 0..10 { ctyp[i] = Digit; }
    // アルファベットの格納.アルファベットをiter::Stepの要素に使えないっぽい.Rustにcharからasciiコード二変換できない可能性がある。
    for i in 'A' as u32 ..'Z' as u32 {
        ctyp[i] = Letter; 
    }
    
    ctyp['_' as u32] = Letter;
    ctyp['=' as u32] = Assgin;
    ctyp['(' as u32] = Lparen;
    ctyp[')' as u32] = Rparen;
    ctyp['<' as u32] = Less;
    ctyp['>' as u32] = Great;
    ctyp['+' as u32] = Plus;
    ctyp['-' as u32] = Minus;
    ctyp['*' as u32] = Multi;
    ctyp['/' as u32] = Divi;
    ctyp['¥' as u32] = SngQ;
    ctyp['"' as u32] = DblQ;
    ctyp[';' as u32] = Semicolon;
}

mod test {
    #[test]
    fn initChTyp_test() {
        assert!(initChyTyp() == ctyp[]);
    }
}
