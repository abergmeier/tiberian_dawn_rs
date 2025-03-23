#[cfg(test)]
mod tests {
    use crate::{
        heap::{InsertError, TFixedIHeapClass},
        team::{FromIniError, ReadTeamEntriesError, TeamTypeClass},
    };
    use ini::Ini;

    #[test]
    fn read_ini_fails_due_to_capacity() {
        let mut heap: TFixedIHeapClass<TeamTypeClass> = TFixedIHeapClass::with_capacity(1);

        let filename = env!("CARGO_MANIFEST_DIR").to_owned() + "/tests/testdata/SCG62EA.INI";
        let ini = Ini::load_from_file(filename).unwrap();
        let result = heap.try_read_ini(&ini);
        assert_eq!(
            result.err(),
            Some(FromIniError::ReadTeamEntriesError(
                ReadTeamEntriesError::Insert(InsertError::ReachedCapacity(19, 1))
            ))
        );
    }

    #[test]
    fn read_ini_works() {
        let mut heap: TFixedIHeapClass<TeamTypeClass> = TFixedIHeapClass::with_capacity(19);
        let filename = env!("CARGO_MANIFEST_DIR").to_owned() + "/tests/testdata/SCG62EA.INI";
        let ini = Ini::load_from_file(filename).unwrap();
        let result = heap.try_read_ini(&ini).unwrap();
        assert_eq!(result.len(), 19);
    }
}
