# rustpic

<div id="header" align="center">


   ### new library on rust for convert an image to ASCII characters
</div>

### Example:

```rust
mod rustpic;

use rustpic::write_to_file;
use rustpic::read_image;

fn main(){
    let image = read_image("src/pig.jpg", 16, 8, "1234567890");
    let _ = write_to_file(image, "src/pig.txt");
}

```
