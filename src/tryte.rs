// SPDX-FileCopyrightText: © 2023 Archaic Archea <archaic.archea@gmail.com>
//
// This Source Code Form is subject to the terms of the Co-operative Nonviolent Public License(CNPL).
// If a copy of the CNPL was not distributed with this file, You can obtain one at https://git.pixie.town/thufie/npl-builder/raw/branch/main/cnpl.md.

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