//!  编译 `toml` 的模块
//!
//! 该模块用来解析 `toml` 文件，可以对 `toml` 文件进行更改，当更改完成之后可以再序列化为 `toml` 字符串。

use std::{fs, path::Path, process};

use toml_edit::{Document, Table, TomlError};

#[derive(Debug)]
pub struct Toml {
    /// 文档
    pub doc: Document,
}

impl Toml {
    /// 解析 `toml` 字符串
    pub fn parse(input: &str) -> Result<Toml, TomlError> {
        match input.parse::<Document>() {
            Ok(doc) => Ok(Toml { doc }),
            Err(e) => Err(e),
        }
    }

    /// 转换为不可变表
    pub fn table(&self) -> &Table {
        self.doc.as_table()
    }

    /// 转换为可变表
    pub fn table_mut(&mut self) -> &mut Table {
        self.doc.as_table_mut()
    }

    /// 转换为字符串
    pub fn to_string(&self) -> String {
        self.doc.to_string_in_original_order().trim().to_string()
    }

    /// 写入到文件中
    pub fn write<P: AsRef<Path>>(&self, path: P) {
        if let Err(e) = fs::write(path, self.to_string()) {
            eprint!("写入文件失败:\n  {}", e);
            process::exit(-1);
        }
    }
}