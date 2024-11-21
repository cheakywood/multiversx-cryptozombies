use crate::{ storage, zombie::Zombie };

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ZombieFactory: storage::StorageModule {
    #[only_owner]
    #[endpoint(createZombie)]
    fn create_zombie(&self, owner: ManagedAddress, name: ManagedBuffer, dna: u64) {
        self.zombie_last_index().update(|id: &mut usize| {
            self.new_zombie_event(*id, name.clone(), dna);
            let new_zombie: Zombie<<Self as ContractBase>::Api> = Zombie { name, dna };
            self.owned_zombies(&owner).insert(*id);
            self.zombies(*id).set(new_zombie);
            *id += 1usize;
        })
    }

    #[view]
    fn generate_random_dna(&self) -> u64 {
        let dna_digits: u8 = self.dna_digits().get();
        let max_dna_value: u64 = (10u64).pow(dna_digits as u32);

        // Generate a random DNA value within the range
        return RandomnessSource::new().next_u64_in_range(0, max_dna_value);
    }

    #[endpoint(createRandomZombie)]
    fn create_random_zombie(&self, name: ManagedBuffer) {
        let caller: ManagedAddress = self.blockchain().get_caller();
        require!(self.owned_zombies(&caller).is_empty(), "You already own a zombie");
        let dna: u64 = self.generate_random_dna();
        self.create_zombie(caller, name, dna);
    }

    #[event("newZombieEvent")]
    fn new_zombie_event(
        &self,
        #[indexed] zombie_id: usize,
        name: ManagedBuffer,
        #[indexed] dna: u64
    );
}
