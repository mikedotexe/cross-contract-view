use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, ext_contract, AccountId, PromiseResult, Promise, Gas};

near_sdk::setup_alloc!();

pub const XCC_GAS: Gas = 20000000000000;

/// This simple example hardcodes the value.
fn get_whitelist_contract() -> AccountId {
    "whitelist.demo.testnet".to_string()
}
fn get_account_to_check() -> AccountId {
    "mike.testnet".to_string()
}


#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn callback_promise_result() -> bool;
    fn callback_arg_macro(#[callback] val: bool) -> bool;
}

#[ext_contract(ext_whitelist)]
pub trait ExtWhitelist {
    fn is_whitelisted(staking_pool_account_id: AccountId) -> bool;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn xcc_use_promise_result(&mut self) -> Promise {
        ext_whitelist::is_whitelisted(get_account_to_check(), &get_whitelist_contract(), 0, XCC_GAS).then(
            ext_self::callback_promise_result(
                &env::current_account_id(),
                0,
                XCC_GAS,
            ),
        )
    }

    pub fn xcc_use_arg_macro(&mut self) -> Promise {
        ext_whitelist::is_whitelisted(get_account_to_check(), &get_whitelist_contract(), 0, XCC_GAS).then(
            ext_self::callback_arg_macro(
                &env::current_account_id(),
                0,
                XCC_GAS,
            ),
        )
    }

    // Callbacks

    #[private]
    pub fn callback_promise_result(&mut self) -> bool {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(is_whitelisted) = near_sdk::serde_json::from_slice::<bool>(&val) {
                    is_whitelisted
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            },
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }

    #[private]
    pub fn callback_arg_macro(&mut self, #[callback] val: bool) -> bool {
        val
    }

}
