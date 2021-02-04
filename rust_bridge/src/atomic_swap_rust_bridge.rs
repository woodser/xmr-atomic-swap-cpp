#[no_mangle]
pub extern "C" fn greetings() 
{
  println!("Hello from Rust library file!");
  
  // random number generator for key generation
  let rng = &mut rand::rngs::OsRng;
  
  // generate keys
  println!("Generating keys");
  let a = swap::bitcoin::SecretKey::new_random(rng);        // Alice's bitcoin private key
  let A = a.public();                                       // Alice's bitcoin public address?
  let s_a = cross_curve_dleq::Scalar::random(rng);          // Alice's monero private key
  let v_a = swap::monero::PrivateViewKey::new_random(rng);  // Alice's monero private view key
  let S_a_monero = swap::monero::PublicKey::from_private_key(&swap::monero::PrivateKey {  // ??? Alice's monero public key
    scalar: s_a.into_ed25519(),
  });
  let S_a_bitcoin: swap::bitcoin::PublicKey = s_a.into_secp256k1().into(); // ??? Alice's bitcoin public key
  let dleq_proof_s_a = cross_curve_dleq::Proof::new(rng, &s_a); // dleq proof for s_a
  
  //let bitcoin_wallet = Arc<swap::bitcoin::Wallet>;
  //let a_redeem_address: swap::bitcoin::Address = bitcoin_wallet.new_address().await?;
  //let a_punish_address = a_redeem_address.close();
  
  
  
  // print keys
  //let a_str = format!("{:?}", a);
  //println!("Bitcoin private key 'a' as string: {}", a_str);
  println!("Alice Bitcoin secret key 'a': {:?}", a);
  println!("Alice Bitcoin dleq scalar 's_a': {:?}", s_a);
  println!("Alice Monero private view key 'v_a': {:?}", v_a);
  
  // serialize keys to json
  let a_json_str = serde_json::to_string(&a).expect("failed to serialize bitcoin secret key a");
  println!("Bitcoin private key 'a' as JSON: {}", a_json_str);
  let s_a_json_str = serde_json::to_string(&s_a).expect("failed to serialize bitcoin dleq scalar");
  println!("Bitcoin dleq scalar 's_a' as JSON: {}", s_a_json_str);
  let v_a_json_str = serde_json::to_string(&v_a).expect("failed to serialize monero private view key");
  println!("Monero private view key 'v_a' as JSON: {}", v_a_json_str);
  
  // deserialize json to keys
  let s_a_json: cross_curve_dleq::Scalar = serde_json::from_str(&s_a_json_str).unwrap();
  println!("s_a re-initialized from json: {:?}", s_a_json);
}

#[no_mangle]
pub extern "C" fn add(x:i32, y:i32) -> i32
{
  x + y
}