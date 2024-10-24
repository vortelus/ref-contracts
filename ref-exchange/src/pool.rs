use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance};

use crate::admin_fee::AdminFees;
use crate::degen_swap::DegenSwapPool;
use crate::simple_pool::SimplePool;
use crate::stable_swap::StableSwapPool;
use crate::rated_swap::RatedSwapPool;
use crate::utils::SwapVolume;

/// Generic Pool, providing wrapper around different implementations of swap pools.
/// Allows to add new types of pools just by adding extra item in the enum without needing to migrate the storage.
#[derive(BorshSerialize, BorshDeserialize)]
pub enum Pool {
    SimplePool(SimplePool),
    StableSwapPool(StableSwapPool),
    RatedSwapPool(RatedSwapPool),
    DegenSwapPool(DegenSwapPool),
}

impl Pool {
    /// Returns pool kind.
    pub fn kind(&self) -> String {
        match self {
            Pool::SimplePool(_) => "SIMPLE_POOL".to_string(),
            Pool::StableSwapPool(_) => "STABLE_SWAP".to_string(),
            Pool::RatedSwapPool(_) => "RATED_SWAP".to_string(),
            Pool::DegenSwapPool(_) => "DEGEN_SWAP".to_string(),
        }
    }

    /// Returns which tokens are in the underlying pool.
    pub fn tokens(&self) -> &[AccountId] {
        match self {
            Pool::SimplePool(pool) => pool.tokens(),
            Pool::StableSwapPool(pool) => pool.tokens(),
            Pool::RatedSwapPool(pool) => pool.tokens(),
            Pool::DegenSwapPool(pool) => pool.tokens(),
        }
    }

    pub fn modify_total_fee(&mut self, total_fee: u32) {
        match self {
            Pool::SimplePool(pool) => pool.modify_total_fee(total_fee),
            Pool::StableSwapPool(pool) => pool.modify_total_fee(total_fee),
            Pool::RatedSwapPool(pool) => pool.modify_total_fee(total_fee),
            Pool::DegenSwapPool(pool) => pool.modify_total_fee(total_fee),
        }
    }

    /// Adds liquidity into underlying pool.
    /// Updates amounts to amount kept in the pool.
    pub fn add_liquidity(
        &mut self,
        sender_id: &AccountId,
        amounts: &mut Vec<Balance>,
        is_view: bool
    ) -> Balance {
        match self {
            Pool::SimplePool(pool) => pool.add_liquidity(sender_id, amounts, is_view),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(_) => unimplemented!(),
            Pool::DegenSwapPool(_) => unimplemented!(),
        }
    }

    pub fn add_stable_liquidity(
        &mut self,
        sender_id: &AccountId,
        amounts: &Vec<Balance>,
        min_shares: Balance,
        admin_fee: AdminFees,
        is_view: bool
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(pool) => pool.add_liquidity(sender_id, amounts, min_shares, &admin_fee, is_view),
            Pool::RatedSwapPool(pool) => pool.add_liquidity(sender_id, amounts, min_shares, &admin_fee, is_view),
            Pool::DegenSwapPool(pool) => pool.add_liquidity(sender_id, amounts, min_shares, &admin_fee, is_view),
        }
    }

    /// Removes liquidity from underlying pool.
    pub fn remove_liquidity(
        &mut self,
        sender_id: &AccountId,
        shares: Balance,
        min_amounts: Vec<Balance>,
        is_view: bool
    ) -> Vec<Balance> {
        match self {
            Pool::SimplePool(pool) => pool.remove_liquidity(sender_id, shares, min_amounts, is_view),
            Pool::StableSwapPool(pool) => {
                pool.remove_liquidity_by_shares(sender_id, shares, min_amounts, is_view)
            },
            Pool::RatedSwapPool(pool) => {
                pool.remove_liquidity_by_shares(sender_id, shares, min_amounts, is_view)
            },
            Pool::DegenSwapPool(pool) => {
                pool.remove_liquidity_by_shares(sender_id, shares, min_amounts, is_view)
            }
        }
    }

    /// Removes liquidity from underlying pool.
    pub fn remove_liquidity_by_tokens(
        &mut self,
        sender_id: &AccountId,
        amounts: Vec<Balance>,
        max_burn_shares: Balance,
        admin_fee: AdminFees,
        is_view: bool
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(pool) => {
                pool.remove_liquidity_by_tokens(sender_id, amounts, max_burn_shares, &admin_fee, is_view)
            },
            Pool::RatedSwapPool(pool) => {
                pool.remove_liquidity_by_tokens(sender_id, amounts, max_burn_shares, &admin_fee, is_view)
            },
            Pool::DegenSwapPool(pool) => {
                pool.remove_liquidity_by_tokens(sender_id, amounts, max_burn_shares, &admin_fee, is_view)
            }
        }
    }

    /// Return share decimal.
    pub fn get_share_decimal(&self) -> u8 {
        match self {
            Pool::SimplePool(_) => 24,
            Pool::StableSwapPool(_) => 18,
            Pool::RatedSwapPool(_) => 24,
            Pool::DegenSwapPool(_) => 24,
        }
    }

    /// Returns given pool's total fee.
    pub fn get_fee(&self) -> u32 {
        match self {
            Pool::SimplePool(pool) => pool.get_fee(),
            Pool::StableSwapPool(pool) => pool.get_fee(),
            Pool::RatedSwapPool(pool) => pool.get_fee(),
            Pool::DegenSwapPool(pool) => pool.get_fee(),
        }
    }

    /// Returns volumes of the given pool.
    pub fn get_volumes(&self) -> Vec<SwapVolume> {
        match self {
            Pool::SimplePool(pool) => pool.get_volumes(),
            Pool::StableSwapPool(pool) => pool.get_volumes(),
            Pool::RatedSwapPool(pool) => pool.get_volumes(),
            Pool::DegenSwapPool(pool) => pool.get_volumes(),
        }
    }

    /// Returns given pool's share price in precision 1e8.
    pub fn get_share_price(&self) -> u128 {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(pool) => pool.get_share_price(),
            Pool::RatedSwapPool(pool) => pool.get_share_price(),
            Pool::DegenSwapPool(pool) => pool.get_share_price(),
        }
    }

    pub fn get_tvl(&self) -> u128 {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(_) => unimplemented!(),
            Pool::DegenSwapPool(pool) => {
                pool.assert_degens_valid();
                pool.get_tvl()
            },
        }
    }

    /// Swaps given number of token_in for token_out and returns received amount.
    pub fn swap(
        &mut self,
        token_in: &AccountId,
        amount_in: Balance,
        token_out: &AccountId,
        min_amount_out: Balance,
        admin_fee: AdminFees,
        is_view: bool
    ) -> Balance {
        match self {
            Pool::SimplePool(pool) => {
                pool.swap(token_in, amount_in, token_out, min_amount_out, &admin_fee, is_view)
            }
            Pool::StableSwapPool(pool) => {
                pool.swap(token_in, amount_in, token_out, min_amount_out, &admin_fee, is_view)
            }
            Pool::RatedSwapPool(pool) => {
                pool.swap(token_in, amount_in, token_out, min_amount_out, &admin_fee, is_view)
            }
            Pool::DegenSwapPool(pool) => {
                pool.swap(token_in, amount_in, token_out, min_amount_out, &admin_fee, is_view)
            }
        }
    }

    /// Swaps token_in for a given amount of token_out and returns the amount of token_in spent.
    pub fn swap_by_output(
        &mut self,
        token_in: &AccountId,
        amount_out: Balance,
        token_out: &AccountId,
        max_amount_in: Option<u128>,
        admin_fee: AdminFees,
        is_view: bool
    ) -> Balance {
        match self {
            Pool::SimplePool(pool) => {
                pool.swap_by_output(token_in, amount_out, token_out, max_amount_in, &admin_fee, is_view)
            }
            Pool::StableSwapPool(_) => {
                unimplemented!()
            }
            Pool::RatedSwapPool(_) => {
                unimplemented!()
            }
            Pool::DegenSwapPool(_) => {
                unimplemented!()
            }
        }
    }
    
    pub fn share_total_balance(&self) -> Balance {
        match self {
            Pool::SimplePool(pool) => pool.share_total_balance(),
            Pool::StableSwapPool(pool) => pool.share_total_balance(),
            Pool::RatedSwapPool(pool) => pool.share_total_balance(),
            Pool::DegenSwapPool(pool) => pool.share_total_balance(),
        }
    }

    pub fn share_balances(&self, account_id: &AccountId) -> Balance {
        match self {
            Pool::SimplePool(pool) => pool.share_balance_of(account_id),
            Pool::StableSwapPool(pool) => pool.share_balance_of(account_id),
            Pool::RatedSwapPool(pool) => pool.share_balance_of(account_id),
            Pool::DegenSwapPool(pool) => pool.share_balance_of(account_id),
        }
    }

    pub fn share_transfer(&mut self, sender_id: &AccountId, receiver_id: &AccountId, amount: u128) {
        match self {
            Pool::SimplePool(pool) => pool.share_transfer(sender_id, receiver_id, amount),
            Pool::StableSwapPool(pool) => pool.share_transfer(sender_id, receiver_id, amount),
            Pool::RatedSwapPool(pool) => pool.share_transfer(sender_id, receiver_id, amount),
            Pool::DegenSwapPool(pool) => pool.share_transfer(sender_id, receiver_id, amount),
        }
    }

    /// See if the given account has been registered as a LP
    pub fn share_has_registered(&self, account_id: &AccountId) -> bool {
        match self {
            Pool::SimplePool(pool) => pool.share_has_registered(account_id),
            Pool::StableSwapPool(pool) => pool.share_has_registered(account_id),
            Pool::RatedSwapPool(pool) => pool.share_has_registered(account_id),
            Pool::DegenSwapPool(pool) => pool.share_has_registered(account_id),
        }
    }

    pub fn share_register(&mut self, account_id: &AccountId) {
        match self {
            Pool::SimplePool(pool) => pool.share_register(account_id),
            Pool::StableSwapPool(pool) => pool.share_register(account_id),
            Pool::RatedSwapPool(pool) => pool.share_register(account_id),
            Pool::DegenSwapPool(pool) => pool.share_register(account_id),
        }
    }

    pub fn share_unregister(&mut self, account_id: &AccountId) {
        match self {
            Pool::SimplePool(pool) => pool.share_unregister(account_id),
            Pool::StableSwapPool(pool) => pool.share_unregister(account_id),
            Pool::RatedSwapPool(pool) => pool.share_unregister(account_id),
            Pool::DegenSwapPool(pool) => pool.share_unregister(account_id),
        }
    }

    pub fn predict_add_rated_liquidity(
        &self,
        amounts: &Vec<Balance>,
        rates: &Option<Vec<Balance>>,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(pool) => pool.predict_add_rated_liquidity(amounts, rates, fees),
            Pool::DegenSwapPool(_) => unimplemented!(),
        }
    }

    pub fn predict_add_degen_liquidity(
        &self,
        amounts: &Vec<Balance>,
        degens: &Option<Vec<Balance>>,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(_) => unimplemented!(),
            Pool::DegenSwapPool(pool) => pool.predict_add_degen_liquidity(amounts, degens, fees),
        }
    }

    pub fn predict_remove_rated_liquidity_by_tokens(
        &self,
        amounts: &Vec<Balance>,
        rates: &Option<Vec<Balance>>,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(pool) => pool.predict_remove_rated_liquidity_by_tokens(amounts, rates, fees),
            Pool::DegenSwapPool(_) => unimplemented!(),
        }
    }

    pub fn predict_remove_degen_liquidity_by_tokens(
        &self,
        amounts: &Vec<Balance>,
        degens: &Option<Vec<Balance>>,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(_) => unimplemented!(),
            Pool::DegenSwapPool(pool) => pool.predict_remove_degen_liquidity_by_tokens(amounts, degens, fees),
        }
    }

    pub fn get_rated_return(
        &self,
        token_in: &AccountId,
        amount_in: Balance,
        token_out: &AccountId,
        rates: &Option<Vec<Balance>>,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(pool) => pool.get_rated_return(token_in, amount_in, token_out, rates, fees),
            Pool::DegenSwapPool(_) => unimplemented!(),
        }
    }

    pub fn get_degen_return(
        &self,
        token_in: &AccountId,
        amount_in: Balance,
        token_out: &AccountId,
        degens: &Option<Vec<Balance>>,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(_) => unimplemented!(),
            Pool::DegenSwapPool(pool) => pool.get_degen_return(token_in, amount_in, token_out, degens, fees),
        }
    }
}

impl Pool {
    pub fn assert_tvl_not_exceed_limit(&self, pool_id: u64) {
        match self {
            Pool::DegenSwapPool(pool) => {
                if let Some(degen_pool_limit) = crate::read_pool_limit_from_storage().get(&pool_id).map(|v| v.get_degen_pool_limit()) {
                    assert!(pool.get_tvl() <= degen_pool_limit.tvl_limit, "Exceed Max TVL");
                }
            },
            _ => {}
        }
    }
}