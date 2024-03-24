use super::model::UserQuery;

#[test]
fn build_user_query_name_firstname() {
    let query_params = UserQuery {
        name: Some("Muster".to_owned()),
        firstname: Some("Max".to_owned()),
        email: None,
        uuid: None,
        is_admin: Some(true)
    };
    let sql = query_params.build_sql_query();
    assert_eq!(sql, format!("SELECT * FROM user WHERE string::lowercase(name) contains 'muster' AND string::lowercase(firstname) contains 'max' AND is_admin = true"));
}

#[test]
fn build_user_query_email_uuid() {
    let query_params = UserQuery {
        name: None,
        firstname: None,
        email: Some("test_email@email.com".to_owned()),
        uuid: Some("test_uuid".to_owned()),
        is_admin: Some(false)
    };
    let sql = query_params.build_sql_query();
    assert_eq!(sql, format!("SELECT * FROM user:`test_uuid` WHERE string::lowercase(email) contains 'test_email@email.com' AND is_admin = false"));
}

#[test]
fn build_user_query_empty() {
    let query_params = UserQuery {
        name: None,
        firstname: None,
        email: None,
        uuid: None,
        is_admin: None
    };
    let sql = query_params.build_sql_query();
    assert_eq!(sql, format!("SELECT * FROM user"));
}
