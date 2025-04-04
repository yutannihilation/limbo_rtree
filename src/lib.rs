use limbo_ext::{VTabCursor, VTabKind, VTabModule, VTabModuleDerive};

#[derive(VTabModuleDerive, Default)]
pub struct RTreeVTab;

/// the cursor holds a snapshot of (rowid, key, value) in memory.
pub struct RTreeCursor {
    rows: Vec<(i64, String, String)>,
    index: Option<usize>,
}

impl VTabModule for RTreeVTab {
    type VCursor = RTreeCursor;

    const VTAB_KIND: VTabKind = VTabKind::VirtualTable;

    const NAME: &'static str = "rtree";

    type Error = String;

    fn create_schema(args: &[limbo_ext::Value]) -> String {
        let table_name = args[0].to_text().unwrap_or("tmp");
        format!(
            r#"-- rtree
CREATE TABLE "{table_name}_node"   (nodeno INTEGER PRIMARY KEY, data);
CREATE TABLE "{table_name}_parent" (nodeno INTEGER PRIMARY KEY, parentnode);
CREATE TABLE "{table_name}_rowid"  (rowid  INTEGER PRIMARY KEY, nodeno);
"#
        )
    }

    fn open(&self) -> Result<Self::VCursor, Self::Error> {
        todo!()
    }

    fn filter(cursor: &mut Self::VCursor, args: &[limbo_ext::Value]) -> limbo_ext::ResultCode {
        todo!()
    }

    fn column(cursor: &Self::VCursor, idx: u32) -> Result<limbo_ext::Value, Self::Error> {
        todo!()
    }

    fn next(cursor: &mut Self::VCursor) -> limbo_ext::ResultCode {
        todo!()
    }

    fn eof(cursor: &Self::VCursor) -> bool {
        todo!()
    }
}

impl VTabCursor for RTreeCursor {
    type Error = String;

    fn rowid(&self) -> i64 {
        todo!()
    }

    fn column(&self, idx: u32) -> Result<limbo_ext::Value, Self::Error> {
        todo!()
    }

    fn eof(&self) -> bool {
        todo!()
    }

    fn next(&mut self) -> limbo_ext::ResultCode {
        todo!()
    }
}
