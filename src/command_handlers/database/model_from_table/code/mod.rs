use crate::fields::{Field, Type, Visibility};
use tree_sitter::{Node, Parser};

// parse code into Fields

/// Scans rust code returning the location of the first WeldsModel
pub(super) fn find_model(code: &str) -> Option<(usize, usize)> {
    if code.trim().is_empty() {
        return None;
    }

    // Initialize parser with Rust grammar
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .ok()?;
    let tree = parser.parse(code, None)?;
    let root = tree.root_node();
    model_scan(code, root)
}

/// Walk the AST, find the first welds model.
fn model_scan(source: &str, node: Node) -> Option<(usize, usize)> {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        // check if the struct is a welds model
        if child.kind() == "struct_item" && check_struct(source, &child) {
            let start = child.start_byte();
            let end = child.end_byte();
            return Some((start, end));
        }
        // keep waking the tree
        if let Some(found) = model_scan(source, child) {
            return Some(found);
        }
    }
    None
}

/// The node is a struct. Test if the node is the WeldsModel we have been looking for
fn check_struct(source: &str, node: &Node) -> bool {
    let mut start = node.start_byte();
    // walk backwards to grab all the attributes of this struct
    let mut prev = *node;
    while let Some(sib) = prev.prev_sibling() {
        if sib.kind() == "comment" || sib.kind() == "attribute_item" {
            start = sib.start_byte();
            prev = sib;
        } else {
            break;
        }
    }
    let test_chunk = &source[start..node.end_byte()];
    test_chunk.contains("WeldsModel")
}

/// reads a snipit of rust code "a struct" and extracts the fields.
pub(super) fn read_fields(code: &str) -> Vec<Field> {
    // Initialize parser with Rust grammar
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .unwrap();

    let tree = match parser.parse(code, None) {
        Some(tree) => tree,
        None => return Vec::default(),
    };

    let root = tree.root_node();
    let blocks = extract_struct_fields(code, root);

    blocks
        .iter()
        .map(|b| Field {
            visibility: b
                .visibility
                .and_then(|v| v.parse().ok())
                .unwrap_or(Visibility::Private),
            primary_key: b.head.contains("primary_key"),
            welds_ignored: b.head.contains("[welds(ignore)]"),
            name: b.name.to_string(),
            // code columns are overriden unless `ignore` no need to parse col_name.
            col_name: b.name.to_string(),
            ty: Type::Raw("".to_string(), b.r#type.to_string()),
            null: b.null,
        })
        .collect()
}

fn find_node_kind<'tree>(kind: &'static str, node: Node<'tree>) -> Option<Node<'tree>> {
    if node.kind() == kind {
        return Some(node);
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if let Some(found) = find_node_kind(kind, child) {
            return Some(found);
        }
    }
    None
}

fn read_node_kind<'s>(kind: &'static str, node: Node, src: &'s str) -> Option<&'s str> {
    if node.kind() == kind {
        let snipit: &str = &src[node.start_byte()..node.end_byte()];
        return Some(snipit);
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if let Some(found) = read_node_kind(kind, child, src) {
            return Some(found);
        }
    }
    None
}

fn all_node_kinds<'tree>(kind: &'static str, node: Node<'tree>, founds: &mut Vec<Node<'tree>>) {
    if node.kind() == kind {
        founds.push(node);
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        all_node_kinds(kind, child, founds)
    }
}

// ** For Debugging **
fn dbg_kind(node: Node, src: &str) {
    let snipit: &str = &src[node.start_byte()..node.end_byte()];
    println!("{}\t\t{}", node.kind(), snipit);
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        dbg_kind(child, src)
    }
}

//walk backwards from a node reading everything in front of it
fn read_head<'s>(node: Node, src: &'s str) -> &'s str {
    let mut start = node.start_byte();
    let end = node.start_byte();
    //let mut parts: Vec<String> = Vec::default();
    let mut prev = node.prev_sibling();
    while let Some(prev_node) = prev {
        let kind = prev_node.kind();
        if kind == "," || kind == "{" {
            break;
        }
        start = prev_node.start_byte();
        prev = prev_node.prev_sibling();
    }
    &src[start..end]
}

/// A single struct‐field, including any leading comments/attributes (`head`),
/// the field’s identifier (`name`), and its type (`r#type`).
#[derive(Debug)]
struct FieldBlock<'s> {
    visibility: Option<&'s str>,
    head: &'s str,   // e.g. comments and `#[…]` attributes
    name: &'s str,   // the bare field name
    r#type: &'s str, // the type annotation
    null: bool,      // field is Option
}

fn extract_struct_fields<'s>(source: &'s str, struct_node: Node) -> Vec<FieldBlock<'s>> {
    let mut out = Vec::new();

    let mut field_declarations = Vec::default();
    all_node_kinds("field_declaration", struct_node, &mut field_declarations);

    for field_declaration in field_declarations {
        let head = read_head(field_declaration, source);

        let visibility = read_node_kind("visibility_modifier", field_declaration, source);

        let name = read_node_kind("field_identifier", field_declaration, source);

        let (null, r#type) = match read_type(source, field_declaration) {
            Some(ty) => ty,
            None => continue,
        };

        let name = match name {
            Some(name) => name,
            None => continue,
        };

        out.push(FieldBlock {
            head,
            visibility,
            name,
            r#type,
            null,
        })
    }

    out
}

/// reads the type out of the field node.
/// if Option it extract the contents of the option
fn read_type<'s>(source: &'s str, node: Node) -> Option<(bool, &'s str)> {
    let is_option: bool = read_node_kind("generic_type", node, source)
        .map(|g| g.contains("Option<"))
        .unwrap_or_default();

    let mut node = node;
    // if we are working with an option, step into the option and read its inner type
    if is_option {
        node = find_node_kind("type_arguments", node)?;
    }

    let ty = None
        .or_else(|| read_node_kind("generic_type", node, source))
        .or_else(|| read_node_kind("scoped_type_identifier", node, source))
        .or_else(|| read_node_kind("type_identifier", node, source))
        .or_else(|| read_node_kind("primitive_type", node, source));

    let ty = ty?;

    Some((is_option, ty))
}

#[cfg(test)]
mod tests;
