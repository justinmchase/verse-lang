use crate::ast::{Expression, Pattern};
use crate::runtime::{
  Context, Library, Match, Name, Namespace, NativePatternHandler, RuntimeError, Scope, Type, Value,
};
use regex::Regex;
use semver::Version;
use std::collections::HashMap;
use std::rc::Rc;

// Other, Control
fn cc() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{cc}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Other, Format
fn cf() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{Format}]$").unwrap(); // cf
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Other, Not Assigned (no characters in the file have this property)
fn cn() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{cn}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Other, Private Use
fn co() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{co}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Other, Surrogate
fn cs() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{cs}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Letter, Cased
fn lc() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{lc}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Letter, Lowercase
fn ll() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{ll}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Letter, Modifier
fn lm() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{lm}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Letter, Other
fn lo() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{lo}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Letter, Titlecase
fn lt() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{lt}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Letter, Uppercase
fn lu() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{lu}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Mark, Spacing Combining
fn mc() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{mc}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Mark, Enclosing
fn me() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{me}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Mark, Nonspacing
fn mn() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{mn}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Number, Decimal Digit
fn nd() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{nd}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Number, Letter
fn nl() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{nl}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Number, Other
fn no() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{no}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Punctuation, Connector
fn pc() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{pc}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Punctuation, Dash
fn pd() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{pd}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Punctuation, Close
fn pe() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{pe}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Punctuation, Final quote (may behave like Ps or Pe depending on usage)
fn pf() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{pf}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Punctuation, Initial quote (may behave like Ps or Pe depending on usage)
fn pi() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{pi}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Punctuation, Other
fn po() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{po}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Punctuation, Open
fn ps() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{ps}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Symbol, Currency
fn sc() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{Currency_Symbol}]$").unwrap(); // sc
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Symbol, Modifier
fn sk() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{sk}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Symbol, Math
fn sm() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{sm}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Symbol, Other
fn so() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{so}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Separator, Line
fn zl() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{zl}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Separator, Paragraph
fn zp() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{zp}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

// Separator, Space
fn zs() -> NativePatternHandler {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[\p{zs}]$").unwrap();
  }
  return |start: Scope| unicode_category(start, &RE);
}

fn unicode_category(start: Scope, re: &Regex) -> Result<Match, RuntimeError> {
  match start.next() {
    Some(s) => match s.value.clone() {
      Value::String(v) => match re.is_match(v.as_str()) {
        true => Ok(Match::ok(s.value.clone(), start, s)),
        _ => Ok(Match::fail(start)),
      },
      _ => Ok(Match::fail(start)),
    },
    _ => Ok(Match::fail(start)),
  }
}

pub fn unicode(
  mut libraries: HashMap<(Name, Version), Rc<Library>>,
) -> Result<HashMap<(Name, Version), Rc<Library>>, RuntimeError> {
  let name = Name::parse("unicode");
  let version = Version::parse("1.0.0").unwrap();
  let lib = Library::new(&name, &version);
  lib.add_module(
    &Namespace::parse("categories.Cc"),
    Expression::Pattern(Box::new(Pattern::Native(cc()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Cf"),
    Expression::Pattern(Box::new(Pattern::Native(cf()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Cn"),
    Expression::Pattern(Box::new(Pattern::Native(cn()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Co"),
    Expression::Pattern(Box::new(Pattern::Native(co()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Cs"),
    Expression::Pattern(Box::new(Pattern::Native(cs()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.LC"),
    Expression::Pattern(Box::new(Pattern::Native(lc()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Ll"),
    Expression::Pattern(Box::new(Pattern::Native(ll()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Lm"),
    Expression::Pattern(Box::new(Pattern::Native(lm()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Lo"),
    Expression::Pattern(Box::new(Pattern::Native(lo()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Lt"),
    Expression::Pattern(Box::new(Pattern::Native(lt()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Lu"),
    Expression::Pattern(Box::new(Pattern::Native(lu()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Mc"),
    Expression::Pattern(Box::new(Pattern::Native(mc()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Me"),
    Expression::Pattern(Box::new(Pattern::Native(me()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Mn"),
    Expression::Pattern(Box::new(Pattern::Native(mn()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Nd"),
    Expression::Pattern(Box::new(Pattern::Native(nd()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Nl"),
    Expression::Pattern(Box::new(Pattern::Native(nl()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.No"),
    Expression::Pattern(Box::new(Pattern::Native(no()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Pc"),
    Expression::Pattern(Box::new(Pattern::Native(pc()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Pd"),
    Expression::Pattern(Box::new(Pattern::Native(pd()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Pe"),
    Expression::Pattern(Box::new(Pattern::Native(pe()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Pf"),
    Expression::Pattern(Box::new(Pattern::Native(pf()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Pi"),
    Expression::Pattern(Box::new(Pattern::Native(pi()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Po"),
    Expression::Pattern(Box::new(Pattern::Native(po()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Ps"),
    Expression::Pattern(Box::new(Pattern::Native(ps()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Sc"),
    Expression::Pattern(Box::new(Pattern::Native(sc()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Sk"),
    Expression::Pattern(Box::new(Pattern::Native(sk()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Sm"),
    Expression::Pattern(Box::new(Pattern::Native(sm()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.So"),
    Expression::Pattern(Box::new(Pattern::Native(so()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Zl"),
    Expression::Pattern(Box::new(Pattern::Native(zl()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Zp"),
    Expression::Pattern(Box::new(Pattern::Native(zp()))),
  )?;
  lib.add_module(
    &Namespace::parse("categories.Zs"),
    Expression::Pattern(Box::new(Pattern::Native(zs()))),
  )?;

  libraries.insert((name, version), Rc::new(lib));
  Ok(libraries)
}

#[cfg(test)]
mod tests {
  use crate::ast::{Expression, ImportExpression, Pattern};
  use crate::runtime::resolvers::MemoryResolver;
  use crate::runtime::{Library, Name, Namespace, RuntimeError, Value, Verse};
  use semver::Version;
  use std::rc::Rc;

  #[test]
  fn test_import_text_module() -> Result<(), RuntimeError> {
    // Ll = import text@1.0.0/classes.Ll
    // Ll("a")

    // Load up each unicode character class and test it against a single character in that class range
    let data = vec![
      ("Cc", "\u{0000}"),
      ("Cf", "\u{00AD}"),
      // ("Cn", ""), // no characters in the file have this property
      ("Co", "\u{E000}"),
      // ("Cs", ""), // can't have these characters in rust strings
      // ("LC", ""), // no characters in the file have this property
      ("Ll", "\u{0061}"),
      ("Lm", "\u{02B0}"),
      ("Lo", "\u{00AA}"),
      ("Lt", "\u{01C5}"),
      ("Lu", "\u{0041}"),
      ("Mc", "\u{0903}"),
      ("Me", "\u{0488}"),
      ("Mn", "\u{0300}"),
      ("Nd", "\u{0030}"),
      ("Nl", "\u{16EE}"),
      ("No", "\u{00B2}"),
      ("Pc", "\u{005F}"),
      ("Pd", "\u{002D}"),
      ("Pe", "\u{0029}"),
      ("Pf", "\u{00BB}"),
      ("Pi", "\u{00AB}"),
      ("Po", "\u{0021}"),
      ("Ps", "\u{0028}"),
      ("Sc", "\u{0024}"),
      ("Sk", "\u{005E}"),
      ("Sm", "\u{002B}"),
      ("So", "\u{00A6}"),
      ("Zl", "\u{2028}"),
      ("Zp", "\u{2029}"),
      ("Zs", "\u{0020}"),
    ];

    for (category, input) in data {
      println!("categories.{}", category);
      let main = Expression::Block(vec![
        Box::new(Expression::Destructure(
          Box::new(Pattern::Var(String::from("P"), Box::new(Pattern::Any))),
          Box::new(Expression::Import(ImportExpression::Global(
            Name::parse("unicode"),
            Version::parse("1.0.0").unwrap(),
            Some(Namespace::parse(
              format!("categories.{}", category).as_str(),
            )),
          ))),
        )),
        Box::new(Expression::Call(
          Box::new(Expression::Ref(String::from("P"))),
          Some(Box::new(Expression::Value(Value::String(String::from(
            input,
          ))))),
        )),
      ]);

      let lib = Library::default();
      lib.add_main(main)?;

      let v = Rc::new(Verse::new(
        Rc::new(lib),
        vec![Box::new(MemoryResolver::core()?)],
      ));

      let res = Verse::run(v, None)?;

      println!("{:?}", res);
      assert_eq!(res, Value::String(String::from(input)));
    }

    Ok(())
  }
}
