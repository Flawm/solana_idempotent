pub mod error;
pub mod state;

use {
    anchor_lang::prelude::*,
    crate::{error::*, state::*},
};

declare_id!("id7Fj1ywco2RdzTcQFNcYxf6Wu9iJZeNPtQY9xdsw87");

#[program]
pub mod sol_idempotent {
    use super::*;

    pub fn create_map(ctx: Context<CreateMap>, size: u32) -> Result<()> {
        let size = size as usize;
        let bit_map = &mut ctx.accounts.map;

        bit_map.owner = *ctx.accounts.payer.key;

        // zero init entire vec
        bit_map.bytes = Box::new(vec![0; size]).to_vec();

        Ok(())
    }

    pub fn mark_bit(ctx: Context<MarkBit>, bit: u32) -> Result<()> {
        let bit = bit as usize;
        let bit_map = &mut ctx.accounts.map;

        match check_bit_h(&bit_map.bytes, bit) {
            true => {
                // bit has already been set, we need to exit
                return err!(CustomError::AlreadyRan);
            },
            false => {
                mark_bit_h(&mut bit_map.bytes, bit);
            }
        };

        Ok(())
    }
}

pub fn check_bit_h(map: &Vec<u8>, bit: usize) -> bool {
    ((map[bit / 8]) >> (7 - (bit % 8))) & 1 == 1
}

pub fn mark_bit_h(map: &mut Vec<u8>, bit: usize) -> () {
    map[bit / 8] = map[bit / 8] | 1 << (7 - (bit % 8)) as u8
}
