multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopDecode, TopEncode, TypeAbi)]
pub struct Zombie<M: ManagedTypeApi> {
    pub dna: u64,
    pub name: ManagedBuffer<M>,
}
