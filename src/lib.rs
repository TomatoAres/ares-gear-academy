#![no_std]

use gstd::{collections::HashMap, msg, prelude::*, ActorId};
use template_io::*;

static mut STATE: Option<HashMap<ActorId, u128>> = None;

// The `init()` entry point.
#[no_mangle]
extern "C" fn init() {
    unsafe { STATE = Some(Default::default()) }
}

// The `handle()` entry point.
#[no_mangle]
extern "C" fn handle() {
    let payload = msg::load().expect("Failed to load payload");

    if let PingPong::Ping = payload {
        let pingers = unsafe { STATE.as_mut().expect("State isn't initialized") };

        pingers
            .entry(msg::source())
            .and_modify(|ping_count| *ping_count = ping_count.saturating_add(1))
            .or_insert(1);

        msg::reply(PingPong::Pong, 0).expect("Failed to reply from `handle()`");
    }
}

// The `state()` entry point.
#[no_mangle]
extern "C" fn state() {
    let state = unsafe { STATE.take().expect("State isn't initialized") };
    msg::reply(State::from_iter(state), 0).expect("Failed to reply from `state()`");
}

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}
