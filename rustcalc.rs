
enum Operator{
    Plus,
    Minus,
    Mult,
    Div,
}

impl ToStr for Operator {
    fn to_str(&self) -> ~str {
        ~"Plus"
    }
}

struct IterStr {
    string: ~str,
    pos: ~int,
}

fn str_to_iter_str(input: &str) -> ~IterStr {
    let mut output = IterStr { string: input.to_str(), pos: ~0 };
    ~output
}

fn parens<T>(input: &str, p: &fn(&str) -> ~T) -> Option<~T> {
    let mut iter_str = str_to_iter_str(input); 
    match iter_str {
        ~IterStr{string: ~"()", pos: _} => Some(p(input)),
        ~IterStr{string: ~"(", pos: _}  => None,
        _                             => None,
    }
}

fn parse_op(input: &str) -> ~Operator {
    match input {
       _ => ~Plus
    }

}

fn main() {
    io::stdout().write_str("Prompt> ");
    let in = io::stdin().read_line();
    let out = parens(in, parse_op);
    match out {
        Some(o) => io::stdout().write_line((*o).to_str()),
        None    => io::stdout().write_line("WAT"),
    }
    ()
}