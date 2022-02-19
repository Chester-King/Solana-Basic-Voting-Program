use anchor_lang::prelude::*;

declare_id!("FeyArC12q6s79euLCbUGjF7Jix8EgECtyWV45zX4J3Xr");

#[program]
pub mod voting_anchor_app {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;  
        vote_account.proposal1 = 0;
        vote_account.proposal2 = 0;
        vote_account.proposal3 = 0;
        vote_account.proposal4 = 0;
        vote_account.proposal5 = 0;
        Ok(())
    }
    pub fn vote_1(ctx: Context<Vote>) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.proposal1 += 1;
        Ok(())
    }
    pub fn vote_2(ctx: Context<Vote>) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.proposal2 += 1;
        Ok(())
    }
    pub fn vote_3(ctx: Context<Vote>) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.proposal3 += 1;
        Ok(())
    }
    pub fn vote_4(ctx: Context<Vote>) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.proposal4 += 1;
        Ok(())
    }
    pub fn vote_5(ctx: Context<Vote>) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.proposal5 += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 16 + 40)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}


#[account]
pub struct VoteAccount {
    pub proposal1: u64,
    pub proposal2: u64,
    pub proposal3: u64,
    pub proposal4: u64,
    pub proposal5: u64,
}
