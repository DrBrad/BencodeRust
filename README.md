BencodeRust
========

This is an implementation of Bencode for Rust. Bencode is used for DHTs, Torrents, and Google DataServers. Its a lightweight fast data serialization.
[Wikipedia](https://en.wikipedia.org/wiki/Bencode)

I have also made an implementation of Bencode with [Java](https://github.com/DrBrad/Bencode), [JavaScript](https://github.com/DrBrad/BencodeJS) and [PHP](https://github.com/DrBrad/BencodePHP).

Implemented Types
-----

| Data | Structure | Default Impl |
| ---  | ---       | ---          |
| Vec  | ✔         | Defines own ordering |
| VecDeque | ✔     | Defines own ordering |
| LinkedList | ✔   | Defines own ordering |
| HashMap | ✔      | Ordering missing but content is ordered by key byte representation. |
| BTreeMap | ✔     | Defines own ordering |
| Every Number Type | ✔     | i8-64 u8-64 f32-64 usize |
| String Types | ✔     | Premitive & Object |

Usage
-----
Here are some examples of how to use the Bencode library.

**Bencode**
```rust
use crate::variables::to_bencode::ToBencode;
use crate::variables::from_bencode::FromBencode;

fn main() {
    let encoded = "blank test".to_bencode();
    println!("{:?}", encoded);

    let decoded = String::from_bencode(&encoded, &mut 0);
    println!("{}", decoded);
}
```


