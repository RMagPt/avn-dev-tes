use super::{AccountId, Box, Runtime, RuntimeCall, RuntimeDebug, Signature};
use codec::{Decode, Encode};
use pallet_avn_proxy::ProvableProxy;
use scale_info::TypeInfo;
use sp_avn_common::{InnerCallValidator, Proof};

// Avn proxy configuration logic
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct AvnProxyConfig {}
impl Default for AvnProxyConfig {
	fn default() -> Self {
		AvnProxyConfig {}
	}
}

impl ProvableProxy<RuntimeCall, Signature, AccountId> for AvnProxyConfig {
	fn get_proof(call: &RuntimeCall) -> Option<Proof<Signature, AccountId>> {
		match call {
			RuntimeCall::TokenManager(pallet_token_manager::pallet::Call::signed_transfer {
				proof,
				from: _,
				to: _,
				token_id: _,
				amount: _,
			}) => return Some(proof.clone()),
			RuntimeCall::TokenManager(pallet_token_manager::pallet::Call::signed_lower {
				proof,
				from: _,
				token_id: _,
				amount: _,
				t1_recipient: _,
			}) => return Some(proof.clone()),
			RuntimeCall::NftManager(pallet_nft_manager::Call::signed_mint_single_nft {
				proof,
				unique_external_ref: _,
				royalties: _,
				t1_authority: _,
			}) => return Some(proof.clone()),
			RuntimeCall::NftManager(pallet_nft_manager::Call::signed_list_nft_open_for_sale {
				proof,
				nft_id: _,
				market: _,
			}) => return Some(proof.clone()),
			RuntimeCall::NftManager(pallet_nft_manager::Call::signed_transfer_fiat_nft {
				proof,
				nft_id: _,
				t2_transfer_to_public_key: _,
			}) => return Some(proof.clone()),
			RuntimeCall::NftManager(pallet_nft_manager::Call::signed_cancel_list_fiat_nft {
				proof,
				nft_id: _,
			}) => return Some(proof.clone()),
			RuntimeCall::NftManager(pallet_nft_manager::Call::signed_create_batch {
				proof,
				total_supply: _,
				royalties: _,
				t1_authority: _,
			}) => return Some(proof.clone()),
			RuntimeCall::NftManager(pallet_nft_manager::Call::signed_mint_batch_nft {
				proof,
				batch_id: _,
				index: _,
				owner: _,
				unique_external_ref: _,
			}) => return Some(proof.clone()),
			RuntimeCall::NftManager(pallet_nft_manager::Call::signed_list_batch_for_sale {
				proof,
				batch_id: _,
				market: _,
			}) => return Some(proof.clone()),
			RuntimeCall::NftManager(pallet_nft_manager::Call::signed_end_batch_sale {
				proof,
				batch_id: _,
			}) => return Some(proof.clone()),
			_ => None,
		}
	}
}

impl InnerCallValidator for AvnProxyConfig {
	type Call = RuntimeCall;

	fn signature_is_valid(call: &Box<Self::Call>) -> bool {
		match **call {
			RuntimeCall::TokenManager(..) =>
				return pallet_token_manager::Pallet::<Runtime>::signature_is_valid(call),
			RuntimeCall::NftManager(..) =>
				return pallet_nft_manager::Pallet::<Runtime>::signature_is_valid(call),
			_ => false,
		}
	}
}
