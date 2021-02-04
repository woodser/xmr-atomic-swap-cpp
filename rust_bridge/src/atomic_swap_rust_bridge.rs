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