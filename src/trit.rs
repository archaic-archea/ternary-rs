#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
pub enum Trit {
    /// Positive, +1
    Pos,
    /// Neutral, 0
    Neu,
    /// Negative, -2
    Neg
}

impl Trit {
    pub fn not(&self) -> Self {
        match self {
            Self::Pos => Self::Neg,
            Self::Neu => Self::Neu,
            Self::Neg => Self::Pos
        }
    }

    pub fn and(&self, rhs: Self) -> Self {
        if rhs > *self {
            return rhs;
        } else {
            return *self;
        }
    }

    pub fn or(&self, rhs: Self) -> Self {
        if rhs < *self {
            return rhs;
        } else {
            return *self;
        }
    }

    pub fn xor(&self, rhs: Self) -> Self {
        if self.is_neu() || rhs.is_neu() {
            return Self::Neu;
        } else if *self == rhs {
            return Self::Neg;
        } else {
            return Self::Pos;
        }
    }

    pub fn is_neu(&self) -> bool {
        *self == Self::Neu
    }
}

impl std::cmp::Ord for Trit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if (*self == Self::Pos) && (*other != Self::Pos) {
            return std::cmp::Ordering::Greater;
        } else if self == other {
            return std::cmp::Ordering::Equal;
        } else {
            return std::cmp::Ordering::Less;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trit;

    #[test]
    fn not() {
        let inputs = vec![
            Trit::Neg,
            Trit::Neu,
            Trit::Pos
        ];
        let outputs = vec![
            Trit::Pos,
            Trit::Neu,
            Trit::Neg
        ];
        
        let inputs: Vec<Trit> = inputs.iter().map(|entry| {
            entry.not()
        }).collect();

        assert_eq!(inputs, outputs)
    }

    #[test]
    fn and() {
        let inputs = vec![
            [Trit::Neg, Trit::Neg],
            [Trit::Neg, Trit::Neu],
            [Trit::Neg, Trit::Pos],
            [Trit::Neu, Trit::Neg],
            [Trit::Neu, Trit::Neu],
            [Trit::Neu, Trit::Pos],
            [Trit::Pos, Trit::Neg],
            [Trit::Pos, Trit::Neu],
            [Trit::Pos, Trit::Pos],
        ];
        let outputs = vec![
            Trit::Neg,
            Trit::Neg,
            Trit::Neg,
            Trit::Neg,
            Trit::Neu,
            Trit::Neu,
            Trit::Neg,
            Trit::Neu,
            Trit::Pos,
        ];
        
        let inputs: Vec<Trit> = inputs.iter().map(|entry| {
            entry[0].and(entry[1])
        }).collect();

        assert_eq!(outputs, inputs);
    }

    #[test]
    fn or() {
        let inputs = vec![
            [Trit::Neg, Trit::Neg],
            [Trit::Neg, Trit::Neu],
            [Trit::Neg, Trit::Pos],
            [Trit::Neu, Trit::Neg],
            [Trit::Neu, Trit::Neu],
            [Trit::Neu, Trit::Pos],
            [Trit::Pos, Trit::Neg],
            [Trit::Pos, Trit::Neu],
            [Trit::Pos, Trit::Pos],
        ];
        let outputs = vec![
            Trit::Neg,
            Trit::Neu,
            Trit::Pos,
            Trit::Neu,
            Trit::Neu,
            Trit::Pos,
            Trit::Pos,
            Trit::Pos,
            Trit::Pos,
        ];
        
        let inputs: Vec<Trit> = inputs.iter().map(|entry| {
            entry[0].or(entry[1])
        }).collect();

        assert_eq!(outputs, inputs);
    }

    #[test]
    fn xor() {
        let inputs = vec![
            [Trit::Neg, Trit::Neg],
            [Trit::Neg, Trit::Neu],
            [Trit::Neg, Trit::Pos],
            [Trit::Neu, Trit::Neg],
            [Trit::Neu, Trit::Neu],
            [Trit::Neu, Trit::Pos],
            [Trit::Pos, Trit::Neg],
            [Trit::Pos, Trit::Neu],
            [Trit::Pos, Trit::Pos],
        ];
        let outputs = vec![
            Trit::Neg,
            Trit::Neu,
            Trit::Pos,
            Trit::Neu,
            Trit::Neu,
            Trit::Neu,
            Trit::Pos,
            Trit::Neu,
            Trit::Neg,
        ];
        
        let inputs: Vec<Trit> = inputs.iter().map(|entry| {
            entry[0].xor(entry[1])
        }).collect();

        assert_eq!(outputs, inputs);
    }
}