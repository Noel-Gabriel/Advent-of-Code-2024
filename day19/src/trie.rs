use std::fmt;

pub struct Trie {
    nodes: [Option<Box::<Trie>>; 26],
    pub found: bool,
}

impl Trie {
    pub fn new(words: &Vec<String>) -> Self {
        let mut root = Trie::empty_node(); 
        words
            .iter()
            .map(|w| w.as_bytes())
            .for_each(|bytes| { root.insert(&bytes, 0); });
        root 
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut trie = self;
        for w in word.chars() {
            let c = w as usize - 'a' as usize;
            if let Some(sub) = &trie.nodes[c] {
                trie = sub.as_ref();
            } else { return false }
        }
        trie.found
    }

    pub fn next(&self, c: char) -> Option<&Trie> {
        match &self.nodes[c as usize - 'a' as usize] {
            Some(sub) => Some(sub.as_ref()),
            None      => None,
        }
    }

    fn empty_node() -> Self {
        Self {
            nodes: [const { None }; 26],
            found: false,
        }
    }

    fn insert(&mut self, bytes: &[u8],  idx: usize) {
        if idx == bytes.len() { self.found = true; return }
        let c = (bytes[idx] - b'a') as usize;
        match &mut self.nodes[c] {
            Some(sub) => sub.insert(bytes, idx + 1),
            None      => {
                self.nodes[c] = Some(Box::<Trie>::new(Trie::empty_node()));
                self.nodes[c]
                    .as_mut()
                    .unwrap()
                    .insert(bytes, idx + 1);
            }
        }
    }
}

impl fmt::Display for Trie {    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[").unwrap();
        for i in 0..26 {
            if let Some(s) = &self.nodes[i] {
                write!(f, "({}, {}, {}), ", (i as u8 + 'a' as u8) as char, s.found, s).unwrap();
            }
        }
        write!(f, "] ")
    }
}
