// Copyright Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for RialtoMessages
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `covid`, CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/millau-bridge-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=RialtoMessages
// --extrinsic=*
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --output=./modules/messages/src/weights.rs
// --template=./.maintain/bridge-weight-template.hbs

#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for RialtoMessages.
pub trait WeightInfo {
	fn receive_single_message_proof() -> Weight;
	fn receive_two_messages_proof() -> Weight;
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight;
	fn receive_single_message_proof_1_kb() -> Weight;
	fn receive_single_message_proof_16_kb() -> Weight;
	fn receive_delivery_proof_for_single_message() -> Weight;
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight;
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight;
	fn receive_single_message_proof_with_dispatch(i: u32) -> Weight;
}

/// Weights for `RialtoMessages` that are generated using one of the Bridge testnets.
///
/// Those weights are test only and must never be used in production.
pub struct BridgeWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for BridgeWeight<T> {
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 52_321 nanoseconds.
		Weight::from_parts(54_478_000, 57170)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_two_messages_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 64_597 nanoseconds.
		Weight::from_parts(69_267_000, 57170)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 64_079 nanoseconds.
		Weight::from_parts(65_905_000, 57170)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof_1_kb() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 50_588 nanoseconds.
		Weight::from_parts(53_544_000, 57170)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof_16_kb() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 78_269 nanoseconds.
		Weight::from_parts(81_748_000, 57170)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:1 w:1)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `9584`
		// Minimum execution time: 45_786 nanoseconds.
		Weight::from_parts(47_382_000, 9584)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:1 w:1)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `596`
		//  Estimated: `9584`
		// Minimum execution time: 44_544 nanoseconds.
		Weight::from_parts(45_451_000, 9584)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:2 w:2)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `596`
		//  Estimated: `12124`
		// Minimum execution time: 47_344 nanoseconds.
		Weight::from_parts(48_311_000, 12124)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	///
	/// The range of component `i` is `[128, 2048]`.
	fn receive_single_message_proof_with_dispatch(i: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 52_385 nanoseconds.
		Weight::from_parts(54_919_468, 57170)
			// Standard Error: 108
			.saturating_add(Weight::from_parts(3_286, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 52_321 nanoseconds.
		Weight::from_parts(54_478_000, 57170)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_two_messages_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 64_597 nanoseconds.
		Weight::from_parts(69_267_000, 57170)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 64_079 nanoseconds.
		Weight::from_parts(65_905_000, 57170)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof_1_kb() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 50_588 nanoseconds.
		Weight::from_parts(53_544_000, 57170)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof_16_kb() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 78_269 nanoseconds.
		Weight::from_parts(81_748_000, 57170)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:1 w:1)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `579`
		//  Estimated: `9584`
		// Minimum execution time: 45_786 nanoseconds.
		Weight::from_parts(47_382_000, 9584)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:1 w:1)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `596`
		//  Estimated: `9584`
		// Minimum execution time: 44_544 nanoseconds.
		Weight::from_parts(45_451_000, 9584)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:2 w:2)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `596`
		//  Estimated: `12124`
		// Minimum execution time: 47_344 nanoseconds.
		Weight::from_parts(48_311_000, 12124)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	///
	/// The range of component `i` is `[128, 2048]`.
	fn receive_single_message_proof_with_dispatch(i: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `57170`
		// Minimum execution time: 52_385 nanoseconds.
		Weight::from_parts(54_919_468, 57170)
			// Standard Error: 108
			.saturating_add(Weight::from_parts(3_286, 0).saturating_mul(i.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
