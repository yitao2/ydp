use crate::{
    GenerateOptions, GeneratedMetadata, GeneratedOutput, Result, SourceBundle, SourceMetadata,
    error, input,
};
use heck::{ToSnakeCase, ToUpperCamelCase};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

type Object = Map<String, Value>;
fn object<'a>(value: &'a Value, path: &str) -> Result<&'a Object> {
    value
        .as_object()
        .ok_or_else(|| error("Schema.Object", path, "此位置必须为 object"))
}
fn string<'a>(obj: &'a Object, key: &str, path: &str) -> Result<&'a str> {
    obj.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| error("Schema.String", path, format!("{key} 必须为 string")))
}
fn array<'a>(obj: &'a Object, key: &str, path: &str) -> Result<&'a [Value]> {
    match obj.get(key) {
        None => Ok(&[]),
        Some(Value::Array(a)) => Ok(a),
        _ => Err(error("Schema.Array", path, format!("{key} 必须为 array"))),
    }
}
fn flag(obj: &Object, key: &str, path: &str) -> Result<bool> {
    match obj.get(key) {
        None => Ok(false),
        Some(Value::Bool(x)) => Ok(*x),
        _ => Err(error(
            "Schema.Boolean",
            path,
            format!("{key} 必须为 boolean"),
        )),
    }
}
fn symbol(s: &str, path: &str) -> Result<()> {
    if s.is_empty() || s.chars().any(|c| c == '.' || c == '/' || c.is_control()) {
        Err(error(
            "Schema.Symbol",
            path,
            "符号不能空或包含点、斜杠、控制字符",
        ))
    } else {
        Ok(())
    }
}
fn pointer(s: &str) -> String {
    s.replace('~', "~0").replace('/', "~1")
}
fn literal(s: &str) -> String {
    format!("{s:?}")
}
fn doc(s: &str) -> String {
    s.split('\n')
        .map(|line| format!("#[doc = {}]\n", literal(line)))
        .collect()
}
// 描述是协议数据；转义 HTML 并隔离 Markdown，避免源描述成为可执行 doctest。
fn description_block(description: &str) -> String {
    let escaped = description
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    format!("<pre>{escaped}</pre>")
}
fn name(s: &str, camel: bool, path: &str) -> Result<String> {
    let mut n = if camel {
        s.to_upper_camel_case()
    } else {
        s.to_snake_case()
    };
    if matches!(n.as_str(), "self" | "Self" | "super" | "crate") {
        n.push('_');
    }
    if syn::parse_str::<syn::Ident>(&n).is_err() {
        n = format!("r#{n}");
    }
    if syn::parse_str::<syn::Ident>(&n).is_err() {
        return Err(error(
            "Naming.Identifier",
            path,
            "名称无法映射为 Rust 标识符，请配置 rust_names",
        ));
    }
    Ok(n)
}
#[derive(Clone)]
struct Domain {
    meta: Object,
    types: BTreeMap<String, Object>,
    commands: BTreeMap<String, Object>,
    events: BTreeMap<String, Object>,
}
struct Compiler<'a> {
    bundle: &'a SourceBundle,
    diagnostics: Vec<String>,
    names: BTreeMap<String, String>,
    symbols: BTreeMap<String, Object>,
    used_config: BTreeSet<String>,
    boxed: BTreeSet<String>,
}
impl Compiler<'_> {
    fn keys(&mut self, obj: &Object, allowed: &[&str], path: &str) -> Result<()> {
        for key in obj.keys() {
            if !allowed.contains(&key.as_str()) {
                if self.bundle.manifest.metadata_keys.contains(key) {
                    self.diagnostics
                        .push(format!("{path}: 保留非语义扩展 {key}"));
                } else {
                    return Err(error(
                        "Schema.UnknownKey",
                        path,
                        format!("不支持键 {key}，请使用明确支持的语义"),
                    ));
                }
            }
        }
        for k in ["experimental", "deprecated", "optional"] {
            flag(obj, k, path)?;
        }
        if let Some(v) = obj.get("description")
            && !v.is_string()
        {
            return Err(error(
                "Schema.Description",
                path,
                "description 必须为 string",
            ));
        }
        Ok(())
    }
    fn docs(&mut self, obj: &Object, path: &str, template: &str) -> Result<String> {
        let mut text = format!("{template}。协议来源：{path}。");
        if let Some(description) = obj.get("description").and_then(Value::as_str) {
            let normalized = description.replace("\r\n", "\n").replace('\r', "\n");
            text.push_str("\n\n协议原文：\n\n");
            text.push_str(&description_block(&normalized));
            if let Some(entry) = self.bundle.docs.get(path) {
                if entry.source_sha256 != input::digest(normalized.as_bytes())
                    || entry.text.trim().is_empty()
                {
                    return Err(error(
                        "Documentation.HashMismatch",
                        path,
                        "可选中文说明摘要失配或文本为空，请审查原文后更新或删除该补充",
                    ));
                }
                text.push_str("\n\n中文补充：\n\n");
                text.push_str(&description_block(&entry.text));
            }
            text.push_str("\n\n");
        }
        if flag(obj, "experimental", path)? {
            text.push_str("此项为 experimental，端点可能不支持。");
        }
        if flag(obj, "deprecated", path)? {
            text.push_str("此项已 deprecated，保留用于旧端点兼容。");
        }
        if let Some(target) = obj.get("redirect").and_then(Value::as_str) {
            text.push_str(&format!("此命令由协议重定向到 {target} 域处理。"));
        }
        Ok(text)
    }
    fn configured(&mut self, path: &str, fallback: &str, camel: bool) -> Result<String> {
        let value = match self.bundle.manifest.rust_names.get(path) {
            Some(v) => {
                self.used_config.insert(path.into());
                v.as_str()
            }
            None => fallback,
        };
        name(value, camel, path)
    }
    fn node(&mut self, obj: &Object, path: &str, role: &str) -> Result<()> {
        let mut keys = vec![
            "type",
            "$ref",
            "enum",
            "items",
            "properties",
            "additionalProperties",
            "x-cdp-rs",
            "description",
            "experimental",
            "deprecated",
        ];
        if role == "type" {
            keys.push("id");
        } else if role == "field" {
            keys.extend(["name", "optional"]);
        }
        self.keys(obj, &keys, path)?;
        let nullable = match obj.get("x-cdp-rs") {
            None => false,
            Some(v) => {
                let ext = object(v, path)?;
                if ext.len() != 1 || !ext.contains_key("nullable") {
                    return Err(error("Schema.Extension", path, "x-cdp-rs 只支持 nullable"));
                }
                flag(ext, "nullable", path)?
            }
        };
        if obj.contains_key("type") == obj.contains_key("$ref") {
            return Err(error("Schema.Shape", path, "必须恰好声明 type 或 $ref"));
        }
        if let Some(v) = obj.get("$ref") {
            if !v.is_string()
                || obj.keys().any(|k| {
                    ["enum", "items", "properties", "additionalProperties"].contains(&k.as_str())
                })
            {
                return Err(error(
                    "Schema.ReferenceShape",
                    path,
                    "$ref 必须为 string 且不能携带类型形状",
                ));
            }
            return Ok(());
        }
        let typ = string(obj, "type", path)?;
        if ![
            "string", "integer", "number", "boolean", "any", "object", "array", "binary",
        ]
        .contains(&typ)
        {
            return Err(error(
                "Schema.UnsupportedType",
                path,
                format!("不支持类型 {typ}"),
            ));
        }
        if nullable && typ == "any" {
            return Err(error(
                "Schema.NullableAny",
                path,
                "any 已允许 null，不能重复声明 nullable",
            ));
        }
        if obj.contains_key("enum") {
            if typ != "string" {
                return Err(error("Schema.Enum", path, "enum 仅用于 string"));
            }
            let vals = array(obj, "enum", path)?;
            let mut seen = BTreeSet::new();
            for v in vals {
                let s = v
                    .as_str()
                    .ok_or_else(|| error("Schema.Enum", path, "enum 值必须为 string"))?;
                if !seen.insert(s) {
                    return Err(error("Schema.Enum", path, "enum 值不能重复"));
                }
            }
        }
        if (typ != "array" && obj.contains_key("items"))
            || (typ != "object"
                && (obj.contains_key("properties") || obj.contains_key("additionalProperties")))
        {
            return Err(error("Schema.Shape", path, "类型与字段形状不兼容"));
        }
        if typ == "array" {
            let item = obj
                .get("items")
                .ok_or_else(|| error("Schema.Items", path, "array 必须声明 items"))?;
            self.node(object(item, path)?, &format!("{path}/items"), "inline")?;
        }
        if typ == "object" {
            self.fields(
                array(obj, "properties", path)?,
                &format!("{path}/properties"),
            )?;
            if let Some(v) = obj.get("additionalProperties") {
                if obj.contains_key("properties") {
                    return Err(error(
                        "Schema.MixedMap",
                        path,
                        "properties 与 additionalProperties 不得混用",
                    ));
                }
                if !v.is_boolean() {
                    self.node(
                        object(v, path)?,
                        &format!("{path}/additionalProperties"),
                        "inline",
                    )?;
                }
            }
        }
        Ok(())
    }
    fn fields(&mut self, fields: &[Value], path: &str) -> Result<()> {
        let mut seen = BTreeSet::new();
        for v in fields {
            let obj = object(v, path)?;
            let n = string(obj, "name", path)?;
            if n.is_empty() || !seen.insert(n) {
                return Err(error("Schema.Field", path, "字段名不能为空或重复"));
            }
            self.node(obj, &format!("{path}/{}", pointer(n)), "field")?;
        }
        Ok(())
    }
    fn merge(&mut self) -> Result<BTreeMap<String, Domain>> {
        let mut domains: BTreeMap<String, Domain> = BTreeMap::new();
        for source in &self.bundle.sources {
            let top = object(&source.value, &source.path)?;
            self.keys(top, &["version", "domains"], &source.path)?;
            if let Some(version) = top.get("version") {
                let version = object(version, &source.path)?;
                self.keys(version, &["major", "minor"], &source.path)?;
                string(version, "major", &source.path)?;
                string(version, "minor", &source.path)?;
            }
            for value in array(top, "domains", &source.path)? {
                let obj = object(value, &source.path)?;
                let domain = string(obj, "domain", &source.path)?;
                symbol(domain, domain)?;
                self.keys(
                    obj,
                    &[
                        "domain",
                        "dependencies",
                        "types",
                        "commands",
                        "events",
                        "description",
                        "experimental",
                        "deprecated",
                    ],
                    domain,
                )?;
                for dep in array(obj, "dependencies", domain)? {
                    symbol(
                        dep.as_str().ok_or_else(|| {
                            error("Schema.Dependency", domain, "dependencies 必须为字符串列表")
                        })?,
                        domain,
                    )?;
                }
                let mut meta = obj.clone();
                for key in ["types", "commands", "events"] {
                    meta.remove(key);
                }
                let entry = domains.entry(domain.into()).or_insert_with(|| Domain {
                    meta: meta.clone(),
                    types: BTreeMap::new(),
                    commands: BTreeMap::new(),
                    events: BTreeMap::new(),
                });
                for (key, value) in meta {
                    if key == "dependencies" {
                        let mut dependencies = BTreeSet::new();
                        for values in [entry.meta.get(&key), Some(&value)].into_iter().flatten() {
                            for dependency in values.as_array().into_iter().flatten() {
                                if let Some(dependency) = dependency.as_str() {
                                    dependencies.insert(dependency.to_string());
                                }
                            }
                        }
                        entry.meta.insert(
                            key,
                            Value::Array(dependencies.into_iter().map(Value::String).collect()),
                        );
                    } else if let Some(old) = entry.meta.get(&key) {
                        if normalize_model(old) != normalize_model(&value) {
                            return Err(error(
                                "Merge.DomainConflict",
                                domain,
                                format!("同域显式元数据 {key} 冲突"),
                            ));
                        }
                    } else {
                        entry.meta.insert(key, value);
                    }
                }
                for (list, kind) in [
                    ("types", "type"),
                    ("commands", "command"),
                    ("events", "event"),
                ] {
                    for value in array(obj, list, domain)? {
                        let item = object(value, domain)?;
                        let key = if kind == "type" { "id" } else { "name" };
                        let wire = string(item, key, domain)?;
                        symbol(wire, domain)?;
                        let path = format!("{domain}.{kind}.{wire}");
                        if kind == "type" {
                            self.node(item, &path, "type")?;
                        } else {
                            self.keys(
                                item,
                                &[
                                    "name",
                                    "parameters",
                                    "returns",
                                    "description",
                                    "experimental",
                                    "deprecated",
                                    "redirect",
                                ],
                                &path,
                            )?;
                            self.fields(
                                array(item, "parameters", &path)?,
                                &format!("{path}/parameters"),
                            )?;
                            self.fields(
                                array(item, "returns", &path)?,
                                &format!("{path}/returns"),
                            )?;
                            if kind == "event"
                                && (item.contains_key("returns") || item.contains_key("redirect"))
                            {
                                return Err(error(
                                    "Schema.Event",
                                    &path,
                                    "event 不允许 returns/redirect",
                                ));
                            }
                            if item.contains_key("redirect") {
                                string(item, "redirect", &path)?;
                            }
                        }
                        let map = match kind {
                            "type" => &mut entry.types,
                            "command" => &mut entry.commands,
                            _ => &mut entry.events,
                        };
                        if let Some(old) = map.get(wire) {
                            if normalize_model(&Value::Object(old.clone()))
                                != normalize_model(&Value::Object(item.clone()))
                            {
                                return Err(error(
                                    "Merge.SymbolConflict",
                                    &path,
                                    "同名符号定义不同，不能静默覆盖",
                                ));
                            }
                        } else {
                            map.insert(
                                wire.into(),
                                object(&normalize_model(&Value::Object(item.clone())), &path)?
                                    .clone(),
                            );
                        }
                    }
                }
            }
        }
        for (config, definition) in &self.bundle.overrides {
            let parts = config.target.split('.').collect::<Vec<_>>();
            let [domain, kind, wire] = parts.as_slice() else {
                return Err(error(
                    "Merge.OverrideTarget",
                    &config.target,
                    "目标必须为 Domain.type/command/event.Name",
                ));
            };
            let data = domains.get_mut(*domain).ok_or_else(|| {
                error("Merge.OverrideTarget", &config.target, "目标 Domain 不存在")
            })?;
            let table = match *kind {
                "type" => &mut data.types,
                "command" => &mut data.commands,
                "event" => &mut data.events,
                _ => {
                    return Err(error(
                        "Merge.OverrideTarget",
                        &config.target,
                        "目标 kind 必须为 type/command/event",
                    ));
                }
            };
            let previous = table
                .get(*wire)
                .ok_or_else(|| error("Merge.OverrideTarget", &config.target, "目标符号不存在"))?;
            let hash = crate::diff::semantic_hash(&Value::Object(previous.clone()))?;
            if !hash.eq_ignore_ascii_case(&config.expected_sha256) {
                return Err(error(
                    "Merge.OverrideHashMismatch",
                    &config.target,
                    format!("原语义摘要为 {hash}，请重新审查覆盖"),
                ));
            }
            match definition {
                None => {
                    table.remove(*wire);
                }
                Some(value) => {
                    let replacement = object(value, &config.target)?;
                    let key = if *kind == "type" { "id" } else { "name" };
                    if string(replacement, key, &config.target)? != *wire {
                        return Err(error(
                            "Merge.OverrideIdentity",
                            &config.target,
                            "替换定义必须保持原符号名字",
                        ));
                    }
                    if *kind == "type" {
                        self.node(replacement, &config.target, "type")?;
                    } else {
                        self.keys(
                            replacement,
                            &[
                                "name",
                                "parameters",
                                "returns",
                                "description",
                                "experimental",
                                "deprecated",
                                "redirect",
                            ],
                            &config.target,
                        )?;
                        self.fields(
                            array(replacement, "parameters", &config.target)?,
                            &format!("{}/parameters", config.target),
                        )?;
                        self.fields(
                            array(replacement, "returns", &config.target)?,
                            &format!("{}/returns", config.target),
                        )?;
                        if *kind == "event"
                            && (replacement.contains_key("returns")
                                || replacement.contains_key("redirect"))
                        {
                            return Err(error(
                                "Schema.Event",
                                &config.target,
                                "event 不允许 returns/redirect",
                            ));
                        }
                        if replacement.contains_key("redirect") {
                            string(replacement, "redirect", &config.target)?;
                        }
                    }
                    table.insert((*wire).into(), replacement.clone());
                }
            }
        }
        if domains
            .values()
            .map(|d| d.types.len() + d.commands.len() + d.events.len())
            .sum::<usize>()
            > 100_000
        {
            return Err(error(
                "Schema.ResourceLimit",
                "protocol",
                "符号数量超过 100000",
            ));
        }
        let mut mods = BTreeSet::new();
        for (domain, d) in &domains {
            let module = self.configured(domain, domain, false)?;
            if !mods.insert(module.clone()) {
                return Err(error("Naming.Collision", domain, "Domain 模块名冲突"));
            }
            self.names.insert(domain.clone(), module);
            let mut types = BTreeSet::new();
            for (wire, node) in &d.types {
                let path = format!("{domain}.type.{wire}");
                let n = self.configured(&path, wire, true)?;
                if !types.insert(n.clone()) {
                    return Err(error("Naming.Collision", &path, "类型名冲突"));
                }
                self.names.insert(path.clone(), n);
                self.symbols.insert(path, node.clone());
            }
        }
        Ok(domains)
    }
    fn reference(&self, reference: &str, domain: &str, path: &str) -> Result<(String, String)> {
        let parts = reference.split('.').collect::<Vec<_>>();
        let (d, n) = match parts.as_slice() {
            [n] => (domain, *n),
            [d, n] => (*d, *n),
            _ => {
                return Err(error(
                    "Reference.Invalid",
                    path,
                    "引用必须为 Type 或 Domain.Type",
                ));
            }
        };
        let key = format!("{d}.type.{n}");
        let rust = self
            .names
            .get(&key)
            .ok_or_else(|| error("Reference.Missing", path, format!("无法解析 {reference}")))?;
        let module = self
            .names
            .get(d)
            .ok_or_else(|| error("Reference.Missing", path, "引用 Domain 不存在"))?;
        Ok((key, format!("crate::{module}::types::{rust}")))
    }
    fn type_expr(
        &mut self,
        obj: &Object,
        domain: &str,
        path: &str,
        inline: &str,
        definitions: &mut String,
        namespace: &mut BTreeSet<String>,
    ) -> Result<String> {
        let runtime = &self.bundle.manifest.runtime_crate;
        let base = if let Some(reference) = obj.get("$ref").and_then(Value::as_str) {
            {
                let target = self.reference(reference, domain, path)?.1;
                if self.boxed.contains(path) {
                    format!("Box<{target}>")
                } else {
                    target
                }
            }
        } else {
            match string(obj, "type", path)? {
                "string" if obj.contains_key("enum") => {
                    self.enum_type(obj, path, inline, definitions, namespace)?;
                    inline.into()
                }
                "string" | "binary" => "String".into(),
                "integer" => "i64".into(),
                "number" => format!("{runtime}::FiniteF64"),
                "boolean" => "bool".into(),
                "any" => "serde_json::Value".into(),
                "array" => {
                    let item = object(&obj["items"], path)?;
                    let inner = self.type_expr(
                        item,
                        domain,
                        &format!("{path}/items"),
                        &format!("{inline}Item"),
                        definitions,
                        namespace,
                    )?;
                    format!("Vec<{inner}>")
                }
                "object" if obj.contains_key("properties") => {
                    self.structure(
                        inline,
                        array(obj, "properties", path)?,
                        domain,
                        &format!("{path}/properties"),
                        obj,
                        false,
                        definitions,
                        namespace,
                    )?;
                    inline.into()
                }
                "object" => match obj.get("additionalProperties") {
                    Some(Value::Bool(false)) => {
                        reserve(namespace, inline, path)?;
                        definitions.push_str(&doc(&self.docs(
                            obj,
                            path,
                            "仅接受空 object 的协议类型",
                        )?));
                        definitions.push_str(&format!("#[derive(Debug,Clone,Default,serde::Serialize)] pub struct {inline} {{}} impl<'de> serde::Deserialize<'de> for {inline} {{fn deserialize<D:serde::Deserializer<'de>>(d:D)->Result<Self,D::Error>{{struct Visitor;impl<'de> serde::de::Visitor<'de> for Visitor{{type Value={inline};fn expecting(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{{f.write_str(\"空 object\")}}fn visit_map<A:serde::de::MapAccess<'de>>(self,mut a:A)->Result<Self::Value,A::Error>{{if a.next_key::<serde::de::IgnoredAny>()?.is_some(){{return Err(serde::de::Error::custom(\"空 object 不允许字段\"));}}Ok({inline}{{}})}}}}d.deserialize_map(Visitor)}}}}\n"));
                        inline.into()
                    }
                    Some(Value::Object(v)) => {
                        let inner = self.type_expr(
                            v,
                            domain,
                            &format!("{path}/additionalProperties"),
                            &format!("{inline}Value"),
                            definitions,
                            namespace,
                        )?;
                        format!("std::collections::BTreeMap<String,{inner}>")
                    }
                    _ => "std::collections::BTreeMap<String,serde_json::Value>".into(),
                },
                _ => return Err(error("Schema.Shape", path, "尚未实现的类型形状")),
            }
        };
        let nullable = obj
            .get("x-cdp-rs")
            .and_then(Value::as_object)
            .and_then(|v| v.get("nullable"))
            .and_then(Value::as_bool)
            == Some(true);
        Ok(if nullable {
            format!(
                "{}::binding_support::Nullable<{base}>",
                self.bundle.manifest.runtime_crate
            )
        } else {
            base
        })
    }
    #[allow(clippy::too_many_arguments)]
    fn structure(
        &mut self,
        rust: &str,
        fields: &[Value],
        domain: &str,
        path: &str,
        source: &Object,
        params: bool,
        definitions: &mut String,
        namespace: &mut BTreeSet<String>,
    ) -> Result<()> {
        reserve(namespace, rust, path)?;
        let mut body = String::new();
        let mut fieldnames = BTreeSet::new();
        let mut field_list = Vec::new();
        let mut all_optional = true;
        for value in fields {
            let field = object(value, path)?;
            let wire = string(field, "name", path)?;
            let fieldpath = format!("{path}/{}", pointer(wire));
            let n = self.configured(&fieldpath, wire, false)?;
            if !fieldnames.insert(n.clone()) {
                return Err(error(
                    "Naming.FieldCollision",
                    &fieldpath,
                    "字段 Rust 名冲突，请配置 rust_names",
                ));
            }
            field_list.push(n.clone());
            let inline = format!("{rust}{}", name(wire, true, &fieldpath)?);
            let ty = self.type_expr(field, domain, &fieldpath, &inline, definitions, namespace)?;
            let optional = flag(field, "optional", &fieldpath)?;
            all_optional &= optional;
            body.push_str(&doc(&self.docs(
                field,
                &fieldpath,
                if optional {
                    "可缺失字段；None 编码时省略，存在时按声明类型校验"
                } else {
                    "必需字段；缺失或类型错误导致解码失败"
                },
            )?));
            let runtime = &self.bundle.manifest.runtime_crate;
            body.push_str(&format!("#[serde(rename={})]\n", literal(wire)));
            if optional {
                body.push_str(&format!("#[serde(default,deserialize_with=\"{runtime}::binding_support::presence\",skip_serializing_if=\"Option::is_none\")] pub {n}: Option<{ty}>,\n"));
            } else {
                body.push_str(&format!("#[serde(deserialize_with=\"{runtime}::binding_support::required\")] pub {n}: {ty},\n"));
            }
        }
        definitions.push_str(&doc(&self.docs(
            source,
            path.strip_suffix("/properties").unwrap_or(path),
            "拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留",
        )?));
        let default = if params && all_optional {
            ",Default"
        } else {
            ""
        };
        definitions.push_str(&format!(
            "#[derive(Debug,Clone,serde::Serialize{default})] pub struct {rust} {{{body}}}\n"
        ));
        let binding = if field_list.is_empty() {
            "_fields"
        } else {
            "fields"
        };
        let assignments = field_list
            .iter()
            .map(|n| format!("{n}:fields.{n},"))
            .collect::<String>();
        definitions.push_str(&format!("impl<'de> serde::Deserialize<'de> for {rust} {{fn deserialize<D:serde::Deserializer<'de>>(d:D)->Result<Self,D::Error>{{struct Visitor;impl<'de> serde::de::Visitor<'de> for Visitor{{type Value={rust};fn expecting(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{{f.write_str(\"协议 object\")}}fn visit_map<A:serde::de::MapAccess<'de>>(self,a:A)->Result<Self::Value,A::Error>{{#[derive(serde::Deserialize)] struct Fields {{{body}}}let {binding}=<Fields as serde::Deserialize>::deserialize(serde::de::value::MapAccessDeserializer::new(a))?;Ok({rust}{{{assignments}}})}}}}d.deserialize_map(Visitor)}}}}"));
        Ok(())
    }
    fn enum_type(
        &mut self,
        obj: &Object,
        path: &str,
        rust: &str,
        definitions: &mut String,
        namespace: &mut BTreeSet<String>,
    ) -> Result<()> {
        reserve(namespace, rust, path)?;
        let mut variants = String::new();
        let mut arms = String::new();
        let mut parse = String::new();
        let mut used = BTreeSet::from(["Unknown".to_string()]);
        for value in array(obj, "enum", path)? {
            let wire = value
                .as_str()
                .ok_or_else(|| error("Schema.Enum", path, "enum 必须为字符串"))?;
            let mut variant = name(wire, true, path)?;
            if variant == "Unknown" {
                variant = "KnownUnknown".into();
            }
            if !used.insert(variant.clone()) {
                return Err(error("Naming.EnumCollision", path, "枚举变体名称冲突"));
            }
            variants.push_str(&format!(
                "{} {variant},",
                doc(&format!("协议值 {}。", literal(wire)))
            ));
            arms.push_str(&format!("Self::{variant}=>{},", literal(wire)));
            parse.push_str(&format!("{}=>{rust}::{variant},", literal(wire)));
        }
        let krate = &self.bundle.manifest.generated_crate;
        let domain = path.split('.').next().unwrap_or("");
        let module = &self.names[domain];
        let role = if path.contains(".command.") {
            "commands"
        } else if path.contains(".event.") {
            "events"
        } else {
            "types"
        };
        let full = format!("{krate}::{module}::{role}::{rust}");
        definitions.push_str(&doc(&self.docs(
            obj,
            path,
            "开放字符串枚举；已知值不分配 String，未知值拥有原始文本",
        )?));
        definitions.push_str(&format!("#[derive(Debug,Clone,PartialEq,Eq)] #[non_exhaustive] pub enum {rust} {{{variants} {} Unknown(String)}}",doc("未知协议文本；Clone 复制字符串，编码保持原值。")));
        definitions.push_str(&format!("impl {rust} {{{} pub fn as_str(&self)->&str {{match self {{{arms} Self::Unknown(value)=>value}}}}}}",doc(&format!("借用原始协议字符串，生命周期绑定自身。\n\n```\nlet value = {full}::from(String::from(\"future-value\"));\nassert_eq!(value.as_str(), \"future-value\");\n```"))));
        definitions.push_str(&format!("impl AsRef<str> for {rust} {{fn as_ref(&self)->&str {{self.as_str()}}}} impl From<String> for {rust} {{fn from(value:String)->Self {{match value.as_str() {{{parse} _=>Self::Unknown(value)}}}}}} impl serde::Serialize for {rust} {{fn serialize<S:serde::Serializer>(&self,s:S)->Result<S::Ok,S::Error>{{s.serialize_str(self.as_str())}}}} impl<'de> serde::Deserialize<'de> for {rust} {{fn deserialize<D:serde::Deserializer<'de>>(d:D)->Result<Self,D::Error>{{struct Visitor;impl<'de> serde::de::Visitor<'de> for Visitor {{type Value={rust};fn expecting(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{{f.write_str(\"协议字符串枚举\")}}fn visit_str<E:serde::de::Error>(self,v:&str)->Result<Self::Value,E>{{Ok(match v {{{parse} _=>{rust}::Unknown(v.to_owned())}})}} fn visit_string<E:serde::de::Error>(self,v:String)->Result<Self::Value,E>{{Ok({rust}::from(v))}}}}d.deserialize_str(Visitor)}}}}"));
        Ok(())
    }
    fn example_value(&self, obj: &Object, domain: &str, path: &str, depth: usize) -> Result<Value> {
        if depth > 128 {
            return Err(error("Layout.ResourceLimit", path, "示例展开超过 128 层"));
        }
        if obj
            .get("x-cdp-rs")
            .and_then(Value::as_object)
            .and_then(|v| v.get("nullable"))
            .and_then(Value::as_bool)
            == Some(true)
        {
            return Ok(Value::Null);
        }
        if let Some(r) = obj.get("$ref").and_then(Value::as_str) {
            let (key, _) = self.reference(r, domain, path)?;
            let target = &self.symbols[&key];
            let owner = key.split('.').next().unwrap_or(domain);
            return self.example_value(target, owner, path, depth + 1);
        }
        Ok(match string(obj, "type", path)? {
            "string" | "binary" => array(obj, "enum", path)?
                .first()
                .cloned()
                .unwrap_or(Value::String(String::new())),
            "integer" => Value::from(0),
            "number" => Value::from(0.0),
            "boolean" => Value::Bool(false),
            "any" => Value::Null,
            "array" => Value::Array(vec![]),
            "object" => {
                let mut fields = Object::new();
                for value in array(obj, "properties", path)? {
                    let field = object(value, path)?;
                    if !flag(field, "optional", path)? {
                        fields.insert(
                            string(field, "name", path)?.into(),
                            self.example_value(field, domain, path, depth + 1)?,
                        );
                    }
                }
                Value::Object(fields)
            }
            _ => return Err(error("Schema.Shape", path, "不能构造类型示例")),
        })
    }
    fn named(
        &mut self,
        obj: &Object,
        domain: &str,
        path: &str,
        rust: &str,
        definitions: &mut String,
        namespace: &mut BTreeSet<String>,
    ) -> Result<()> {
        let nullable = obj
            .get("x-cdp-rs")
            .and_then(Value::as_object)
            .and_then(|v| v.get("nullable"))
            .and_then(Value::as_bool)
            == Some(true);
        if !nullable
            && obj.get("type").and_then(Value::as_str) == Some("string")
            && obj.contains_key("enum")
        {
            return self.enum_type(obj, path, rust, definitions, namespace);
        }
        if !nullable
            && obj.get("type").and_then(Value::as_str) == Some("object")
            && obj.contains_key("properties")
        {
            return self.structure(
                rust,
                array(obj, "properties", path)?,
                domain,
                &format!("{path}/properties"),
                obj,
                false,
                definitions,
                namespace,
            );
        }
        reserve(namespace, rust, path)?;
        let ty = self.type_expr(
            obj,
            domain,
            path,
            &format!("{rust}Value"),
            definitions,
            namespace,
        )?;
        definitions.push_str(&doc(&self.docs(
            obj,
            path,
            "命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制",
        )?));
        definitions.push_str(&format!("#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)] #[serde(transparent)] pub struct {rust}({ty});"));
        let module = &self.names[domain];
        let krate = &self.bundle.manifest.generated_crate;
        let full = format!("{krate}::{module}::types::{rust}");
        let json = serde_json::to_string(&self.example_value(obj, domain, path, 0)?)
            .map_err(|e| error("Output.Example", path, e.to_string()))?;
        let example = format!(
            "\n\n```\nlet value: {full} = serde_json::from_str({}).unwrap();\nlet inner = value.into_inner();\nlet restored = {full}::new(inner);\nassert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>({}).unwrap());\n```",
            literal(&json),
            literal(&json)
        );
        definitions.push_str(&format!("impl {rust} {{{} pub fn new(value:{ty})->Self{{Self(value)}} {} pub fn into_inner(self)->{ty}{{self.0}}",doc(&format!("消耗底层值构造命名包装，不新增验证或额外分配。{example}")),doc(&format!("消耗包装并移出底层拥有型值，不复制数据。{example}"))));
        if ty == "String" {
            definitions.push_str(&format!("{} pub fn as_str(&self)->&str{{&self.0}}",doc(&format!("借用字符串；返回值不长于包装的生命周期。\n\n```\nlet value = {full}::new(String::from(\"示例\"));\nassert_eq!(value.as_str(), \"示例\");\n```"))));
        }
        definitions.push('}');
        Ok(())
    }
    fn domain(
        &mut self,
        domain: &str,
        data: &Domain,
        files: &mut BTreeMap<String, String>,
    ) -> Result<String> {
        let module = self.names[domain].clone();
        let file_module = module.strip_prefix("r#").unwrap_or(&module);
        let runtime = self.bundle.manifest.runtime_crate.clone();
        let krate = self.bundle.manifest.generated_crate.clone();
        let mut facade = String::new();
        let mut methods = BTreeSet::new();
        let mut types = String::new();
        let mut namespace = BTreeSet::new();
        for (wire, obj) in &data.types {
            let path = format!("{domain}.type.{wire}");
            let rust = self.names[&path].clone();
            self.named(obj, domain, &path, &rust, &mut types, &mut namespace)?;
        }
        files.insert(format!("{file_module}/types.rs"), formatted(&types)?);
        for (kind, items) in [("command", &data.commands), ("event", &data.events)] {
            let mut definitions = String::new();
            let mut namespace = BTreeSet::new();
            for (wire, obj) in items {
                let path = format!("{domain}.{kind}.{wire}");
                let rust = self.configured(&path, wire, true)?;
                let method = self.configured(&path, wire, false)?;
                reserve(&mut namespace, &rust, &path)?;
                let params = format!("{rust}Params");
                self.structure(
                    &params,
                    array(obj, "parameters", &path)?,
                    domain,
                    &format!("{path}/parameters"),
                    &Object::new(),
                    true,
                    &mut definitions,
                    &mut namespace,
                )?;
                definitions.push_str(&doc(&self.docs(
                    obj,
                    &path,
                    "无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为",
                )?));
                definitions.push_str(&format!("#[derive(Debug,Clone,Copy)] pub struct {rust};"));
                let methodwire = literal(&format!("{domain}.{wire}"));
                if kind == "command" {
                    let response = format!("{rust}Response");
                    self.structure(
                        &response,
                        array(obj, "returns", &path)?,
                        domain,
                        &format!("{path}/returns"),
                        &Object::new(),
                        false,
                        &mut definitions,
                        &mut namespace,
                    )?;
                    definitions.push_str(&format!("impl {runtime}::Command for {rust} {{type Params={params};type Response={response};const METHOD:&'static str={methodwire};}}"));
                    if !methods.insert(method.clone()) {
                        return Err(error(
                            "Naming.FacadeCollision",
                            &path,
                            "Domain 方法归一后冲突",
                        ));
                    }
                    let mut method_docs = self.docs(
                        obj,
                        &path,
                        "拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行",
                    )?;
                    method_docs.push_str(&format!(
                        "\n\n```no_run\nuse {krate}::SessionDomains as _;\n# async fn call(session: &{runtime}::Session, params: {krate}::{module}::commands::{params}) -> {runtime}::Result<()> {{\nlet response = session.{module}().{method}(params).await?;\n# Ok(())\n# }}\n```"
                    ));
                    facade.push_str(&format!("{} pub fn {method}(&self,params:commands::{params})->{runtime}::Request<commands::{rust}>{{self.session.send::<commands::{rust}>(params)}}",doc(&method_docs)));
                } else {
                    definitions.push_str(&format!("impl {runtime}::Event for {rust}{{type Params={params};const METHOD:&'static str={methodwire};}}"));
                }
            }
            files.insert(
                format!("{file_module}/{}s.rs", kind),
                formatted(&definitions)?,
            );
        }
        let module_src = format!(
            "{} pub mod types; {} pub mod commands; {} pub mod events; {} pub struct Domain<'a> {{#[doc=\"借用的通信路由，生命周期不超过 Session。\"] pub(super) session:&'a {runtime}::Session}} impl<'a> Domain<'a>{{{facade}}}",
            doc("命名协议类型；不包含连接状态。"),
            doc("命令 marker、完整参数与成功响应。"),
            doc("事件 marker 和拥有型事件参数。"),
            doc(&self.docs(
                &data.meta,
                domain,
                "薄 Domain 视图，仅借用 Session；创建不发送数据"
            )?)
        );
        files.insert(format!("{file_module}/mod.rs"), formatted(&module_src)?);
        let example = format!(
            "借用 Session 创建 {domain} 域视图，不分配远端资源。\n\n```no_run\nuse {krate}::SessionDomains as _;\n# fn view(session:&{runtime}::Session) {{\nlet domain = session.{module}();\n# }}\n```"
        );
        Ok(format!(
            "{} fn {module}(&self)->{module}::Domain<'_>;",
            doc(&example)
        ))
    }
}
fn reserve(names: &mut BTreeSet<String>, name: &str, path: &str) -> Result<()> {
    if names.insert(name.into()) {
        Ok(())
    } else {
        Err(error(
            "Naming.Collision",
            path,
            format!("生成符号 {name} 冲突，请配置 rust_names"),
        ))
    }
}
fn formatted(source: &str) -> Result<String> {
    let ast = syn::parse_file(source)
        .map_err(|e| error("Output.RustSyntax", "generated", e.to_string()))?;
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--config", "skip_children=true"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| error("Output.Formatter", "rustfmt", e.to_string()))?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| error("Output.Formatter", "rustfmt", "未取得格式化输入"))?;
    stdin
        .write_all(prettyplease::unparse(&ast).as_bytes())
        .map_err(|e| error("Output.Formatter", "rustfmt", e.to_string()))?;
    drop(stdin);
    let output = child
        .wait_with_output()
        .map_err(|e| error("Output.Formatter", "rustfmt", e.to_string()))?;
    if !output.status.success() {
        return Err(error(
            "Output.Formatter",
            "rustfmt",
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }
    let formatted = String::from_utf8(output.stdout)
        .map_err(|e| error("Output.Formatter", "rustfmt", e.to_string()))?;
    Ok(formatted.replace("\r\n", "\n").replace('\r', "\n"))
}
/// 统一语义检查和 Rust 输出，不执行文件写入；集成测试对独立生成进行字节比较。
pub(super) fn compile(
    bundle: &SourceBundle,
    _options: &GenerateOptions,
) -> Result<GeneratedOutput> {
    let mut c = Compiler {
        bundle,
        diagnostics: Vec::new(),
        names: BTreeMap::new(),
        symbols: BTreeMap::new(),
        used_config: BTreeSet::new(),
        boxed: BTreeSet::new(),
    };
    let domains = c.merge()?;
    c.boxed = crate::layout::plan(&c.symbols)?;
    let mut files = BTreeMap::new();
    let mut root = String::new();
    let mut trait_methods = String::new();
    let mut implementations = String::new();
    for (domain, data) in &domains {
        let module = c.names[domain].clone();
        root.push_str(&format!(
            "{} pub mod {module};",
            doc(&format!("{domain} 协议域；所有调用共享同一个 core。"))
        ));
        trait_methods.push_str(&c.domain(domain, data, &mut files)?);
        implementations.push_str(&format!(
            "fn {module}(&self)->{module}::Domain<'_>{{{module}::Domain{{session:self}}}}"
        ));
    }
    root.push_str(&format!("{} pub trait SessionDomains {{{trait_methods}}} impl SessionDomains for {}::Session {{{implementations}}}",doc("生成的薄 Domain 扩展；只借用 Session，不缓存或解释浏览器状态。"),bundle.manifest.runtime_crate));
    for path in bundle.manifest.rust_names.keys() {
        if !c.used_config.contains(path) {
            return Err(error(
                "Naming.UnknownOverride",
                path,
                "rust_names 未匹配任何可命名符号",
            ));
        }
    }
    let inputs = bundle
        .sources
        .iter()
        .map(|s| {
            Ok(SourceMetadata {
                path: s.path.clone(),
                sha256: s.sha256.clone(),
                version: serde_json::from_value(
                    s.value
                        .get("version")
                        .cloned()
                        .unwrap_or_else(|| Value::Object(Object::new())),
                )
                .map_err(|e| error("Schema.Version", &s.path, e.to_string()))?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let metadata = GeneratedMetadata {
        generated_contract_version: 1,
        generator_version: env!("CARGO_PKG_VERSION").into(),
        binding_package: bundle.manifest.generated_crate.clone(),
        inputs,
        configuration_sha256: input::configuration(bundle)?,
    };
    let provenance = serde_json::to_string_pretty(&metadata)
        .map_err(|e| error("Output.Metadata", "provenance", e.to_string()))?
        + "\n";
    root.push_str(&format!("{} pub struct GeneratedMetadata {{{} pub generated_contract_version:u32,{} pub generator_version:&'static str,{} pub binding_package:&'static str,{} pub inputs:&'static [SourceMetadata],{} pub configuration_sha256:&'static str}}",doc("只读生成来源信息；不包含 runtime 方法查找表。"),doc("生成契约版本。"),doc("生成器包版本。"),doc("绑定 crate 名称。"),doc("每个输入自己的路径、版本和摘要。"),doc("配置和说明目录的 SHA-256。")));
    root.push_str(&format!("{} pub struct SourceMetadata {{{} pub path:&'static str,{} pub sha256:&'static str,{} pub version_major:&'static str,{} pub version_minor:&'static str}}",doc("一个固定源文件的独立 provenance；不编造浏览器版本。"),doc("manifest 相对路径。"),doc("原始字节 SHA-256。"),doc("源协议主版本字符串。"),doc("源协议次版本字符串。")));
    let mut sources = String::new();
    for source in &metadata.inputs {
        sources.push_str(&format!(
            "SourceMetadata{{path:{},sha256:{},version_major:{},version_minor:{}}},",
            literal(&source.path),
            literal(&source.sha256),
            literal(
                source
                    .version
                    .get("major")
                    .map(String::as_str)
                    .unwrap_or("")
            ),
            literal(
                source
                    .version
                    .get("minor")
                    .map(String::as_str)
                    .unwrap_or("")
            )
        ));
    }
    root.push_str(&format!("{} pub const GENERATED_METADATA: GeneratedMetadata = GeneratedMetadata{{generated_contract_version:1,generator_version:{},binding_package:{},inputs:&[{sources}],configuration_sha256:{}}};",doc("固定生成快照的只读元数据；所有字符串静态存储，无运行时加载或分配。"),literal(&metadata.generator_version),literal(&metadata.binding_package),literal(&metadata.configuration_sha256)));
    files.insert("mod.rs".into(), formatted(&root)?);
    files.insert("provenance.json".into(), provenance);
    let owned = files
        .iter()
        .map(|(path, contents)| (path.clone(), input::digest(contents.as_bytes())))
        .collect::<BTreeMap<_, _>>();
    files.insert(
        ".ydp-gen-files.json".into(),
        serde_json::to_string_pretty(&owned)
            .map_err(|e| error("Output.Inventory", "generated", e.to_string()))?
            + "\n",
    );
    if files.values().map(String::len).sum::<usize>() > 256 * 1024 * 1024 {
        return Err(error(
            "Output.ResourceLimit",
            "generated",
            "生成源码超过 256 MiB",
        ));
    }
    c.diagnostics.sort();
    c.diagnostics.dedup();
    let release_ready = true;
    Ok(GeneratedOutput {
        files,
        diagnostics: c.diagnostics,
        release_ready,
    })
}

/// 返回规范符号定义与 Rust 名字配置，diff 和 PDL 投影测试共享实际合并及引用校验。
pub(super) fn snapshot(bundle: &SourceBundle) -> Result<BTreeMap<String, (Value, String)>> {
    let mut c = Compiler {
        bundle,
        diagnostics: Vec::new(),
        names: BTreeMap::new(),
        symbols: BTreeMap::new(),
        used_config: BTreeSet::new(),
        boxed: BTreeSet::new(),
    };
    let domains = c.merge()?;
    crate::layout::plan(&c.symbols)?;
    let mut result = BTreeMap::new();
    for (domain, data) in domains {
        for (kind, nodes) in [
            ("type", data.types),
            ("command", data.commands),
            ("event", data.events),
        ] {
            for (wire, node) in nodes {
                let path = format!("{domain}.{kind}.{wire}");
                let mut stack = vec![Value::Object(node.clone())];
                while let Some(value) = stack.pop() {
                    match value {
                        Value::Object(object) => {
                            if let Some(reference) = object.get("$ref").and_then(Value::as_str) {
                                c.reference(reference, &domain, &path)?;
                            }
                            stack.extend(object.into_values());
                        }
                        Value::Array(values) => stack.extend(values),
                        _ => {}
                    }
                }
                let rust = c.configured(&path, &wire, true)?;
                let config = bundle
                    .manifest
                    .rust_names
                    .iter()
                    .filter(|(k, _)| {
                        *k == &path || k.starts_with(&format!("{path}/")) || *k == &domain
                    })
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<_>>()
                    .join(";");
                result.insert(path, (Value::Object(node), format!("{rust};{config}")));
            }
        }
    }
    Ok(result)
}

fn normalize_model(value: &Value) -> Value {
    match value {
        Value::Object(object) => Value::Object(
            object
                .iter()
                .filter_map(|(key, value)| {
                    if ["optional", "experimental", "deprecated"].contains(&key.as_str())
                        && value == &Value::Bool(false)
                    {
                        return None;
                    }
                    if ["parameters", "returns"].contains(&key.as_str())
                        && value == &Value::Array(Vec::new())
                    {
                        return None;
                    }
                    let normalized = if key == "description" {
                        value
                            .as_str()
                            .map(|s| Value::String(s.replace("\r\n", "\n").replace('\r', "\n")))
                            .unwrap_or_else(|| value.clone())
                    } else if key == "type" && value.as_str() == Some("binary") {
                        Value::String("string".into())
                    } else {
                        normalize_model(value)
                    };
                    Some((key.clone(), normalized))
                })
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.iter().map(normalize_model).collect()),
        _ => value.clone(),
    }
}

#[cfg(test)]
mod tests {
    // 防止多平台格式化换行符不统一
    #[test]
    fn formatted_output_uses_lf_on_all_platforms() {
        let output = super::formatted("").unwrap();

        assert_eq!(output, "\n");
        assert!(!output.contains('\r'));
    }
}
