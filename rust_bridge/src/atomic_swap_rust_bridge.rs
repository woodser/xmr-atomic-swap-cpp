#[no_mangle]
pub extern "C" fn greetings() 
{
  println!("Hello from Rust library file!");
  
  // generate keys
  println!("Generating keys");
  let rng = &mut rand::rngs::OsRng;
  let a = swap::bitcoin::SecretKey::new_random(rng);
  let s_a = cross_curve_dleq::Scalar::random(rng);
  let v_a = swap::monero::PrivateViewKey::new_random(rng);
  
  // print keys
  println!("Bitcoin private key 'a': {:?}", a);
  let a_str = format!("{:?}", a);
  println!("Bitcoin private key 'a' as string: {}", a_str);
  
  // serialize keys to json
  let a_json_str = serde_json::to_string(&a).expect("failed to serialize bitcoin secret key a");
  println!("Bitcoin private key 'a' as JSON: {}", a_json_str);
}

#[no_mangle]
pub extern "C" fn add(x:i32, y:i32) -> i32
{
  x + y
}