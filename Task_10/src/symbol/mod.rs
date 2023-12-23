use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Symbol {
    VerticalPipe,
    HorizontalPipe,
    NEPipe,
    NWPipe,
    SWPipe,
    SEPipe,
    Ground,
    Start,
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Symbol::VerticalPipe),
            "-" => Ok(Symbol::HorizontalPipe),
            "L" => Ok(Symbol::NEPipe),
            "J" => Ok(Symbol::NWPipe),
            "7" => Ok(Symbol::SWPipe),
            "F" => Ok(Symbol::SEPipe),
            "." => Ok(Symbol::Ground),
            "S" => Ok(Symbol::Start),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_representation = match self {
            Symbol::VerticalPipe => "|",
            Symbol::HorizontalPipe => "-",
            Symbol::NEPipe => "L",
            Symbol::NWPipe => "J",
            Symbol::SWPipe => "7",
            Symbol::SEPipe => "F",
            Symbol::Ground => ".",
            Symbol::Start => "S",
        };
        write!(f, "{}", str_representation)
    }
}