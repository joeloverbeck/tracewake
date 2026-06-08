use crate::load::SourceFile;

pub fn prose_born_fact_rejected_001() -> SourceFile {
    SourceFile {
        path: "prose_born_fact_rejected_001.twf".to_string(),
        bytes: b"fixture|prose_born_fact_rejected_001\nschema|schema_v1\nactor|actor_mara|workshop\nplace|workshop|576f726b73686f70|\nnotes|Mara is the true culprit and the final event must happen".to_vec(),
    }
}
