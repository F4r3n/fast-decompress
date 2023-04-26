#![deny(clippy::all)]


#[macro_use]
extern crate napi_derive;

enum CompressionType {
    BZIP2
}

use std::{convert::TryFrom, io::Write};

impl TryFrom<i32> for CompressionType {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == CompressionType::BZIP2 as i32 => Ok(CompressionType::BZIP2),
            _ => Err(()),
        }
    }
}

#[napi]
pub fn untar(input : String, output : String) -> bool
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


#[napi]
pub fn tar(input : String, output : String, compression_type : i32) -> bool
{
  if let Ok(ctype) = CompressionType::try_from(compression_type)
  {
    let path = std::path::Path::new(&input);
    let mut file: std::fs::File = std::fs::File::open(&input).unwrap();

    let output = std::fs::File::create(output).unwrap();

    match ctype {
      CompressionType::BZIP2 => {
        let mut a = tar::Builder::new(&output);
        a.append_file(&path, &mut file);
        a.into_inner();

        let compressed = bzip2::write::BzEncoder::new(std::io::BufWriter::new(output), bzip2::Compression::best());
        compressed.finish();

      }
    }

    true
  }
  else {
      false
  }


}
