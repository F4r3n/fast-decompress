#![deny(clippy::all)]


#[macro_use]
extern crate napi_derive;



#[napi]
pub fn untar(input : String, output : String) -> bool
{

  let file = std::fs::File::open(input).unwrap();
  let f = std::io::BufReader::new(file);
  let decompressed = bzip2::read::BzDecoder::new(f);

  let mut a = tar::Archive::new(decompressed);
  match a.unpack(output) {
    Ok(r)=> {
      return true
    },
    Err(e)=> {
      println!("{}", e);
      return false
    }
  }
}
