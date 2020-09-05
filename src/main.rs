extern crate quick_xml;
extern crate rayon;
extern crate serde;

mod releases;

use releases::load_releases;

fn main() {
    load_releases();
}
