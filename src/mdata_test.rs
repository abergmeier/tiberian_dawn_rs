#[cfg(test)]
mod test {

    use crate::abstract_::LookupByName;
    use crate::mdata::MissionNames;
    use crate::mission::MissionType::MISSION_GUARD_AREA;

    #[test]
    fn lookup_works() {
        let result = MissionNames::lookup_type_enum_variant_by_name("Area Guard");
        assert_eq!(result, Some(MISSION_GUARD_AREA));
    }
}
