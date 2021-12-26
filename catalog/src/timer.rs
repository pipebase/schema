#[cfg(test)]
mod tests {
    use crate::utils::{CatalogSchemaValidatorBuilder, MockCatalogBuilder};
    use pipebuilder_common::ValidateCatalog;

    #[test]
    fn test_timer() {
        let catalog = MockCatalogBuilder::default()
            .path("catalogs/timer_millis.yml")
            .build()
            .expect("build 'timer_millis' catalog failed");
        let mut validator = CatalogSchemaValidatorBuilder::default()
            .path("schemas/timer.json")
            .build()
            .expect("build 'timer' schema validator failed");
        catalog
            .accept(&mut validator)
            .expect("'timer' schema validator visit 'timer_millis' catalog failed");
        validator
            .validate()
            .expect("catalog schema validation failed, catalog: 'timer_millis', schema: 'timer'");
    }
}
