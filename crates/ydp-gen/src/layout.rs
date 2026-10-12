//! 有限值固定点和确定性直接存储图；所有图遍历均使用有界显式工作栈。
use crate::{Result, error};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
type Object = Map<String, Value>;
#[derive(Clone)]
struct Edge {
    target: String,
    path: String,
    direct: bool,
}
fn reference(value: &str, owner: &str) -> Result<String> {
    let pieces = value.split('.').collect::<Vec<_>>();
    let domain = owner.split('.').next().unwrap_or("");
    match pieces.as_slice() {
        [name] => Ok(format!("{domain}.type.{name}")),
        [domain, name] => Ok(format!("{domain}.type.{name}")),
        _ => Err(error(
            "Reference.Invalid",
            owner,
            "引用必须为 Type 或 Domain.Type",
        )),
    }
}
fn edges(owner: &str, node: &Object) -> Result<Vec<Edge>> {
    let mut result = Vec::new();
    let mut stack = vec![(node, owner.to_string(), true)];
    while let Some((node, path, direct)) = stack.pop() {
        if let Some(r) = node.get("$ref").and_then(Value::as_str) {
            result.push(Edge {
                target: reference(r, owner)?,
                path,
                direct,
            });
            continue;
        }
        if let Some(items) = node.get("items").and_then(Value::as_object) {
            stack.push((items, format!("{path}/items"), false));
        }
        if let Some(items) = node.get("additionalProperties").and_then(Value::as_object) {
            stack.push((items, format!("{path}/additionalProperties"), false));
        }
        if let Some(fields) = node.get("properties").and_then(Value::as_array) {
            for field in fields.iter().rev() {
                if let Some(field) = field.as_object() {
                    let name = field
                        .get("name")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .replace('~', "~0")
                        .replace('/', "~1");
                    stack.push((field, format!("{path}/properties/{name}"), direct));
                }
            }
        }
    }
    result.sort_by(|a, b| (&a.target, &a.path).cmp(&(&b.target, &b.path)));
    Ok(result)
}
fn finite(owner: &str, node: &Object, known: &BTreeSet<String>) -> Result<bool> {
    let mut stack = vec![node];
    while let Some(n) = stack.pop() {
        if n.get("optional").and_then(Value::as_bool) == Some(true)
            || n.get("x-cdp-rs")
                .and_then(Value::as_object)
                .and_then(|x| x.get("nullable"))
                .and_then(Value::as_bool)
                == Some(true)
        {
            continue;
        }
        if let Some(r) = n.get("$ref").and_then(Value::as_str) {
            if !known.contains(&reference(r, owner)?) {
                return Ok(false);
            }
            continue;
        }
        if n.get("type").and_then(Value::as_str) == Some("object")
            && let Some(fields) = n.get("properties").and_then(Value::as_array)
        {
            for f in fields {
                if let Some(f) = f.as_object() {
                    stack.push(f);
                }
            }
        }
    }
    Ok(true)
}
/// 验证全部引用与有限 JSON 可构造性，并返回需要 Box 的精确字段路径；模块测试通过编译器公开入口执行。
pub(super) fn plan(symbols: &BTreeMap<String, Object>) -> Result<BTreeSet<String>> {
    let mut graph = BTreeMap::new();
    let mut incoming: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (owner, node) in symbols {
        let e = edges(owner, node)?;
        for edge in &e {
            if !symbols.contains_key(&edge.target) {
                return Err(error(
                    "Reference.Missing",
                    &edge.path,
                    format!("无法解析 {}", edge.target),
                ));
            }
            incoming
                .entry(edge.target.clone())
                .or_default()
                .insert(owner.clone());
        }
        graph.insert(owner.clone(), e);
    }
    let mut known = BTreeSet::new();
    let mut queue = VecDeque::from_iter(symbols.keys().cloned());
    let mut queued = BTreeSet::from_iter(symbols.keys().cloned());
    while let Some(key) = queue.pop_front() {
        queued.remove(&key);
        if known.contains(&key) {
            continue;
        }
        if finite(&key, &symbols[&key], &known)? {
            known.insert(key.clone());
            if let Some(dependents) = incoming.get(&key) {
                for dependent in dependents {
                    if !known.contains(dependent) && queued.insert(dependent.clone()) {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }
    }
    if let Some(key) = symbols.keys().find(|key| !known.contains(*key)) {
        return Err(error(
            "Layout.NoFiniteValue",
            key,
            "required 引用环没有有限 JSON 基础值；请检查 optional/nullable 或容器边界",
        ));
    }
    let mut boxed = BTreeSet::new();
    for direct_only in [false, true] {
        let mut colors: BTreeMap<String, u8> = BTreeMap::new();
        let mut lengths: BTreeMap<String, usize> = BTreeMap::new();
        for start in symbols.keys() {
            if colors.contains_key(start) {
                continue;
            }
            colors.insert(start.clone(), 1);
            let mut stack = vec![(start.clone(), 0usize, 1usize)];
            while let Some((key, index, length)) = stack.pop() {
                let list = &graph[&key];
                if index == list.len() {
                    colors.insert(key.clone(), 2);
                    lengths.insert(key, length);
                    continue;
                }
                let edge = &list[index];
                if direct_only && !edge.direct {
                    stack.push((key, index + 1, length));
                    continue;
                }
                match colors.get(&edge.target).copied().unwrap_or(0) {
                    1 => {
                        if direct_only {
                            boxed.insert(edge.path.clone());
                        }
                        stack.push((key, index + 1, length));
                    }
                    2 => {
                        let depth = 1 + lengths.get(&edge.target).copied().unwrap_or(1);
                        if depth > 128 {
                            return Err(error(
                                "Layout.ResourceLimit",
                                &edge.path,
                                "引用展开深度超过 128",
                            ));
                        }
                        stack.push((key, index + 1, length.max(depth)));
                    }
                    _ => {
                        if stack.len() >= 127 {
                            return Err(error(
                                "Layout.ResourceLimit",
                                &edge.path,
                                "引用展开深度超过 128",
                            ));
                        }
                        stack.push((key, index, length));
                        colors.insert(edge.target.clone(), 1);
                        stack.push((edge.target.clone(), 0, 1));
                    }
                }
            }
        }
    }
    Ok(boxed)
}
