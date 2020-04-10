use json_errors::JsonErrors;

pub type CheckoutResult<T> = Result<T, JsonErrors>;
