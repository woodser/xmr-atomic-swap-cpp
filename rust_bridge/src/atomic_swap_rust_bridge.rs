use rand::rngs::OsRng;

#[no_mangle]
pub extern "C" fn greetings() 
{
  println!("Hello from Rust library file!");
  
  let rng = &mut OsRng;
  let a = swap::bitcoin::SecretKey::new_random(rng);
  let s_a = cross_curve_dleq::Scalar::random(rng);
  let v_a = swap::monero::PrivateViewKey::new_random(rng);
  println!("OK?");
}

#[no_mangle]
pub extern "C" fn add(x:i32, y:i32) -> i32
{
  x + y
}