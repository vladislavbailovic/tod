#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Priority {
    Normal,
    High(usize),
}

impl Default for Priority {
    fn default() -> Self {
        Self::Normal
    }
}

impl From<usize> for Priority {
    fn from(p: usize) -> Self {
        if p > 0 {
            return Self::High(p);
        }
        Self::Normal
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_is_normal() {
        assert!(Priority::Normal == Priority::default());
    }

    #[test]
    fn test_normal_from_zero() {
        assert!(Priority::Normal == Priority::from(0));
    }

    #[test]
    fn test_high_priority() {
        assert!(Priority::High(1) == Priority::from(1));
        assert!(Priority::High(2) == Priority::from(2));
    }

    #[test]
    fn high_is_more_urgent_than_normal() {
        assert!(Priority::High(1) > Priority::Normal);
    }

    #[test]
    fn higher_priority_is_more_urgent_than_lower_priority() {
        assert!(Priority::High(2) > Priority::High(1));
    }
}
