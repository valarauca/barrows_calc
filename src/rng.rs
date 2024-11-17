
use rand::{
    SeedableRng,RngCore,
    rngs::{
        OsRng,
        adapter::ReseedingRng,
    },
    prng::chacha::{ChaChaCore},
};


// The boilerplate to create our RNG data
pub fn produce_rng() -> Result<ReseedingRng::<ChaChaCore,OsRng>,std::io::Error> {
    let mut os_rng = OsRng::new()?;

    // create seed for chacha
    let mut data = [0u8;32];
    os_rng.fill_bytes(&mut data);
    let chacha: ChaChaCore = <ChaChaCore as SeedableRng>::from_seed(data);

    // NSA suggests re-seeding every 2^64 bytes. Instead I'm doing that twice as often
    Ok(ReseedingRng::<ChaChaCore,OsRng>::new(chacha, 2 << 32, os_rng))
}

