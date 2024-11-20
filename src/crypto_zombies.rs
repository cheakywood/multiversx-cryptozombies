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
        self.dna_digits().set(16u8);
        self.zombie_last_index().set(1usize);
    }

    #[upgrade]
    fn upgrade(&self) {}

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
        let mut my_random: RandomnessSource = RandomnessSource::new();
        let dna_digits: u8 = self.dna_digits().get();
        let max_dna_value: u64 = u64::pow(10u64, dna_digits as u32);
        return my_random.next_u64_in_range(0u64, max_dna_value);
    }

    #[endpoint]
    fn create_random_zombie(&self, name: ManagedBuffer) {
        let dna: u64 = self.generate_random_dna();
        self.create_zombie(name, dna);
    }

    #[event("newZombieEvent")]
    fn new_zombie_event(
        &self,
        #[indexed] zombie_id: usize,
        name: ManagedBuffer,
        #[indexed] dna: u64
    );

    #[storage_mapper("dnaDigits")]
    fn dna_digits(&self) -> SingleValueMapper<u8>;

    #[storage_mapper("zombieLastIndex")]
    fn zombie_last_index(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("zombies")]
    fn zombies(&self, id: usize) -> SingleValueMapper<Zombie<Self::Api>>;
}
