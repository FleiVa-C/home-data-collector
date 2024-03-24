use super::model::SignalMetaQuery;

#[test]
fn build_query_name_uuid() {
    let query = SignalMetaQuery{
        name: Some("test_measurement".to_owned()),
        uuid: Some("test_uuid".to_owned()),
        interface_uuid: None
    };
    let sql = query.build_sql_query();
    assert_eq!(sql, format!("SELECT * FROM signal:`test_uuid` WHERE string::lowercase(name) contains 'test_measurement'"))
}

#[test]
fn build_query_empty() {
    let query = SignalMetaQuery{
        name: None,
        uuid: None,
        interface_uuid: None
    };
    let sql = query.build_sql_query();
    assert_eq!(sql, format!("SELECT * FROM signal"))
}
