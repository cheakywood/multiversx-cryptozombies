#![no_std]

mod storage;
mod zombie;
mod zombie_factory;
mod zombie_feeding;
mod crypto_kitty_proxy;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait CryptoZombies: zombie_factory::ZombieFactory + storage::StorageModule + zombie_feeding::ZombieFeeding {
    #[init]
    fn init(&self) {
        self.dna_digits().set(16);
        self.zombie_last_index().set(0);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
