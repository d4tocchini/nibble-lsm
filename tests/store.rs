/*
 * Nibble - Concurrent Log-Structured Memory for Many-Core Key-Value Stores
 *
 * (c) 2017 Hewlett Packard Enterprise Development LP.
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Lesser General Public License as published by the Free Software Foundation, either version 3
 * of the License, or (at your option) any later version. This program is distributed in the hope that
 * it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License along with this program.
 * If not, see <http://www.gnu.org/licenses/>. As an exception, the copyright holders of this Library
 * grant you permission to (i) compile an Application with the Library, and (ii) distribute the Application
 * containing code generated by the Library and added to the Application during this compilation process
 * under terms of your choice, provided you also meet the terms and conditions of the Application license.
 */


// XXX this test doesn't do what it originally intended.
// it should test the compaction by sustaining a level of capacity
// and measuring the performance
#![feature(test)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate rand; // import before kvs
#[macro_use]
extern crate log;
extern crate test;
extern crate time;

extern crate kvs;

use std::time::Duration;
use std::thread;
use rand::Rng;
use log::LogLevel;

use kvs::lsm::LSM;
use kvs::segment::{ObjDesc,SEGMENT_SIZE};
use kvs::logger;
use kvs::common::ErrorCode;
use kvs::epoch;
use kvs::numa::NodeId;

// TODO test objects larger than block, and segment
// TODO put_object which must traverse chunks
// TODO a get_object which must traverse chunks
// TODO test we can determine live vs dead entries in segment
// TODO test specific cases where header cross block boundaries

/// Allocate a bunch of objects, free some, observe compactor.
/// We assume compactor runs continuously.
#[cfg(IGNORE)]
fn alloc_free(pct_to_free: f32) {
    assert!(pct_to_free <= 1.0);
    assert!(pct_to_free >= 0.0);

    logger::enable();

    let mut kvs = LSM::default();

    kvs.enable_compaction(NodeId(0));
    thread::yield_now();

    let mut allkeys: Vec<String> = Vec::new();

    // allocate until full
    let mut counter: usize = 0;
    let mut size: usize = 0;
    let mut rng = rand::thread_rng();
    let value = rng.gen_ascii_chars().take(1000).collect();
    info!("inserting objects to fill two segments");
    while size < (2*SEGMENT_SIZE) {
        let key = counter.to_string();
        {
            let obj = ObjDesc::new2(&key, &value);
            size += obj.len_with_header();
            if let Err(code) = kvs.put_object(&obj) {
                match code {
                    ErrorCode::OutOfMemory => break,
                    _ => panic!("put failed"),
                }
            }
        }
        allkeys.push(key);
        counter += 1;
    }
    info!("inserted {} objects {} bytes", counter, size);

    // free some (TODO random picking)
    let many: usize = ((counter as f32) * pct_to_free) as usize;
    for key in allkeys.iter().take(many) {
        assert!(kvs.del_object(key).is_ok());
    }

    let dur = Duration::from_secs(10);
    thread::sleep(dur);
}

#[cfg(IGNORE)]
#[allow(dead_code)]
fn alloc_free_all() {
    alloc_free(1.0f32);
}

#[cfg(IGNORE)]
fn alloc_free_half() {
    alloc_free(0.5f32);
}
