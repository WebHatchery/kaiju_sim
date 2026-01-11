//! Lineage and ancestry tracking for kaiju.

use serde::{Deserialize, Serialize};

use super::types::*;

/// Notable achievement for lineage highlights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageHighlight {
    /// Token ID of notable ancestor
    pub token_id: TokenId,

    /// Ancestor's name
    pub name: String,

    /// Achievement description
    pub achievement: String,

    /// Generation of ancestor
    pub generation: Generation,
}

/// Recursive lineage tree structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lineage {
    /// Token ID of this kaiju
    pub kaiju_id: TokenId,

    /// Kaiju name
    pub name: String,

    /// Generation number
    pub generation: Generation,

    /// Parent lineages (None for generation 0)
    pub parents: Option<Box<(Lineage, Lineage)>>,

    /// Notable ancestors (champions, etc.)
    pub notable_ancestors: Vec<LineageHighlight>,
}

impl Lineage {
    /// Create lineage for generation 0 kaiju
    pub fn new_wild(token_id: TokenId, name: String) -> Self {
        Self {
            kaiju_id: token_id,
            name,
            generation: 0,
            parents: None,
            notable_ancestors: Vec::new(),
        }
    }

    /// Calculate total lineage depth
    pub fn depth(&self) -> u32 {
        match &self.parents {
            None => 1,
            Some(parents) => 1 + parents.0.depth().max(parents.1.depth()),
        }
    }

    /// Check if lineage contains a specific ancestor
    pub fn has_ancestor(&self, token_id: TokenId) -> bool {
        if self.kaiju_id == token_id {
            return true;
        }

        match &self.parents {
            None => false,
            Some(parents) => parents.0.has_ancestor(token_id) || parents.1.has_ancestor(token_id),
        }
    }

    /// Validate that two kaiju are not directly related (prevent incest)
    pub fn can_breed_with(lineage_a: &Lineage, lineage_b: &Lineage) -> bool {
        // Cannot breed with self
        if lineage_a.kaiju_id == lineage_b.kaiju_id {
            return false;
        }

        // Cannot breed parent with child
        if lineage_a.has_ancestor(lineage_b.kaiju_id)
            || lineage_b.has_ancestor(lineage_a.kaiju_id)
        {
            return false;
        }

        // Cannot breed siblings (same parents)
        match (&lineage_a.parents, &lineage_b.parents) {
            (Some(parents_a), Some(parents_b)) => {
                parents_a.0.kaiju_id != parents_b.0.kaiju_id
                    && parents_a.1.kaiju_id != parents_b.1.kaiju_id
            }
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wild_lineage() {
        let lineage = Lineage::new_wild(1, "Primordial".to_string());
        assert_eq!(lineage.generation, 0);
        assert_eq!(lineage.depth(), 1);
        assert!(lineage.parents.is_none());
    }

    #[test]
    fn test_incest_prevention() {
        let parent_a = Lineage::new_wild(1, "Parent A".to_string());
        let parent_b = Lineage::new_wild(2, "Parent B".to_string());

        // Parents can breed
        assert!(Lineage::can_breed_with(&parent_a, &parent_b));

        // Cannot breed with self
        assert!(!Lineage::can_breed_with(&parent_a, &parent_a));
    }
}
