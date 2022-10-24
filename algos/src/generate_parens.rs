/// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
/// https://leetcode.com/problems/generate-parentheses/
use std::collections::HashSet;

type OpenParens = Vec<Paren>;

#[derive(Debug, Default, Clone)]
struct Paren(OpenParens);

impl Paren {
    pub fn find_merge_possibilities(self, to_merge: &Paren) -> OpenParens {
        let mut possibilities = OpenParens::new();

        for counter in 0..(self.0.len() + 1) {
            let mut this = self.clone();
            this.0.insert(counter, to_merge.to_owned());
            possibilities.push(this);
        }

        for (index, paren) in self.0.clone().into_iter().enumerate() {
            for possibility in paren.find_merge_possibilities(to_merge) {
                let mut this = self.clone();
                this.0[index] = possibility;
                possibilities.push(this);
            }
        }

        possibilities
    }
}

impl ToString for Paren {
    fn to_string(&self) -> String {
        if self.0.is_empty() {
            return "()".to_string();
        }

        let mut buff = String::new();

        for paren in &self.0 {
            buff += &paren.to_string();
        }

        format!("({})", buff)
    }
}

fn merge(mut parens: OpenParens, results: &mut Vec<OpenParens>) {
    if parens.len() <= 1 {
        return;
    }

    let last = parens.pop().unwrap();

    for (index, paren) in parens.clone().into_iter().enumerate() {
        for possibility in paren.find_merge_possibilities(&last) {
            let mut parens = parens.clone();
            parens[index] = possibility;
            results.push(parens.clone());
            eprintln!("{:?}", parens);
            merge(parens, results);
        }
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let parens = vec![Paren::default(); n as usize];
    let mut results = vec![parens.clone()];

    merge(parens, &mut results);

    let mut uniques = HashSet::new();

    for result in results {
        let mut buff = String::new();

        for paren in result {
            buff += &paren.to_string();
        }

        uniques.insert(buff);
    }

    uniques.iter().cloned().collect()
}

#[test]
fn generate_parens() {
    panic!("{:?}", generate_parenthesis(3));
}
