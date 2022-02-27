use anchor_lang::prelude::*;
use std::vec::Vec;
use anchor_lang::require;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bi_direcrtional_payments {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, _users: Vec<Pubkey>, _death: u64, _chal: u64) -> ProgramResult {
        let _data = &mut ctx.accounts.data_acc;
        let _now = Clock::get().unwrap().unix_timestamp as u64;

        require!(_now < _death, ProgError::Time);
        require!(_chal > 0, ProgError::Input);

        _data.death = _death;
        _data.chal = _chal;
        _data.ids = _users;
        _data.amnts = vec![0; _data.ids.len()];

        Ok(())
    }

    pub fn update(ctx: Context<Update>, _user: u64, _amnt1: u64, _amnt2: u64) -> ProgramResult {
        let signer = &mut ctx.accounts.signer;
        let _data = &mut ctx.accounts.data_acc;
        let _prop = &mut ctx.accounts.proposal_acc;

        require!(Clock::get().unwrap().unix_timestamp as u64 <= _data.death, ProgError::Expired);
        require!(_data.to_account_info().lamports() > _amnt1+_amnt2, ProgError::Funds);

        _prop.length += 1;
        if _user == 0
        {
            _prop.vote1 = true;
            _prop.vote2 = false;
        }
        else
        {
            _prop.vote1 = false;
            _prop.vote2 = true;
        }

        _prop.amnt1 = _amnt1;
        _prop.amnt2 = _amnt2;
        _data.death = Clock::get().unwrap().unix_timestamp as u64 + _data.chal;
        Ok(())
    }

    pub fn vote(ctx: Context<Update>, _user: u64, _vote: bool) -> ProgramResult {
        let signer = &mut ctx.accounts.signer;
        let _data = &mut ctx.accounts.data_acc;
        let _prop = &mut ctx.accounts.proposal_acc;

        require!(Clock::get().unwrap().unix_timestamp as u64 <= _data.death, ProgError::Expired);

        if _user == 0
        {
            _prop.vote1 = _vote;
        }
        else
        {
            _prop.vote2 = _vote;
        }

        _data.death = Clock::get().unwrap().unix_timestamp as u64 + _data.chal;
        Ok(())
    }

    pub fn execute(ctx: Context<Update>, _user: u64) -> ProgramResult {
        let signer = &mut ctx.accounts.signer;
        let _data = &mut ctx.accounts.data_acc;
        let _prop = &mut ctx.accounts.proposal_acc;

        require!(_prop.vote1 && _prop.vote2, ProgError::Votes);
        require!(Clock::get().unwrap().unix_timestamp as u64>_data.death, ProgError::Expired);

        _data.amnts[0] = _prop.amnt1;
        _data.amnts[1] = _prop.amnt2;

        let bal = _data.amnts[_user as usize];

        require!(_data.to_account_info().lamports() > bal, ProgError::Funds);

        **_data.to_account_info().try_borrow_mut_lamports()? -= bal;
        **ctx.accounts.signer.try_borrow_mut_lamports()? += bal;
        _data.amnts[_user as usize] = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 320 + 80)]
    pub data_acc: Account<'info, Data>,
    #[account(init, payer = user, space = 8 + 8 + 8 + 1 + 1)]
    pub proposal_acc: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub data_acc: Account<'info, Data>,
    #[account(mut)]
    pub proposal_acc: Account<'info, Proposal>,
    pub signer: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[account]
pub struct Proposal {
    pub length: u64,
    pub amnt1: u64,
    pub amnt2: u64,
    pub vote1: bool,
    pub vote2: bool,
}

#[account]
pub struct Data {
    pub death: u64,
    pub chal: u64,
    pub ids: Vec<Pubkey>,
    pub amnts: Vec<u64>,
}

#[error]
pub enum ProgError {
   Time,
   Input,
   Expired,
   Funds,
   Votes,
}
