use crate::errors::ApplicationException;

pub fn assert_forbidden_error(err: ApplicationException) {
    if let ApplicationException::ForBiddenException(_) = err {
    } else {
        panic!("should be forbidden error but was {:?}", err);
    }
}

pub fn assert_duplication_error(err: ApplicationException, valid_key: &str) {
    if let ApplicationException::DuplicationException { key, .. } = err {
        assert_eq!(key, valid_key);
    } else {
        assert!(false);
    }
}

pub fn assert_validation_error(error: ApplicationException) {
    if let ApplicationException::ValidationException { .. } = error {
    } else {
        assert!(false);
    }
}

pub fn assert_validation_error_with_key(error: ApplicationException, valid_key: &str) {
    if let ApplicationException::ValidationException { key, .. } = error {
        assert_eq!(valid_key, key);
    } else {
        assert!(false, "expected ValidationException");
    }
}
