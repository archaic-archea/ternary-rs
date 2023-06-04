use crate::trit::Trit;

pub struct Tryte([Trit; 6]);

impl Tryte {
    pub fn to_i16(&self) -> i16 {
        let mut total = 0;

        for (pos, trit) in self.0.iter().rev().enumerate() {
            let modifier = 3_i16.pow(pos as u32);
            match trit {
                Trit::Neg => total -= modifier,
                Trit::Neu => (),
                Trit::Pos => total += modifier
            }
        }

        total
    }
}