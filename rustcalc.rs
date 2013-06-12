struct IterStr {
    string: ~str,
    pos: uint,
}

struct ParseResult<T> {
    iter: ~IterStr,      /* Position of the first unconsumed token. */
    parsed: ~Option<T>, /* The parse result. */
}

fn consume(i: &IterStr) -> (~IterStr, ~Option<char>) {
  let c = if (i.pos > i.string.len()) {
            ~None
          } else {
            ~Some(i.string.char_at(i.pos))
          };

  (~IterStr {string: i.string.to_owned(), pos: i.pos + 1}, c)
}

fn str_to_iter_str(ss: ~str) -> ~IterStr {
    ~IterStr { string: ss, pos: 0 }
}

fn parse_chr(input: &IterStr, c: char) -> ~ParseResult<~char> {
  let (i, sc) = consume(input);
  let p = match sc {
    ~None => ~None,
    ~Some(x) => {
      if (c == x) {
        ~Some(~x)
      }
      else
      {
        ~None
      }
    }
  };

  ~ParseResult { iter: i, parsed: p }
}

fn parse_parens<T>(input: &IterStr, p: &fn(&IterStr) -> ~ParseResult<T>) -> ~ParseResult<T> {
  fn mk_res<T>(iter: ~IterStr, parsed: ~Option<T>) -> ~ParseResult<T> {
    ~ParseResult {
      iter: iter,
      parsed: parsed
    }
  }

  let ~ParseResult { iter: lp_iter, parsed: lp_parsed } = parse_chr(input, '(');
  let n = ~None;

  match lp_parsed {
    ~None => mk_res(lp_iter, n),
    _ => {
      let ~ParseResult { iter: r_iter, parsed: r_parsed } = p(lp_iter);

      match r_parsed {
        ~None => mk_res(r_iter, n),
        _ => {
          let ~ParseResult { iter: rp_iter, parsed: rp_parsed } = parse_chr(r_iter, ')');

          match rp_parsed {
            ~None => mk_res(rp_iter, n),
            _ => mk_res(rp_iter, r_parsed),
          }
        }
      }
    }
  }
}

/* Attempt to apply the parser. If the parse fails, do not consume any input. */
fn try<T>(input: &IterStr, p: &fn(&IterStr) -> ~ParseResult<T>) -> ~ParseResult<T> {
  let orig_pos = input.pos;
  let r = p(input);

  match (r.parsed) {
    ~Some(*) => r,
    ~None => ~ParseResult {
      iter: ~IterStr {
        string: r.iter.string.to_owned(),
        pos: orig_pos
      },
      parsed: ~None
    }
  }
}

fn main() {
  io::stdout().write_str("Prompt> ");

  let in = io::stdin().read_line();
  let it: ~IterStr = str_to_iter_str(in);

  let res = parse_parens(it, |itr| parse_chr(itr, 'X'));

  match *res.parsed {
    Some(*) => io::stdout().write_line("PARSED"),
    None    => io::stdout().write_line("NOPE"),
  }
}
