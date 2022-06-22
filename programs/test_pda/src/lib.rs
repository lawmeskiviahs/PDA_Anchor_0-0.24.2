use anchor_lang::prelude::*;
use solana_program::system_program;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::system_instruction;
declare_id!("FTybXRiGT5qSbGYhL3RN5M3sEh5PM68rUybe89i7DRNA");

#[program]
pub mod myprogram {
    // use solana_program::bpf_loader::id;

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

    pub fn transfer_nft(ctx: Context<TransferNFT>, bump:u8) -> ProgramResult {
        msg!("Welcome to pda transfer NFT");
        
            let amount = 1;
            // let seed = b"blablahuehuepda";

            // let ix = spl_token::instruction::transfer(
            //     ctx.accounts.token_program.key,
            //     ctx.accounts.from_token_account.key,
            //     ctx.accounts.to_token_account.key,
            //     ctx.accounts.from_account.key,
            //     &[ctx.accounts.from_account.key],
            //     amount,
            // )?;
            // let (_, bump) = Pubkey::find_program_address(&[seed], &id());
            invoke_signed(
                &system_instruction::transfer(ctx.accounts.from_token_account.key, ctx.accounts.to_token_account.key, amount),
                &[
                    ctx.accounts.from_token_account.clone(),
                    ctx.accounts.to_token_account.clone(),
                    // ctx.accounts.from_account.clone(),
                    // ctx.accounts.token_program.clone(),
                ],
                &[&[b"blablahuehuepda", &[bump]]],
            )?;

        return Ok(());
    }

    pub fn transfer_lamports(ctx: Context<TransferSOL>) -> ProgramResult {
        
        let amount = 1000000000;

        invoke(
            &system_instruction::transfer(ctx.accounts.from_account.key, ctx.accounts.to_account.key, amount),
            &[ctx.accounts.from_account.clone(), ctx.accounts.to_account.clone()],
        )?;
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
            "blablahuehuepda".as_bytes(),
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

#[derive(Accounts)]
pub struct TransferNFT<'info> {
    #[account(signer)]
    /// CHECK xyy
    pub from_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub from_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK xyz
    pub to_token_account: AccountInfo<'info>,
    /// CHECK xyz
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferSOL<'info> {
    #[account(signer)]
    /// CHECK xyz
    pub from_account: AccountInfo<'info>,
    /// CHECK xyz
    #[account(mut)]
    pub to_account: AccountInfo<'info>,
    /// CHECK xyz
    pub system_program: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct BaseAccount {
    pub counter: u32,
    pub key: Pubkey,
}
