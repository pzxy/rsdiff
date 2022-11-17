use anyhow::Result;
use rsdiff::DiffConfig;
fn main()->Result<()> {
    let content = include_str!("../fixtures/test.yml");
    let config = DiffConfig::from_yaml(content);
    println!("{:#?}", config);
    Ok(())
}
