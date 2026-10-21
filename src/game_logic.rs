struct Board {
    height: i32,
    width: i32,
}

struct Rule {
    live: i32,
    die: i32,
}

pub struct Sim {
    board: Board,
    rule: Rule,
}

impl Sim {
    pub fn new(height: i32, width: i32, live: i32, die: i32) -> Sim {
        Sim {
            board: Board { height, width },
            rule: Rule { live, die },
        }
    }
}
