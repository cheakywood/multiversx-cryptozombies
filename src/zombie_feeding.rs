use crate::{
    crypto_kitties_proxy::Kitty,
    crypto_kitties_proxy,
    storage,
    zombie::Zombie,
    zombie_factory,
};
multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ZombieFeeding: storage::StorageModule + zombie_factory::ZombieFactory {
    #[endpoint]
    fn feed_and_multiply(&self, zombie_id: usize, target_dna: u64) {
        let caller: ManagedAddress = self.blockchain().get_caller();
        require!(
            caller == self.zombie_owner(zombie_id).get(),
            "Only the owner of the zombie can perform this operation"
        );
        let my_zombie: Zombie<<Self as ContractBase>::Api> = self.zombies(zombie_id).get();
        let max_dna_value: u64 = (10u64).pow(self.dna_digits().get() as u32);

        let new_dna: u64 = (my_zombie.dna + (target_dna % max_dna_value)) / 2;
        let new_name: ManagedBuffer = ManagedBuffer::from("NewZombie");
        self.create_zombie(caller, new_name, new_dna);
    }

    #[callback]
    fn get_kitty_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<Kitty>,
        zombie_id: usize
    ) {
        match result {
            ManagedAsyncCallResult::Ok(kitty) => {
                self.feed_and_multiply(zombie_id, kitty.genes);
            }
            ManagedAsyncCallResult::Err(_) => {}
        }
    }

    #[endpoint]
    fn feed_on_kitty(&self, zombie_id: usize, kitty_id: usize) {
        let crypto_kitties_sc_address: ManagedAddress = self.crypto_kitties_sc_address().get();

        self.kitty_proxy(crypto_kitties_sc_address)
            .get_kitty(kitty_id)
            .async_call()
            .with_callback(self.callbacks().get_kitty_callback(zombie_id))
            .call_and_exit();
    }

    #[proxy]
    fn kitty_proxy(&self, to: ManagedAddress) -> crypto_kitties_proxy::Proxy<Self::Api>;
}
