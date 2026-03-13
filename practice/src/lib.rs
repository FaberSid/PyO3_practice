use std::process::Command;

use practice_core::get_message;
use pyo3::prelude::*;
use pythonize::depythonize;
use serde::Deserialize;

#[pyclass(get_all)]
#[derive(Debug, Deserialize)]
struct UserData {
    id: u64,
    name: String,
}

#[pymethods]
impl UserData {
    fn __repr__(&self) -> String {
        format!("UserData(id={}, name=\"{}\")", self.id, self.name)
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for UserData {
    type Error = PyErr;

    fn extract(ob: pyo3::Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        // Borrowed<'_, '_, PyAny> は Deref で &Bound<'_, PyAny> になるため
        // そのまま pythonize に渡せます。
        depythonize(&ob).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "UserDataへのバインドに失敗しました: {}",
                e
            ))
        })
    }
}

#[pyfunction]
fn check_user(data: UserData) -> PyResult<String> {
    Ok(format!(
        "ID: {}, Name: {} をRustで受信しました",
        data.id, data.name
    ))
}

// 単一のデータを返す
#[pyfunction]
fn get_user() -> UserData {
    UserData {
        id: 1,
        name: "田中".to_string(),
    }
}

// 複数のデータをリストで返す
#[pyfunction]
fn get_users() -> Vec<UserData> {
    vec![
        UserData { id: 1, name: "田中".to_string() },
        UserData { id: 2, name: "佐藤".to_string() },
    ]
}

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok(get_message())
}

#[pymodule]
#[pyo3(name = "practice")]
fn practice_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // 1. インポート時に SHA1 チェックを実行
    check_version_consistency();

    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(check_user, m)?)?;
    m.add_function(wrap_pyfunction!(get_user, m)?)?;
    m.add_function(wrap_pyfunction!(get_users, m)?)?;
    m.add_class::<UserData>()?;
    Ok(())
}

fn check_version_consistency() {
    // 1. ビルド時のSHA1 (env! マクロで取得)
    let build_sha1 = env!("BUILD_GIT_SHA1");

    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    // 2. 実行時の最新SHA1を取得
    let current_sha1 = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(manifest_dir)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() { String::from_utf8(o.stdout).ok() } else { None }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string()); // ここも揃える

    // 3. 厳格に比較。不一致なら即警告。
    if build_sha1 != current_sha1 {
        // 安全に 7文字取るヘルパー
        let to_display = |s: &str| -> String {
            s.chars().take(7).collect::<String>()
        };

        let b_display = to_display(build_sha1);
        let c_display = to_display(&current_sha1);

        eprintln!(
            "\n\x1b[33m[WARNING] practice version mismatch detected!\x1b[0m\n\
             Binary build SHA1: {}\n\
             Current Git SHA1:  {}\n\
             \x1b[36mAction: Please run 'uv run maturin develop' to update.\x1b[0m\n",
            b_display,
            c_display
        );
    }
}
