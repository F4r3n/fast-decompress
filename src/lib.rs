#![deny(clippy::all)]


#[macro_use]
extern crate napi_derive;


#[napi]
pub fn untar_lzma(input : String, output : String) -> bool
{
  if let Ok(file) = std::fs::File::open(input)
  {
    let mut f = std::io::BufReader::new(file);
    let mut o = Vec::new();


    match lzma_rs::xz_decompress( &mut f, &mut o) {
      Ok(()) => {

      },
      Err(..)=> {
        return false;
      }
    }

    let mut a = tar::Archive::new(o.as_slice());
    match a.unpack(output) {
      Ok(_r)=> {
        return true
      },
      Err(e)=> {
        println!("{}", e);
        return false
      }
    }
  }
false
}



#[napi]
pub fn untar_bzip2(input : String, output : String) -> bool
{

  if let Ok(file) = std::fs::File::open(input)
  {
    let f = std::io::BufReader::new(file);
    let decompressed = bzip2::read::BzDecoder::new(f);
  
    let mut a = tar::Archive::new(decompressed);
    match a.unpack(output) {
      Ok(_r)=> {
        return true
      },
      Err(e)=> {
        println!("{}", e);
        return false
      }
    }
  }

false
}

