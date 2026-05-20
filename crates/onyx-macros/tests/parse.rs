#[test]
fn test_parse_local_resource() -> Result<(), Box<dyn std::error::Error>> {
    let resource = onyx_macros::parse!("file://path/to/file.txt")?;
    assert_eq!(resource.filename(), "file.txt");
    Ok(())
}

#[test]
fn test_parse_remote_resource() -> Result<(), Box<dyn std::error::Error>> {
    let resource = onyx_macros::parse!("http://path/to/file.json")?;
    assert_eq!(resource.filename(), "file.json");
    Ok(())
}

#[cfg(feature = "huggingface")]
#[test]
fn test_parse_hf_resource() -> Result<(), Box<dyn std::error::Error>> {
    let resource = onyx_macros::parse!("hf://facebook/bart-large/model.onnx")?;
    assert_eq!(resource.filename(), "model.onnx");
    Ok(())
}
