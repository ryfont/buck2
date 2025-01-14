/*
 * Copyright 2019 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use dupe::Dupe;

use crate::codemap::CodeMap;
use crate::codemap::FileSpan;
use crate::syntax::ast::AstStmt;
use crate::syntax::ast::Stmt;
use crate::syntax::AstModule;

fn go(x: &AstStmt, codemap: &CodeMap, res: &mut Vec<FileSpan>) {
    match &**x {
        Stmt::Statements(_) => {} // These are not interesting statements that come up
        _ => res.push(FileSpan {
            span: x.span,
            file: codemap.dupe(),
        }),
    }
    x.visit_stmt(|x| go(x, codemap, res))
}

impl AstModule {
    /// Locations where statements occur.
    pub fn stmt_locations(&self) -> Vec<FileSpan> {
        let mut res = Vec::new();
        go(&self.statement, &self.codemap, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use gazebo::prelude::*;

    use crate::assert;

    #[test]
    fn test_locations() {
        fn get(code: &str) -> String {
            assert::parse_ast(code)
                .stmt_locations()
                .map(|x| x.resolve_span().to_string())
                .join(" ")
        }

        assert_eq!(&get("foo"), "1:1-4");
        assert_eq!(&get("foo\ndef x():\n   pass"), "1:1-4 2:1-3:8 3:4-8");
    }
}
