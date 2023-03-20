use crate:: {
	ics23::{
		FakeInner,
	},
};
use cosmwasm_std::Deps;
use ics10_grandpa::client_state::ClientState;
use ibc_proto::google::protobuf::Any;
use ibc::{
	core::{
		ics02_client::{
			error::Error,
		},
		ics24_host::identifier::ClientId,
	},
	protobuf::Protobuf,
	Height,
};
use prost::Message;

/// Retrieves raw bytes from storage and deserializes them into [`ClientState`]
pub fn get_client_state<H: Clone>(deps: Deps) -> Result<ClientState<H>, Error> {
	deps.storage
		.get(&format!("clientState").into_bytes())
		.ok_or_else(|| Error::unknown_client_state_type("08-wasm-0".to_string()))
		.and_then(deserialize_client_state)
}

fn deserialize_client_state<H: Clone>(
	client_state: Vec<u8>,
) -> Result<ClientState<H>, Error> {
	let any = Any::decode(&*client_state).map_err(Error::decode)?;
	let wasm_state =
		ics08_wasm::client_state::ClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(
			&any.value,
		)
		.map_err(|e| {
			Error::implementation_specific(format!(
				"[client_state]: error decoding client state bytes to WasmClientState {}",
				e
			))
		})?;
	let any = Any::decode(&*wasm_state.data).map_err(|e| Error::decode(e))?;
	let state =
		ClientState::<H>::decode_vec(&*any.value).map_err(Error::invalid_any_client_state)?;
	Ok(state)
}

pub fn get_consensus_state(deps: Deps, client_id: &ClientId, height: Height) -> Result<Vec<u8>, Error> {
	deps.storage
		.get(&get_consensus_state_key(height))
		.ok_or_else(|| Error::consensus_state_not_found(client_id.clone(), height))
}

pub fn get_consensus_state_key(height: Height) -> Vec<u8> {
	[format!("consensusStates/").into_bytes(), format!("{}", height).into_bytes()].concat()
}