use anchor_lang::prelude::*;
use solana_program::system_program;
use solana_program::rent::Rent;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke,invoke_signed};
pub use solana_program::sysvar;
use solana_program::system_instruction;
// use solana_program::instruction::{AccountMeta, Instruction};
// use spl_token::id;
// use spl_token::instruction::TokenInstruction::;
// use spl_associated_token_account::instruction::create_associated_token_account;
declare_id!("CFNwgx5rdMbDtdBcDztcCzPeviMCtH8D1yFaLVTi1khL");

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

    // pub fn create_associated_token_account(
    //     funding_address: &Pubkey,
    //     wallet_address: &Pubkey,
    //     spl_token_mint_address: &Pubkey,
    // ) -> Instruction {
    //     // let associated_account_address =
    //     //     get_associated_token_address(wallet_address, spl_token_mint_address);
    
    //     Instruction {
    //         program_id: id(),
    //         accounts: vec![
    //             AccountMeta::new(*funding_address, true),
    //             AccountMeta::new(associated_account_address, false),
    //             AccountMeta::new_readonly(*wallet_address, false),
    //             AccountMeta::new_readonly(*spl_token_mint_address, false),
    //             AccountMeta::new_readonly(solana_program::system_program::id(), false),
    //             AccountMeta::new_readonly(spl_token::id(), false),
    //             AccountMeta::new_readonly(sysvar::rent::id(), false),
    //         ],
    //         data: vec![],
    //     }
    // }

    pub fn transfer_nft(ctx: Context<TransferNFT>) -> ProgramResult {
        msg!("Welcome to pda transfer nft");
        
            let amount: u64 = 1;

            // let destination_pubkey: Pubkey = ;
            // let to_token_account = Pubkey::find_program_address(
            //     &[
            //         &ctx.accounts.to_account.key.to_bytes(),
            //         &ctx.accounts.token_program.key.to_bytes(),
            //         &ctx.accounts.mint.key.to_bytes(),
            //     ],
            //     ctx.accounts.associated_token_program.key,
            // ).0;

            // msg!("token account for account {:?} and {:?} mint is {:?}", ctx.accounts.from_account.key, ctx.accounts.mint.key, token_account);

            // let ix2 = create_associated_token_account(
            //     &ctx.accounts.from_account.key, 
            //     &ctx.accounts.from_account.key, 
            //     &ctx.accounts.mint.key,
                // &ctx.accounts.token_program,
            // );
            // invoke_signed(
            //     &ix2,
            //     &[
            //         ctx.accounts.from_account.clone(),
            //         ctx.accounts.mint.clone(),
            //         ctx.accounts.token_program.clone()
            //     ],
            //     &[&[ &ctx.accounts.to_account.key.as_ref(), &ctx.accounts.token_program.key.as_ref(),&ctx.accounts.mint.key.as_ref() ]]
            // )?;

            msg!("token account created");

            let ix = spl_token::instruction::transfer(
                ctx.accounts.token_program.key,
                ctx.accounts.from_token_account.key,
                ctx.accounts.to_token_account.key,
                ctx.accounts.from_account.key,
                &[ctx.accounts.from_account.key],
                amount,
            )?;
            invoke(
                &ix,
                &[
                    ctx.accounts.from_token_account.clone(),
                    ctx.accounts.to_token_account.clone(),
                    ctx.accounts.from_account.clone(),
                    ctx.accounts.token_program.clone(),
                ],
            )?;

        return Ok(());
    }

    pub fn transfer_lamports(ctx: Context<TransferSOL>) -> ProgramResult {
        
        let amount = 10000000;

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
    #[account(mut)]
    /// CHECK xyz
    pub to_account: AccountInfo<'info>,
    /// CHECK xyz
    pub mint: AccountInfo<'info>,
    /// CHECK xyz
    pub token_program: AccountInfo<'info>,
    /// CHECK xyz
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK xyz
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferSOL<'info> {
    #[account(mut, signer)]
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
