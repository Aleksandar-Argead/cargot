use cargo_metadata::{Dependency as CargoDep, DependencyKind, Metadata, MetadataCommand};
use color_eyre::eyre::{Context, Result};
use std::path::Path;

use crate::models::dependency::{DepKind, Dependency};

pub fn load_dependencies() -> Result<Vec<Dependency>> {
    let metadata = MetadataCommand::new()
        .manifest_path(Path::new("Cargo.toml"))
        .exec()
        .wrap_err("Failed to execute cargo metadata")?;

    let mut deps = Vec::new();

    // We take packages that are direct dependencies of the root package
    if let Some(root_pkg) = metadata.root_package() {
        for dep in &root_pkg.dependencies {
            let resolved_name = dep.rename.as_deref().unwrap_or(&dep.name);

            // Find the actual version used (from resolve)
            let version = find_resolved_version(&metadata, resolved_name, &dep.kind);

            let kind = match dep.kind {
                DependencyKind::Normal => DepKind::Normal,
                DependencyKind::Development => DepKind::Development,
                DependencyKind::Build => DepKind::Build,
                _ => DepKind::Normal,
            };

            deps.push(Dependency {
                name: resolved_name.to_string(),
                version_req: version,
                kind,
            });
        }
    }

    // Sort by name for consistency
    deps.sort_by_key(|d| d.name.clone());

    Ok(deps)
}

fn find_resolved_version(metadata: &Metadata, name: &str, kind: &DependencyKind) -> String {
    // Very simplified version - in real app you'd probably want more accurate matching
    for pkg in &metadata.packages {
        if pkg.name == name {
            return pkg.version.to_string();
        }
    }
    "unknown".to_string()
}
