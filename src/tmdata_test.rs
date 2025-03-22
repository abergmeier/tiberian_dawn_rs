#[cfg(test)]
mod tests {
    use crate::abstract_::LookupByName;
    use crate::team::TeamMissionType::TMISSION_MOVECELL;
    use crate::tmdata::TeamMissionNames;

    #[test]
    fn lookup_works() {
        let mut result = TeamMissionNames::lookup_type_enum_variant_by_name("Harvest");
        assert_eq!(result, None);
        result = TeamMissionNames::lookup_type_enum_variant_by_name("Move to Cell");
        assert_eq!(result, Some(TMISSION_MOVECELL))
    }
}
