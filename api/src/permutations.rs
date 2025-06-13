/// Six permutation patterns for any three terms
#[derive(Debug, Clone)]
pub struct Permutation<T> {
    pub name: String,
    pub pattern: [usize; 3],
    pub terms: [T; 3],
}

impl<T: Clone> Permutation<T> {
    pub fn new(name: String, pattern: [usize; 3], terms: [T; 3]) -> Self {
        Self { name, pattern, terms }
    }
    
    pub fn ordered_terms(&self) -> [T; 3] {
        [
            self.terms[self.pattern[0]].clone(),
            self.terms[self.pattern[1]].clone(),
            self.terms[self.pattern[2]].clone(),
        ]
    }
}

/// Set of all six permutation patterns
#[derive(Debug, Clone)]
pub struct PermutationSet<T> {
    original_terms: [T; 3],
    permutations: Vec<Permutation<T>>,
}

impl<T: Clone> PermutationSet<T> {
    pub fn new(terms: [T; 3]) -> Self {
        let permutations = vec![
            Permutation::new("Expansion".to_string(), [0, 1, 2], terms.clone()),
            Permutation::new("Interaction".to_string(), [0, 2, 1], terms.clone()),
            Permutation::new("Concentration".to_string(), [1, 0, 2], terms.clone()),
            Permutation::new("Identity".to_string(), [1, 2, 0], terms.clone()),
            Permutation::new("Order".to_string(), [2, 0, 1], terms.clone()),
            Permutation::new("Freedom".to_string(), [2, 1, 0], terms.clone()),
        ];
        
        Self {
            original_terms: terms,
            permutations,
        }
    }
    
    pub fn permutations(&self) -> &[Permutation<T>] {
        &self.permutations
    }
    
    pub fn original_terms(&self) -> &[T; 3] {
        &self.original_terms
    }
} 