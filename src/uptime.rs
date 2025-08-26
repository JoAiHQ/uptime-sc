#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

const HEARTBEAT_INTERVAL: u64 = 3600; // 1 hour in seconds
const MAX_ALLOWED_DELAY: u64 = HEARTBEAT_INTERVAL / 2; // 30 minutes (half of heartbeat interval)

#[multiversx_sc::contract]
pub trait Uptime {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(heartbeat)]
    fn heartbeat_endpoint(&self) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let last_heartbeat = self.last_heartbeat_timestamp(&caller).get();

        if last_heartbeat > 0 && (current_timestamp - last_heartbeat) > MAX_ALLOWED_DELAY {
            self.lifetime_count(&caller).set(&1);
        } else {
            self.lifetime_count(&caller).update(|current| *current += 1);
        }

        self.last_heartbeat_timestamp(&caller).set(&current_timestamp);
        self.heartbeat_count(&caller).update(|current| *current += 1);
    }

    #[view(getLifetimeInfo)]
    fn get_lifetime_info(&self, agent: &ManagedAddress) -> MultiValue4<u64, u64, u64, u64> {
        let total_heartbeats = self.heartbeat_count(agent).get();
        let lifetime_count = self.lifetime_count(agent).get();
        let lifetime_seconds = lifetime_count * HEARTBEAT_INTERVAL;
        let time_since_last = self.get_time_since_last_heartbeat(agent);

        let time_remaining = if time_since_last >= MAX_ALLOWED_DELAY {
            0
        } else {
            MAX_ALLOWED_DELAY - time_since_last
        };

        (total_heartbeats, lifetime_seconds, time_since_last, time_remaining).into()
    }

    fn get_time_since_last_heartbeat(&self, agent: &ManagedAddress) -> u64 {
        let last_heartbeat = self.last_heartbeat_timestamp(agent).get();
        if last_heartbeat == 0 {
            return 0;
        }
        self.blockchain().get_block_timestamp() - last_heartbeat
    }

    #[view(getHeartbeatCount)]
    #[storage_mapper("heartbeat_count")]
    fn heartbeat_count(&self, agent: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getLastHeartbeatTimestamp)]
    #[storage_mapper("last_heartbeat_timestamp")]
    fn last_heartbeat_timestamp(&self, agent: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getLifetimeCount)]
    #[storage_mapper("lifetime_count")]
    fn lifetime_count(&self, agent: &ManagedAddress) -> SingleValueMapper<u64>;
}
