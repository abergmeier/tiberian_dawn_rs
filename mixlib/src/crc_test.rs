#[cfg(test)]
mod tests {
    use crate::CRC;

    #[test]
    fn test_const() {
        const TEST_CRCS: [(&str, u32, u32); 6] = [
            (
                "LROTOR.SHP",
                CRC::calc_u32_from_str("LROTOR.SHP"),
                4154080791,
            ),
            (
                "ROCKET1.AUD",
                CRC::calc_u32_from_str("ROCKET1.AUD"),
                2310290196,
            ),
            (
                "RROTOR.SHP",
                CRC::calc_u32_from_str("RROTOR.SHP"),
                4154080815,
            ),
            (
                "STRCLOST.AUD",
                CRC::calc_u32_from_str("STRCLOST.AUD"),
                4198838547,
            ),
            (
                "STRUGGLE.AUD",
                CRC::calc_u32_from_str("STRUGGLE.AUD"),
                607592713,
            ),
            (
                "SUNDIAL.VQA",
                CRC::calc_u32_from_str("SUNDIAL.VQA"),
                1846749493,
            ),
        ];

        let calculated = TEST_CRCS.map(|crc| (crc.0, crc.1));
        let expected = TEST_CRCS.map(|crc| (crc.0, crc.2));
        assert_eq!(calculated, expected);
    }
}
