//! Layout Anchors - Type-safe AutoLayout constraints

/// Represents a horizontal layout anchor (X-axis)
#[derive(Debug, Clone)]
pub struct LayoutAnchorX {
    pub id: String,
}

impl LayoutAnchorX {
    pub fn new(id: &str) -> Self {
        LayoutAnchorX {
            id: id.to_string(),
        }
    }

    /// Create a constraint equal to another anchor with offset
    pub fn constraint_equal_to(&self, other: &LayoutAnchorX) -> LayoutConstraint {
        LayoutConstraint {
            id: format!("{}_eq_{}", self.id, other.id),
            relation: ConstraintRelation::Equal,
            constant: 0.0,
            priority: 1000.0,
        }
    }
}

/// Represents a vertical layout anchor (Y-axis)
#[derive(Debug, Clone)]
pub struct LayoutAnchorY {
    pub id: String,
}

impl LayoutAnchorY {
    pub fn new(id: &str) -> Self {
        LayoutAnchorY {
            id: id.to_string(),
        }
    }

    /// Create a constraint equal to another anchor with offset
    pub fn constraint_equal_to(&self, other: &LayoutAnchorY) -> LayoutConstraint {
        LayoutConstraint {
            id: format!("{}_eq_{}", self.id, other.id),
            relation: ConstraintRelation::Equal,
            constant: 0.0,
            priority: 1000.0,
        }
    }
}

/// Represents a dimension layout anchor (width/height)
#[derive(Debug, Clone)]
pub struct LayoutAnchorDimension {
    pub id: String,
}

impl LayoutAnchorDimension {
    pub fn new(id: &str) -> Self {
        LayoutAnchorDimension {
            id: id.to_string(),
        }
    }

    /// Create a constraint equal to a constant value
    pub fn constraint_equal_to_constant(&self, constant: f64) -> LayoutConstraint {
        LayoutConstraint {
            id: format!("{}_const_{}", self.id, constant as i32),
            relation: ConstraintRelation::Equal,
            constant,
            priority: 1000.0,
        }
    }
}

/// Constraint relation types
#[derive(Debug, Clone, Copy)]
pub enum ConstraintRelation {
    Equal,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// A layout constraint
#[derive(Debug, Clone)]
pub struct LayoutConstraint {
    pub id: String,
    pub relation: ConstraintRelation,
    pub constant: f64,
    pub priority: f64,
}

impl LayoutConstraint {
    /// Set the offset (constant) for this constraint
    pub fn offset(mut self, offset: f64) -> Self {
        self.constant = offset;
        self
    }

    /// Set the priority for this constraint
    pub fn priority(mut self, priority: f64) -> Self {
        self.priority = priority;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_anchor_x() {
        let anchor = LayoutAnchorX::new("test");
        assert_eq!(anchor.id, "test");
    }

    #[test]
    fn test_layout_anchor_y() {
        let anchor = LayoutAnchorY::new("test");
        assert_eq!(anchor.id, "test");
    }

    #[test]
    fn test_layout_anchor_dimension() {
        let anchor = LayoutAnchorDimension::new("test");
        assert_eq!(anchor.id, "test");
    }

    #[test]
    fn test_constraint_creation() {
        let anchor1 = LayoutAnchorX::new("a");
        let anchor2 = LayoutAnchorX::new("b");
        let constraint = anchor1.constraint_equal_to(&anchor2);
        assert!(constraint.id.contains("a_eq_b"));
    }

    #[test]
    fn test_constraint_offset() {
        let anchor = LayoutAnchorDimension::new("width");
        let constraint = anchor.constraint_equal_to_constant(100.0).offset(10.0);
        assert_eq!(constraint.constant, 10.0);
    }

    #[test]
    fn test_constraint_priority() {
        let anchor = LayoutAnchorDimension::new("width");
        let constraint = anchor.constraint_equal_to_constant(100.0).priority(800.0);
        assert_eq!(constraint.priority, 800.0);
    }
}
