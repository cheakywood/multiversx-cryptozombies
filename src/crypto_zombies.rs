#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopDecode, TopEncode, TypeAbi)]
pub struct Zombie<M: ManagedTypeApi> {
    dna: u64,
    name: ManagedBuffer<M>,
}

#[multiversx_sc::contract]
pub trait CryptoZombies {
    #[init]
    fn init(&self) {
        self.dna_digits().set(16);
        self.zombie_last_index().set(0);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint(createZombie)]
    fn create_zombie(&self, name: ManagedBuffer, dna: u64) {
        self.zombie_last_index().update(|id: &mut usize| {
            self.new_zombie_event(*id, name.clone(), dna);
            let new_zombie: Zombie<<Self as ContractBase>::Api> = Zombie { name, dna };
            self.zombies(*id).set(new_zombie);
            *id += 1usize;
        })
    }

    #[view]
    fn generate_random_dna(&self) -> u64 {
        let dna_digits: u8 = self.dna_digits().get();
        let max_dna_value: u64 = 10u64.pow(dna_digits as u32);

        // Generate a random DNA value within the range
        return RandomnessSource::new().next_u64_in_range(0, max_dna_value)
    }

    #[endpoint(createRandomZombie)]
    fn create_random_zombie(&self, name: ManagedBuffer) {
        let dna: u64 = self.generate_random_dna();
        self.create_zombie(name, dna);
    }

    #[event("newZombieEvent")]
    fn new_zombie_event(
        &self,
        #[indexed] zombie_id: usize,
        name: ManagedBuffer,
        #[indexed] dna: u64,
    );

    #[storage_mapper("dnaDigits")]
    fn dna_digits(&self) -> SingleValueMapper<u8>;

    #[storage_mapper("zombieLastIndex")]
    fn zombie_last_index(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("zombies")]
    fn zombies(&self, id: usize) -> SingleValueMapper<Zombie<Self::Api>>;
}
