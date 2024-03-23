use super::model::InterfaceQuery;


#[test]
fn surreal_build_interface_query() {
   let query_params = InterfaceQuery{
       uuid: None,
       url: None,
       interface_type: Some("ShellyV1".to_owned()),
       name: Some("Name".to_owned())
   };
    let sql = query_params.build_sql_query();
    assert_eq!(sql, format!("SELECT * FROM interface WHERE string::lowercase(interface_type) contains 'shellyv1' AND string::lowercase(name) contains 'name'"));
}
