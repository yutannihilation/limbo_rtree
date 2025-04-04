#[derive(VTabModuleDerive, Default)]
pub struct RTreeVTab;

/// the cursor holds a snapshot of (rowid, key, value) in memory.
pub struct RTreeCursor {
    rows: Vec<(i64, String, String)>,
    index: Option<usize>,
}

impl limbo_ext::VTabModule for RTreeVTab {
    type VCursor;

    const VTAB_KIND: limbo_ext::VTabKind = limbo_ext::VTabKind::VirtualTable;

    const NAME: &'static str = "rtree";

    type Error = String;

    fn create_schema(args: &[limbo_ext::Value]) -> String {
        let table_name = args[0].to_text().unwrap_or("tmp");
        format!(
            r#"-- rtree
CREATE TABLE IF NOT EXISTS "{table_name}_node"   (nodeno INTEGER PRIMARY KEY, data);
CREATE TABLE IF NOT EXISTS "{table_name}_parent" (nodeno INTEGER PRIMARY KEY, parentnode);
CREATE TABLE IF NOT EXISTS "{table_name}_rowid"  (rowid  INTEGER PRIMARY KEY, nodeno);
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
