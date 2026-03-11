use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() { String::from_utf8(o.stdout).ok() } else { None }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string()); // ここで固定

    println!("cargo:rustc-env=BUILD_GIT_SHA1={}", output);
    
    // .gitがあれば監視（なければ無視）
    if std::path::Path::new(".git/HEAD").exists() {
        println!("cargo:rerun-if-changed=.git/HEAD");
    }
}
