use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WaveRounds {
    pub proposal: u32,
    pub support: u32,
    pub certificate: u32,
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
#[error("slot {slot} exceeds the representable three-round horizon")]
pub struct WaveRoundError {
    pub slot: u32,
}

pub fn wave_rounds(slot: u32) -> Result<WaveRounds, WaveRoundError> {
    let proposal = slot.checked_mul(3).ok_or(WaveRoundError { slot })?;
    let support = proposal.checked_add(1).ok_or(WaveRoundError { slot })?;
    let certificate = proposal.checked_add(2).ok_or(WaveRoundError { slot })?;
    Ok(WaveRounds {
        proposal,
        support,
        certificate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_is_checked() {
        let maximum_slot = (u32::MAX - 2) / 3;
        assert!(wave_rounds(maximum_slot).is_ok());
        assert!(wave_rounds(maximum_slot + 1).is_err());
    }
}
