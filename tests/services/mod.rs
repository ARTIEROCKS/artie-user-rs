use user_service::services::security_service::validate_password;

#[test]
fn test_validate_password() {
    let original_password = "1234";
    let stored_password = "5ee6256c4e0040d6850bfeb02ef985084ea50b867d31018e357d70f6031e8c3ad409dcf8de1f5dcbc215cffe540548937822a6f69cea508ba0fe40149e424444";
    let stored_salt = "f6d4989453eb2601635368a422727f01";
    assert_eq!(validate_password(original_password, stored_password, stored_salt), true);
}
