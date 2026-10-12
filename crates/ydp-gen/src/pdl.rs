//! PDL 缩进语法解析，输出统一 JSON 语义节点；未知非注释行一律报错。
use crate::{Result, error};
use serde_json::{Map, Value, json};
struct Line {
    text: String,
    description: String,
    children: Vec<usize>,
    number: usize,
}
struct Parser<'a> {
    file: &'a str,
    lines: Vec<Line>,
}
impl Parser<'_> {
    fn fail(&self, id: usize, message: &str) -> crate::GeneratorError {
        error(
            "Parse.PDL",
            format!("{}:{}", self.file, self.lines[id].number),
            message,
        )
    }
    fn leaf(&self, id: usize) -> Result<()> {
        if self.lines[id].children.is_empty() {
            Ok(())
        } else {
            Err(self.fail(id, "此声明不允许子项"))
        }
    }
    fn head(&self, id: usize) -> Result<(Map<String, Value>, Vec<&str>)> {
        let mut words = self.lines[id].text.split_whitespace().collect::<Vec<_>>();
        let mut obj = Map::new();
        while words
            .first()
            .is_some_and(|w| ["experimental", "deprecated", "optional"].contains(w))
        {
            let modifier = words.remove(0);
            if obj.insert(modifier.into(), Value::Bool(true)).is_some() {
                return Err(self.fail(id, "修饰词不能重复"));
            }
        }
        if !self.lines[id].description.is_empty() {
            obj.insert(
                "description".into(),
                Value::String(self.lines[id].description.clone()),
            );
        }
        Ok((obj, words))
    }
    fn shape(&self, id: usize, words: &[&str]) -> Result<Map<String, Value>> {
        if words.first() == Some(&"array") {
            if words.len() < 3 || words[1] != "of" {
                return Err(self.fail(id, "数组声明必须为 array of Type"));
            }
            return Ok(Map::from_iter([
                ("type".into(), json!("array")),
                ("items".into(), Value::Object(self.shape(id, &words[2..])?)),
            ]));
        }
        if words.len() != 1 {
            return Err(self.fail(id, "类型声明必须为 primitive 或引用"));
        }
        let typ = words[0];
        if typ == "enum" {
            return Ok(Map::from_iter([("type".into(), json!("string"))]));
        }
        Ok(Map::from_iter([(
            if [
                "string", "integer", "number", "boolean", "any", "object", "binary",
            ]
            .contains(&typ)
            {
                "type"
            } else {
                "$ref"
            }
            .into(),
            json!(typ),
        )]))
    }
    fn enum_values(&self, id: usize) -> Result<Value> {
        let mut values = Vec::new();
        for &child in &self.lines[id].children {
            self.leaf(child)?;
            values.push(json!(self.lines[child].text));
        }
        Ok(Value::Array(values))
    }
    fn field(&self, id: usize) -> Result<Value> {
        let (mut obj, words) = self.head(id)?;
        if words.len() < 2 {
            return Err(self.fail(id, "字段需要类型和名称"));
        }
        let name = words[words.len() - 1];
        obj.insert("name".into(), json!(name));
        obj.extend(self.shape(id, &words[..words.len() - 1])?);
        if words[..words.len() - 1] == ["enum"] {
            obj.insert("enum".into(), self.enum_values(id)?);
        } else if !self.lines[id].children.is_empty() {
            return Err(self.fail(id, "非 enum 字段不允许直接嵌套声明"));
        }
        Ok(Value::Object(obj))
    }
    fn section(&self, id: usize) -> Result<Value> {
        let mut fields = Vec::new();
        for &child in &self.lines[id].children {
            fields.push(self.field(child)?);
        }
        Ok(Value::Array(fields))
    }
    fn member(&self, id: usize) -> Result<(&'static str, Value)> {
        let (mut obj, words) = self.head(id)?;
        if obj.contains_key("optional") {
            return Err(self.fail(id, "类型、命令和事件不允许 optional"));
        }
        match words.as_slice() {
            ["type", name, "extends", shape @ ..] if !shape.is_empty() => {
                obj.insert("id".into(), json!(name));
                obj.extend(self.shape(id, shape)?);
                for &child in &self.lines[id].children {
                    let key = self.lines[child].text.as_str();
                    let value = match key {
                        "properties" => self.section(child)?,
                        "enum" => self.enum_values(child)?,
                        _ => return Err(self.fail(child, "type 子项必须为 properties 或 enum")),
                    };
                    if obj.insert(key.into(), value).is_some() {
                        return Err(self.fail(child, "type 子项重复"));
                    }
                }
                Ok(("types", Value::Object(obj)))
            }
            [kind @ ("command" | "event"), name] => {
                obj.insert("name".into(), json!(name));
                for &child in &self.lines[id].children {
                    let text = &self.lines[child].text;
                    let pieces = text.split_whitespace().collect::<Vec<_>>();
                    let (key, value) = match pieces.as_slice() {
                        [key @ ("parameters" | "returns")]
                            if *kind == "command" || *key == "parameters" =>
                        {
                            (*key, self.section(child)?)
                        }
                        ["redirect", target] if *kind == "command" => {
                            self.leaf(child)?;
                            ("redirect", json!(target))
                        }
                        _ => return Err(self.fail(child, "命令/事件子项不受支持")),
                    };
                    if obj.insert(key.into(), value).is_some() {
                        return Err(self.fail(child, "命令/事件子项重复"));
                    }
                }
                Ok((
                    if *kind == "command" {
                        "commands"
                    } else {
                        "events"
                    },
                    Value::Object(obj),
                ))
            }
            _ => Err(self.fail(id, "未知 Domain 声明；需要 type extends、command 或 event")),
        }
    }
    fn domain(&self, id: usize) -> Result<Value> {
        let (mut obj, words) = self.head(id)?;
        if obj.contains_key("optional") {
            return Err(self.fail(id, "Domain 不允许 optional"));
        }
        let ["domain", name] = words.as_slice() else {
            return Err(self.fail(id, "Domain 声明错误"));
        };
        obj.insert("domain".into(), json!(name));
        for &child in &self.lines[id].children {
            let text = &self.lines[child].text;
            if let Some(dependency) = text.strip_prefix("depends on ") {
                self.leaf(child)?;
                let values = obj.entry("dependencies").or_insert_with(|| json!([]));
                if let Some(values) = values.as_array_mut() {
                    values.push(json!(dependency));
                }
            } else {
                let (kind, value) = self.member(child)?;
                let values = obj.entry(kind).or_insert_with(|| json!([]));
                if let Some(values) = values.as_array_mut() {
                    values.push(value);
                }
            }
        }
        Ok(Value::Object(obj))
    }
}
/// 解析单个 PDL 文件，返回统一语义对象和相对 include 路径；文件系统边界由 input 层负责。
pub(super) fn parse(text: &str, file: &str) -> Result<(Value, Vec<String>)> {
    let mut parser = Parser {
        file,
        lines: Vec::new(),
    };
    let mut roots = Vec::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut comments = Vec::new();
    for (number, raw) in text.lines().enumerate() {
        if raw.trim().is_empty() {
            comments.clear();
            continue;
        }
        if raw.contains('\t') {
            return Err(error(
                "Parse.PDL",
                format!("{file}:{}", number + 1),
                "PDL 缩进不能使用制表符",
            ));
        }
        let trimmed = raw.trim_start();
        let indent = raw.len() - trimmed.len();
        if let Some(comment) = trimmed.strip_prefix('#') {
            comments.push(comment.strip_prefix(' ').unwrap_or(comment).to_owned());
            continue;
        }
        while stack.last().is_some_and(|(i, _)| *i >= indent) {
            stack.pop();
        }
        if stack.is_empty() && indent != 0 {
            return Err(error(
                "Parse.PDL",
                format!("{file}:{}", number + 1),
                "顶层声明必须无缩进",
            ));
        }
        if parser.lines.len() >= 1_000_000 || stack.len() > 128 {
            return Err(error(
                "Schema.ResourceLimit",
                file,
                "PDL 节点或缩进深度超限",
            ));
        }
        let id = parser.lines.len();
        parser.lines.push(Line {
            text: trimmed.trim_end().into(),
            description: std::mem::take(&mut comments).join("\n"),
            children: Vec::new(),
            number: number + 1,
        });
        if let Some((_, parent)) = stack.last() {
            parser.lines[*parent].children.push(id);
        } else {
            roots.push(id);
        }
        stack.push((indent, id));
    }
    let mut top = Map::new();
    let mut domains = Vec::new();
    let mut includes = Vec::new();
    for id in roots {
        let words = parser.lines[id].text.split_whitespace().collect::<Vec<_>>();
        match words.as_slice() {
            ["version"] => {
                let mut version = Map::new();
                for &child in &parser.lines[id].children {
                    parser.leaf(child)?;
                    let words = parser.lines[child]
                        .text
                        .split_whitespace()
                        .collect::<Vec<_>>();
                    let [key @ ("major" | "minor"), value] = words.as_slice() else {
                        return Err(parser.fail(child, "version 仅支持 major/minor"));
                    };
                    if version.insert((*key).into(), json!(value)).is_some() {
                        return Err(parser.fail(child, "版本字段重复"));
                    }
                }
                if top
                    .insert("version".into(), Value::Object(version))
                    .is_some()
                {
                    return Err(parser.fail(id, "version 重复"));
                }
            }
            ["include", path] => {
                parser.leaf(id)?;
                includes.push((*path).into());
            }
            _ => domains.push(parser.domain(id)?),
        }
    }
    top.insert("domains".into(), Value::Array(domains));
    Ok((Value::Object(top), includes))
}
