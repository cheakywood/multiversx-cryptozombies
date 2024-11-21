#![no_std]

mod storage;
mod zombie;
mod zombie_factory;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait CryptoZombies: zombie_factory::ZombieFactory + storage::StorageModule {
    #[init]
    fn init(&self) {
        self.dna_digits().set(16);
        self.zombie_last_index().set(0);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
