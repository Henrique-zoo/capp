use std::{
    fs,
    path::{Path, PathBuf},
};

use cucumber::World as _;

#[derive(cucumber::World, Debug, Default)]
struct AppWorld;

#[tokio::main]
async fn main() {
    let features_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../features");

    assert!(
        features_dir.is_dir(),
        "features directory not found: {}",
        features_dir.display()
    );
    assert!(
        contains_feature_file(&features_dir),
        "no .feature files found in {}",
        features_dir.display()
    );

    AppWorld::run(features_dir).await;
}

fn contains_feature_file(path: &Path) -> bool {
    let Ok(entries) = fs::read_dir(path) else {
        return false;
    };

    entries.filter_map(Result::ok).any(|entry| {
        let path = entry.path();

        if path.is_dir() {
            return contains_feature_file(&path);
        }

        path.extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("feature"))
    })
}
