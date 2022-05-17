use anchor_lang::prelude::*;
use solana_program::system_program;
use solana_program::entrypoint::ProgramResult;
declare_id!("9fnWiZpicj8MNHymzYiKpA9GtdxqKeV5p7AYevp9hzWF");

#[program]
pub mod myprogram {
    use super::*;

 pub fn initialize(ctx: Context<Initialize>, _bump:u8) -> ProgramResult {
     msg!("Welcome to pda creation");
        let base_account: &mut Account<BaseAccount> = &mut ctx.accounts.base_account;
        base_account.counter += 1;
        base_account.key = base_account.key();
        msg!("base_account.counter = {}", base_account.counter);
        return Ok(());
    }

    pub fn fetch(ctx: Context<FetchInfo>, _bump:u8) -> ProgramResult {
        msg!("Welcome to pda fetch data");
           let base_account: &mut Account<BaseAccount> = &mut ctx.accounts.base_account;
           msg!("base_account.counter = {}", base_account.counter);
           return Ok(());
       }
}

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = creator, 
        space = 200,
        seeds = [
            "blablahuehue".as_bytes(),
            ], 
        bump)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK xyz
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct FetchInfo<'info> {
    #[account(
        mut,
        seeds = [
            "blablahuehue".as_bytes(),
            ], 
        bump)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
#[derive(Default)]
pub struct BaseAccount {
    pub counter: u32,
    pub key: Pubkey,
}
