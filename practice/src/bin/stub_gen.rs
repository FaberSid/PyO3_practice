use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = _practice::stub_info()?; // define_stub_info_gatherer! で定義した関数
    stub.generate()?;
    Ok(())
}