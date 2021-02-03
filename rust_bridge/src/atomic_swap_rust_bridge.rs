#[no_mangle]
pub extern "C" fn greetings() 
{
  println!("Hello from Rust library file!");
  
  // generate keys
  let rng = &mut rand::rngs::OsRng;
  let a = swap::bitcoin::SecretKey::new_random(rng);
  let s_a = cross_curve_dleq::Scalar::random(rng);
  let v_a = swap::monero::PrivateViewKey::new_random(rng);
  println!("Keys generated");
  
  // print keys
  let a_str = serde_json::to_string(&a).expect("failed to serialize bitcoin secret key a");
  println!("Bitcoin private key a: {:?}", a_str);
}

#[no_mangle]
pub extern "C" fn add(x:i32, y:i32) -> i32
{
  x + y
}