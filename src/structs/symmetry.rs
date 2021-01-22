#![allow(dead_code)]
use super::unit_cell::*;
use crate::reference_tables;
use crate::transformation::*;

#[derive(Debug, Clone)]
/// A Space group of a crystal
pub struct Symmetry {
    /// The fully qualified Herman Mauguin symbol for the space group
    symbol: String,
    /// The index of this symbol in Int. Crys. Handbook Vol A 2016
    index: usize,
}

impl Symmetry {
    /// Create a new Symmetry based on a fully qualified Herman Mauguin symbol
    pub fn new(symbol: &str) -> Option<Self> {
        reference_tables::get_index_for_symbol(symbol.trim()).map(|index| Symmetry {
            symbol: symbol.trim().to_string(),
            index,
        })
    }

    /// Create a new Symmetry based on the index of a symbol in Int. Crys. Handbook Vol A 2016
    pub fn from_index(index: usize) -> Option<Self> {
        reference_tables::get_symbol_for_index(index).map(|s| Symmetry {
            symbol: s.to_string(),
            index,
        })
    }

    /// Get the fully qualified Herman Mauguin symbol for the space group
    pub fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    /// Get the Z value, the number of polymeric sub units in a unit cell, for this space group
    #[allow(clippy::unwrap_used)]
    pub fn z(&self) -> usize {
        reference_tables::get_transformation(self.index)
            .unwrap()
            .len()
            + 1
    }

    /// Get the index of this space group in Int. Crys. Handbook Vol A 2016
    pub fn index(&self) -> usize {
        self.index
    }

    /// Get the transformations for this space group needed to fill the unit cell.
    /// The first transformation is always an identity transformation.
    /// The translation is fractional to the unit cell size.
    #[allow(clippy::unwrap_used)]
    pub fn transformations(&self) -> Vec<TransformationMatrix> {
        let matrices = reference_tables::get_transformation(self.index).unwrap();
        let mut output = Vec::with_capacity(matrices.len() + 1);
        output.push(TransformationMatrix::identity());
        for matrix in matrices {
            output.push(TransformationMatrix::from_matrix(*matrix));
        }
        output
    }

    /// Get the transformations for this space group needed to fill the unit cell.
    /// The first transformation is always an identity transformation.
    /// The translation is in Å.
    #[allow(clippy::unwrap_used)]
    pub fn transformations_absolute(&self, unit_cell: &UnitCell) -> Vec<TransformationMatrix> {
        let matrices = reference_tables::get_transformation(self.index).unwrap();
        let mut output = Vec::with_capacity(matrices.len() + 1);
        output.push(TransformationMatrix::identity());
        for matrix in matrices {
            let mut ma = TransformationMatrix::from_matrix(*matrix);
            ma.multiply_translation(unit_cell.size());
            output.push(ma);
        }
        output
    }
}

impl PartialEq for Symmetry {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Symmetry {}

#[cfg(test)]
mod tests {
    use super::Symmetry;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn both_creations() {
        let a = Symmetry::new("P 21 21 21").unwrap();
        let b = Symmetry::from_index(19).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.z(), a.transformations().len());
        assert_eq!(
            4,
            a.transformations_absolute(&crate::UnitCell::new(1.0, 1.0, 1.0, 90.0, 90.0, 90.0))
                .len()
        );
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn symbol_invariant() {
        let a = Symmetry::new("P 21 21 21").unwrap();
        assert_eq!(a.symbol(), "P 21 21 21")
    }
}
