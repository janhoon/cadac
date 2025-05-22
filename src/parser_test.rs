use crate::parser::{ModelMetadata, ModelParser};
use color_eyre::Result;

#[test]
fn test_parse_simple_select() -> Result<()> {
    let sql = "SELECT a, b, c FROM source_table";
    let mut model = ModelMetadata::new("test_model".to_string());
    let result = model.parse_model(sql)?;

    // Verify model name
    assert_eq!(result.name, "test_model");

    // Verify sources
    assert_eq!(result.sources.len(), 1);
    assert_eq!(result.sources.iter().next().unwrap().name, "source_table");

    // Verify columns
    assert_eq!(result.columns.len(), 3);
    assert!(result.columns.iter().any(|c| c.name == "a"));
    assert!(result.columns.iter().any(|c| c.name == "b"));
    assert!(result.columns.iter().any(|c| c.name == "c"));

    Ok(())
}

#[test]
fn test_parse_select_with_aliases() -> Result<()> {
    let sql = "SELECT 
        a as alias_a, 
        b as alias_b, 
        c 
    FROM source_table";

    let mut model = ModelMetadata::new("test_model".to_string());
    let result = model.parse_model(sql)?;

    // Verify sources
    assert_eq!(result.sources.len(), 1);
    assert_eq!(result.sources.iter().next().unwrap().name, "source_table");

    // Verify columns with aliases
    assert_eq!(result.columns.len(), 3);
    assert!(result.columns.iter().any(|c| c.name == "alias_a"));
    assert!(result.columns.iter().any(|c| c.name == "alias_b"));
    assert!(result.columns.iter().any(|c| c.name == "c"));

    Ok(())
}

#[test]
fn test_parse_select_with_table_reference() -> Result<()> {
    let sql = "SELECT 
        t.a, 
        t.b, 
        t.c 
    FROM source_table t";

    let mut model = ModelMetadata::new("test_model".to_string());
    let result = model.parse_model(sql)?;

    // Verify sources
    assert_eq!(result.sources.len(), 1);
    assert_eq!(result.sources.iter().next().unwrap().name, "source_table");

    // Verify columns
    assert_eq!(result.columns.len(), 3);
    assert!(result.columns.iter().any(|c| c.name == "a"));
    assert!(result.columns.iter().any(|c| c.name == "b"));
    assert!(result.columns.iter().any(|c| c.name == "c"));

    Ok(())
}

#[test]
fn test_parse_select_with_comments() -> Result<()> {
    let sql = "-- Model description
    -- Another comment line
    SELECT 
        a, -- Column a description
        b, -- Column b description
        c  -- Column c description
    FROM source_table";

    let mut model = ModelMetadata::new("test_model".to_string());
    let result = model.parse_model(sql)?;

    // Verify model description
    assert!(result.description.is_some());
    let desc = result.description.as_ref().unwrap();
    assert!(desc.contains("Model description"));

    // Verify sources
    assert_eq!(result.sources.len(), 1);
    assert_eq!(result.sources.iter().next().unwrap().name, "source_table");

    // Verify columns
    assert_eq!(result.columns.len(), 3);

    // Ideally we would check column descriptions too, but that's not implemented yet

    Ok(())
}
