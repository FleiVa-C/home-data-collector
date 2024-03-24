use super::model::InterfaceQuery;

#[test]
fn build_interface_query_type_name() {
   let query_params = InterfaceQuery{
       uuid: None,
       url: None,
       interface_type: Some("ShellyV1".to_owned()),
       name: Some("Name".to_owned())
   };
    let sql = query_params.build_sql_query();
    assert_eq!(sql, format!("SELECT * FROM interface WHERE string::lowercase(interface_type) contains 'shellyv1' AND string::lowercase(name) contains 'name'"));
}

#[test]
fn build_interface_query_uuid_url() {
   let query_params = InterfaceQuery{
       uuid: Some("test_uuid".to_owned()),
       url: Some("test_url".to_owned()),
       interface_type: None,
       name: None
   };
    let sql = query_params.build_sql_query();
    assert_eq!(sql, format!("SELECT * FROM interface:`test_uuid` WHERE url = 'test_url'"));
}

#[test]
fn build_interface_query_empty() {
    let query_params = InterfaceQuery {
        uuid: None,
        url: None,
        interface_type: None,
        name: None
    };
    let sql = query_params.build_sql_query();
    assert_eq!(sql, "SELECT * FROM interface".to_owned());
}
