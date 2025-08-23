#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait Uptime {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(heartbeat)]
    fn heartbeat_endpoint(&self, agent: ManagedBuffer) {}
}
