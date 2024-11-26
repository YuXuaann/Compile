#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Symbol {
    pub name: String,
    pub is_terminal: bool,
}

impl Symbol {
    pub fn epsilon() -> Self {
        Symbol {
            name: String::from(""),
            is_terminal: true,
        }
    }
    pub fn from(name: &str, is_terminal: bool) -> Self {
        Symbol {
            name: name.to_string(),
            is_terminal,
        }
    }
    pub fn is_epsilon(&self) -> bool {
        self.name == "" && self.is_terminal
    }
    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_epsilon() {
            write!(f, "ε")
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_epsilon() {
            write!(f, "ε")
        } else {
            write!(f, "{}", self.name)
        }
    }
}
