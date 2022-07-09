use {
    anchor_lang::*,
    anchor_lang::prelude::*,
};

// 8 byte discriminator, owner
pub const MAP_SIZE: usize = 8 + 32 + 4;

#[derive(Accounts)]
#[instruction(size: u32)]
pub struct CreateMap<'info> {
    #[account(
        mut
    )]
    pub payer: Signer<'info>,
    #[account(
        init,
        signer,
        payer = payer,
        space = MAP_SIZE + size as usize,
    )]
    pub map: Account<'info, MapAccount>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MarkBit<'info> {
    pub payer: Signer<'info>,
    #[account(
        mut,
        constraint = map.owner == *payer.key
    )]
    pub map: Box<Account<'info, MapAccount>>
}

#[account]
pub struct MapAccount {
    pub owner: Pubkey,
    pub bytes: Vec<u8>
}
