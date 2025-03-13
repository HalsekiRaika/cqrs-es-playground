pub struct Context {
    pub(in crate::process) seq: i64,
}

impl Context {
    pub fn new(seq: i64) -> Self {
        Self {
            seq
        }
    }
}

impl Context {
    pub fn seq(&self) -> i64 {
        self.seq
    }
}
