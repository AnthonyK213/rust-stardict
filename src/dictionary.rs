use crate::{dict, idx, ifo};

use dict::Dict;
use idx::Idx;
use ifo::Ifo;

pub struct Dictionary {
    idx: Idx,
    dict: Dict,
    ifo: Ifo,
}

impl Dictionary {
    pub fn new(dict: Dict, idx: Idx, ifo: Ifo) -> Self {
        Self { idx, dict, ifo }
    }

    pub fn search(&self, word: String) -> Option<String> {
        if let Some(item) = self.idx.index(&word) {
            Some(self.dict.get(item))
        } else {
            None
        }
    }
}
