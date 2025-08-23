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
    fn heartbeat_endpoint(&self) {
        let caller = self.blockchain().get_caller();

        self.heartbeat_count(&caller).update(|current| *current += 1);
    }

    #[view(getHeartbeatCount)]
    #[storage_mapper("heartbeat_count")]
    fn heartbeat_count(&self, agent: &ManagedAddress) -> SingleValueMapper<u64>;
}
