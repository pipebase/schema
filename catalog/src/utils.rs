use pipebuilder_common::{Catalog, CatalogSchemaMetadataKey, CatalogSchemaValidator};

const MOCK_CATALOG_NAME: &str = "mock_catalog";
const MOCK_NAMESPACE: &str = "mock";
const MOCK_SCHEMA_ID: &str = "mock_schema";
const MOCK_SCHEMA_VERSION: u64 = 0;

pub struct MockCatalogBuilder<P>
where
    P: AsRef<std::path::Path>,
{
    path: Option<P>,
}

impl<P> Default for MockCatalogBuilder<P>
where
    P: AsRef<std::path::Path>,
{
    fn default() -> Self {
        MockCatalogBuilder { path: None }
    }
}

impl<P> MockCatalogBuilder<P>
where
    P: AsRef<std::path::Path>,
{
    pub fn path(mut self, path: P) -> Self {
        self.path = Some(path);
        self
    }

    pub fn build(self) -> anyhow::Result<Catalog> {
        let path = self.path.expect("path undefined");
        let yml = std::fs::read_to_string(path)?;
        Ok(Catalog {
            schema: Self::mock_schema(),
            name: String::from(MOCK_CATALOG_NAME),
            yml,
        })
    }

    fn mock_schema() -> CatalogSchemaMetadataKey {
        CatalogSchemaMetadataKey {
            namespace: String::from(MOCK_NAMESPACE),
            id: String::from(MOCK_SCHEMA_ID),
            version: MOCK_SCHEMA_VERSION,
        }
    }
}

pub struct CatalogSchemaValidatorBuilder<P>
where
    P: AsRef<std::path::Path>,
{
    path: Option<P>,
}

impl<P> Default for CatalogSchemaValidatorBuilder<P>
where
    P: AsRef<std::path::Path>,
{
    fn default() -> Self {
        CatalogSchemaValidatorBuilder { path: None }
    }
}

impl<P> CatalogSchemaValidatorBuilder<P>
where
    P: AsRef<std::path::Path>,
{
    pub fn path(mut self, path: P) -> Self {
        self.path = Some(path);
        self
    }

    pub fn build(self) -> anyhow::Result<CatalogSchemaValidator> {
        let path = self.path.expect("path undefined");
        let json = std::fs::read(path)?;
        // TODO: FROM pipebuilder_common::Error to anyhow::Error
        let validator =
            CatalogSchemaValidator::from_buffer(&json).expect("invalid catalog json schema");
        Ok(validator)
    }
}
