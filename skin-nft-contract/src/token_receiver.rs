use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FuseArgs {
    pub token_ids: Vec<TokenId>,
    pub target_token_series_id: TokenSeriesId
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let ft_contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();

        assert_ne!(
            ft_contract_id, signer_id,
            "Skin: ft_on_approve should only be called via cross-contract call"
        );

        let FuseArgs {token_ids, target_token_series_id} = near_sdk::serde_json::from_str(&msg).expect("Not valid FuseArgs");

        let fuse_cost= self.token_series_by_id.get(&target_token_series_id).unwrap().fuse_cost.unwrap();

        assert_eq!(
            fuse_cost.0,
            ft_contract_id,
            "Skin: Fungible token contract does not match"
        );

        assert!(
            amount.0 >= fuse_cost.1.0,
            "Skin: amount is lower than requirement"
        );

        let result = self.internal_fuse(token_ids, target_token_series_id, sender_id.to_string());
        if result.is_some() {
            PromiseOrValue::Value(U128(0))
        } else {
            panic!("Skin: not minted");
        }
    }
}