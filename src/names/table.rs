/// Extracts the table name from a migration name.
///
/// Supports these common patterns:
///   - `add_<thing>_to_<table>`        → `<table>`
///   - `remove_<thing>_from_<table>`   → `<table>`
///   - `create_<table>`                → `<table>`
///   - `drop_<table>`                  → `<table>`
///   - `update_<table>_<suffix...>`    → `<table>`
///   - `rename_<table>_<suffix...>`    → `<table>`
///   - `migrate_<table>_<suffix...>`   → `<table>`
pub fn extract_table_name(migration: &str) -> Option<&str> {
    // Patterns where the table follows a keyword
    for prefix in &["add_", "remove_", "drop_column_", "create_column_"] {
        if let Some(rest) = migration.strip_prefix(prefix) {
            // Table name comes after `_to_` or `_from_`
            for separator in &["_to_", "_from_"] {
                if let Some(pos) = rest.find(separator) {
                    let after = &rest[pos + separator.len()..];
                    if !after.is_empty() {
                        // Table name is the first word after the separator
                        let table = after.split('_').next().unwrap_or(after);
                        // But if there are multiple words, take everything (it may be a compound table name)
                        // Convention: table name goes to end of string after separator
                        return Some(after);
                    }
                }
            }
        }
    }

    //Patterns where the table immediately follows the verb
    for prefix in &["create_", "drop_"] {
        if let Some(table) = migration.strip_prefix(prefix) {
            if !table.is_empty() {
                return Some(table);
            }
        }
    }

    // Patterns where the table is the first noun after the verb,
    // and a suffix follows: update_<table>_<stuff>
    for prefix in &["update_", "rename_", "migrate_", "backfill_", "index_"] {
        if let Some(rest) = migration.strip_prefix(prefix) {
            if !rest.is_empty() {
                // Find the table by taking the first `_`-delimited token
                // This is ambiguous for compound table names like `user_profiles`,
                // so we return everything up to the last recognizable suffix keyword.
                let table = extract_noun_before_suffix(rest);
                return Some(table);
            }
        }
    }

    None
}

/// For migrations like `update_asdf_types` or `update_user_profiles_nullability`,
/// tries to find where the table name ends and a "suffix" keyword begins.
///
/// Known suffix keywords: types, nullability, index, indexes, indices, column,
/// columns, constraint, constraints, default, defaults, fk, pk, trigger, view
fn extract_noun_before_suffix(s: &str) -> &str {
    const SUFFIX_KEYWORDS: &[&str] = &[
        "types",
        "type",
        "nullability",
        "index",
        "indexes",
        "indices",
        "column",
        "columns",
        "constraint",
        "constraints",
        "default",
        "defaults",
        "fk",
        "pk",
        "trigger",
        "view",
        "data",
        "null",
    ];

    let parts: Vec<&str> = s.split('_').collect();

    // Walk from the right; once we hit a non-suffix word, everything before is the table
    let mut suffix_start = parts.len();
    for i in (0..parts.len()).rev() {
        if SUFFIX_KEYWORDS.contains(&parts[i]) {
            suffix_start = i;
        } else {
            break;
        }
    }

    // Reconstruct the table portion
    if suffix_start == 0 {
        // Everything is a suffix keyword — just return the whole string
        s
    } else {
        // Return the slice of the original string up to where the suffix starts
        let end = parts[..suffix_start].iter().map(|p| p.len()).sum::<usize>()
            + suffix_start.saturating_sub(1); // underscores between parts
        &s[..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_to() {
        assert_eq!(
            extract_table_name("add_column_bananas_to_asdf"),
            Some("asdf")
        );
        assert_eq!(extract_table_name("add_email_to_users"), Some("users"));
        assert_eq!(
            extract_table_name("add_index_to_user_profiles"),
            Some("user_profiles")
        );
    }

    #[test]
    fn test_remove_from() {
        assert_eq!(
            extract_table_name("remove_column_from_orders"),
            Some("orders")
        );
        assert_eq!(
            extract_table_name("remove_index_from_user_profiles"),
            Some("user_profiles")
        );
    }

    #[test]
    fn test_create_drop() {
        assert_eq!(extract_table_name("create_users"), Some("users"));
        assert_eq!(
            extract_table_name("create_user_profiles"),
            Some("user_profiles")
        );
        assert_eq!(
            extract_table_name("drop_legacy_orders"),
            Some("legacy_orders")
        );
    }

    #[test]
    fn test_update_with_suffix() {
        assert_eq!(extract_table_name("update_asdf_types"), Some("asdf"));
        assert_eq!(
            extract_table_name("update_users_nullability"),
            Some("users")
        );
        assert_eq!(extract_table_name("update_orders_default"), Some("orders"));
    }

    #[test]
    fn test_backfill() {
        assert_eq!(extract_table_name("backfill_users_data"), Some("users"));
    }
}
