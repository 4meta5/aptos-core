// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_crypto::ValidCryptoMaterialStringExt;
use aptos_keygen::KeyGen;
use aptos_types::transaction::authenticator::AuthenticationKey;

fn main() {
    let (privkey, auth_key, account_addr) =
        KeyGen::from_os_rng().generate_credentials_for_account_creation();

    println!(
        "Private Key:\n{}\n\nAuth Key:\n{}\n\nAccount Address:\n{}\n\n",
        privkey.to_encoded_string().unwrap(),
        AuthenticationKey::new(auth_key.as_slice().try_into().unwrap())
            .to_encoded_string()
            .unwrap(),
        account_addr,
    );
}
