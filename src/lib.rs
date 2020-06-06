use std::time::{SystemTime, UNIX_EPOCH};

struct PerlinGen {
    seed: u64,
    repeat: u64
}

impl PerlinGen {
    pub fn default() -> PerlinGen {
        let repeat = 0;
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error getting seed.")
            .as_secs();
        PerlinGen {seed, repeat}
    }

    pub fn new(seed: u64, repeat: u64) -> PerlinGen {
        PerlinGen {seed, repeat}
    }

    pub fn perlin(&self, input: &[f32], octaves: u32, persistence: f32) -> f32 {
        unimplemented!()
    }

    fn repeat_inc(&self, mut n: u64) -> u64{
        n += 1;
        if self.repeat > 0 {
            n %= self.repeat;
        }

        n
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let pgen = crate::PerlinGen::default();
        let res = pgen.perlin(&[1.0, 1.0], 1, 1.0);
        assert!(res >= 0.0 && res < 1.0)
    }
}
