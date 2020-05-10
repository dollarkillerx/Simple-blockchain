use serde::{Serialize,Deserialize};
use bincode;
use crypto::sha3::Sha3;
use crypto::digest::Digest;

pub fn unmarshal<'a,T>(bytes: &'a [u8]) -> T
    where T: Deserialize<'a>
{
    bincode::deserialize(bytes).unwrap()
}

pub fn marshal<T: ?Sized>(val: &T) ->Vec<u8>
    where T: Serialize
{
    bincode::serialize(val).unwrap()
}

pub fn hash(data: &[u8]) -> String {
    let mut had = Sha3::sha3_256();
    had.input(data);
    had.result_str()
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug,Deserialize,Serialize)]
    struct Point {
        x: i32,
        y: String,
    }

    #[test]
    fn test_mush() {
        let a = Point{x:212,y:String::from("Abc")};
        let a1 = marshal(&a);
        println!("a1: {}",String::from_utf8_lossy(&a1));

        let a2:Point = unmarshal(&a1);
        println!("a2: {:#?}",a2);
    }

}