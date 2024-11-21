use crate::{ storage, zombie::Zombie };
multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ZombieFactory: storage::StorageModule {
    #[only_owner]
    #[endpoint(createZombie)]
    fn create_zombie(&self, owner: ManagedAddress, name: ManagedBuffer, dna: u64) {
        self.zombie_last_index().update(|id: &mut usize| {
            let new_zombie: Zombie<<Self as ContractBase>::Api> = Zombie {
                name: name.clone(),
                dna,
            };

            self.owned_zombies(&owner).insert(*id);
            self.zombie_owner(*id).set(owner);
            self.zombies(*id).set(new_zombie);

            self.new_zombie_event(*id, &name, dna);
            *id += 1;
        });
    }

    #[view]
    fn generate_random_dna(&self) -> u64 {
        let max_dna: u64 = (10u64).pow(self.dna_digits().get() as u32);
        RandomnessSource::new().next_u64_in_range(0, max_dna)
    }

    #[endpoint(createRandomZombie)]
    fn create_random_zombie(&self, name: ManagedBuffer) {
        let caller: ManagedAddress = self.blockchain().get_caller();
        require!(self.owned_zombies(&caller).is_empty(), "You already own a zombie");

        let random_dna: u64 = self.generate_random_dna();
        self.create_zombie(caller, name, random_dna);
    }

    #[event("newZombieEvent")]
    fn new_zombie_event(
        &self,
        #[indexed] zombie_id: usize,
        name: &ManagedBuffer,
        #[indexed] dna: u64
    );
}
