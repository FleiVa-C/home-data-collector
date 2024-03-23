use super::model::UserQuery;

#[test]
fn surreal_build_user_query() {
    let query_params = UserQuery {
        name: Some("Muster".to_owned()),
        firstname: Some("Max".to_owned()),
        email: None,
        uuid: None,
        is_admin: Some(true),
    };
    let sql = query_params.build_sql_query();
    assert_eq!(sql, format!("SELECT * FROM user WHERE string::lowercase(name) contains 'muster' AND string::lowercase(firstname) contains 'max' AND is_admin = true"));
}
