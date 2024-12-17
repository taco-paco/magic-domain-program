use std::ops::{BitAndAssign, BitOrAssign, Not};

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct FeaturesSet([u64; 4]);

/// A first approximation of features supported by ER validator
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Feature {
    Randomness = 0,
    // .. and nothing else comes to mind, 256 should be more than enough
}

impl FeaturesSet {
    const SEGMENT: usize = u64::BITS as usize;

    pub fn add(&mut self, feature: Feature) {
        let (segment, offset) = self.locate(feature);
        segment.bitor_assign(1 << offset);
    }

    pub fn remove(&mut self, feature: Feature) {
        let (segment, offset) = self.locate(feature);
        segment.bitand_assign((1_u64 << offset).not());
    }

    fn locate(&mut self, feature: Feature) -> (&mut u64, u64) {
        let index = feature as usize / Self::SEGMENT;
        let offset = feature as usize % Self::SEGMENT;
        // SAFETY: feature cannot exceed 255 (repr(u8)), 0..255 / 64 <= 3
        let segment = unsafe { self.0.get_unchecked_mut(index) };
        (segment, offset as u64)
    }

    pub fn contains(&mut self, feature: Feature) -> bool {
        let (segment, offset) = self.locate(feature);
        (*segment & (1 << offset)) >> offset == 1
    }
}

#[cfg(test)]
#[test]
fn test_features_op() {
    let mut features = FeaturesSet::default();
    features.add(Feature::Randomness);
    assert!(features.contains(Feature::Randomness));
    features.remove(Feature::Randomness);
    assert!(!features.contains(Feature::Randomness));
}
