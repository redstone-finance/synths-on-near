mod redstone;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

const BTC_BYTES_32_HEX_STR: &str =
    "4254430000000000000000000000000000000000000000000000000000000000";
const SIGNER_1_PUB_KEY: &str = "466d7fcae563e5cb09a0d1870bb580344804617879a14949cf22285f1bae3f276728176c3c6431f8eeda4538dc37c865e2784f3a9e77d044f33e407797e1278a";
const SIGNER_2_PUB_KEY: &str = "4f355bdcb7cc0af728ef3cceb9615d90684bb5b2ca5f859ab0f0b704075871aa385b6b1b8ead809ca67454d9683fcf2ba03456d6fe2c4abe2b07f0fbdbb2f1c1";

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: i8,
}

fn get_pub_key(hex_pub_key: &str) -> [u8; 64] {
    let pub_key_vec = redstone::decode_hex(hex_pub_key).unwrap();
    pub_key_vec.try_into().unwrap()
}

#[near_bindgen]
impl Counter {
    // Public read-only method: Returns the counter value.
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    // Public method: Increment the counter.
    pub fn increment(&mut self) {
        self.val += 1;
        log!("Increased number to {}", self.val);
    }

    // Public method: Decrement the counter.
    pub fn decrement(&mut self) {
        self.val -= 1;
        log!("Decreased number to {}", self.val);
    }

    // Public method - Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        log!("Reset counter to zero");
    }

    pub fn set_val(&mut self, new_val: u128, redstone_payload_str: String) {
        log!("Received redstone payload: {}", redstone_payload_str);
        let redstone_payload = redstone::decode_hex(&redstone_payload_str).unwrap();

        let data_feed_id_vec = redstone::decode_hex(BTC_BYTES_32_HEX_STR).unwrap();
        let data_feed_id: [u8; 32] = data_feed_id_vec.try_into().unwrap();
        let authorised_signers: Vec<[u8; 64]> =
            vec![get_pub_key(SIGNER_1_PUB_KEY), get_pub_key(SIGNER_2_PUB_KEY)];
        let unique_signers_threshold = 1;
        let current_timestamp_milliseconds = 1654353400000;
        let oracle_value = redstone::get_oracle_value(
            &data_feed_id,
            unique_signers_threshold,
            &authorised_signers,
            current_timestamp_milliseconds,
            &redstone_payload,
        );

        log!("Extracted oracle value for BTC: {}", oracle_value);

        if new_val == 42 {
            self.val = 42;
        } else {
            self.val += 2;
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be: `cargo test`
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment() {
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { val: 0 };
        contract.increment();
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let mut contract = Counter { val: 0 };
        contract.decrement();
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        let mut contract = Counter { val: 0 };
        contract.increment();
        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract = Counter { val: 127 };
        contract.increment();
    }

    #[test]
    #[should_panic]
    fn panics_on_underflow() {
        let mut contract = Counter { val: -128 };
        contract.decrement();
    }
}
