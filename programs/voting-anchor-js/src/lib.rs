use anchor_lang::prelude::*;
use std::vec::Vec;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
// declare_id!("6ERnYjMfsr4nvawGLgy1sz3fc6qHm2Ncnpz5GApJgmFs");

#[program]
pub mod voting_anchor_js {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, vac_string: Vec<String>) -> ProgramResult {
        let update_account = &mut ctx.accounts.state;
        update_account.array = vac_string;
        let prop_length = update_account.array.len();
        let prop_vector = vec![0; prop_length];
        update_account.votecountarray = prop_vector;
        Ok(())
    }

    pub fn vote_on_proposal(ctx: Context<Vote>, proposal_string: String) -> ProgramResult {
        let update_account = &mut ctx.accounts.state;
        let vote_account = &mut ctx.accounts.vote_account;
        let signer_address = &mut ctx.accounts.signer;
        

        let prop_length = update_account.array.len();

        for x in 0..prop_length{
            if update_account.array[x]==proposal_string{
                update_account.votecountarray[x]=update_account.votecountarray[x]+1;
                break;
            }
        }
        
        Ok(())
    }
    
    pub fn winning_proposal(ctx: Context<Vote>) -> ProgramResult {
        let update_account = &mut ctx.accounts.state;
        let mut winning_proposal : String;
        let mut winning_vote : u64 = 0;
        let prop_length = update_account.array.len();

        for x in 0..prop_length{
            if winning_vote > update_account.votecountarray[x]{
                winning_vote = update_account.votecountarray[x];
                // winning proposal will corresponds to the maximum vote count
            }
        }


        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 16 + 80)]
    pub state: Account<'info, State>,
    #[account(init, payer = user, space = 16 + 80)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
    pub state: Account<'info, State>,
    pub signer: Signer<'info>
}


#[account]
pub struct VoteAccount {

    pub uservote: Pubkey,
    pub didvote: bool,
    pub prposalstring: String,
}



// Struct 3
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub state: Account<'info, State>
}



// Struct 1
#[account]
pub struct State {

pub array: Vec<String>,
pub votecountarray : Vec<u64>
}

