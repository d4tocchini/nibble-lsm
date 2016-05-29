#[macro_use]
extern crate nibble;
#[macro_use]
extern crate log;

use nibble::nib::Nibble;
use nibble::logger;
use std::thread;
use std::time::Duration;
//use nibble::numa::NodeId;

fn main() {
    logger::enable();

    let mut nib = Nibble::new(1<<40);

    //nib.enable_compaction(NodeId(0));

    //println!("tsc {}",nibble::epoch::read());

    //let manager = segmgr_ref!(0, segment::SEGMENT_SIZE, 1<<23);
    //let index = index_ref!();
    //let mut compactor = Compactor::new(&manager, &index);
    //compactor.spawn();

    // add a bunch of segments and leave the index empty
    // the compactor will think all items are dead!

    let sec = 5;
    info!("Sleeping {} seconds", sec);
    thread::sleep(Duration::from_secs(sec));
}
