use puniyu_error::Result;

#[test]
fn test_result_ok_unit() {
	fn operation() -> Result {
		Ok(())
	}

	let result = operation();
	assert!(result.is_ok());
}

#[test]
fn test_result_ok_value() {
	fn get_value() -> Result<i32> {
		Ok(42)
	}

	let result = get_value();
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_result_err() {
	fn fail() -> Result {
		Err("error".into())
	}

	let result = fail();
	assert!(result.is_err());
}

#[test]
fn test_result_with_string() {
	fn get_string() -> Result<String> {
		Ok("hello".to_string())
	}

	let result = get_string();
	assert_eq!(result.unwrap(), "hello");
}

#[test]
fn test_result_with_vec() {
	fn get_vec() -> Result<Vec<i32>> {
		Ok(vec![1, 2, 3])
	}

	let result = get_vec();
	assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_result_propagation() {
	fn inner() -> Result<i32> {
		Ok(42)
	}

	fn outer() -> Result<i32> {
		let value = inner()?;
		Ok(value * 2)
	}

	let result = outer();
	assert_eq!(result.unwrap(), 84);
}

#[test]
fn test_result_error_propagation() {
	fn inner() -> Result {
		Err("inner error".into())
	}

	fn outer() -> Result {
		inner()?;
		Ok(())
	}

	let result = outer();
	assert!(result.is_err());
}

#[test]
fn test_result_map() {
	fn get_value() -> Result<i32> {
		Ok(10)
	}

	let result = get_value().map(|v| v * 2);
	assert_eq!(result.unwrap(), 20);
}

#[test]
fn test_result_and_then() {
	fn get_value() -> Result<i32> {
		Ok(10)
	}

	fn double(v: i32) -> Result<i32> {
		Ok(v * 2)
	}

	let result = get_value().and_then(double);
	assert_eq!(result.unwrap(), 20);
}

#[test]
fn test_result_or_else() {
	fn fail() -> Result<i32> {
		Err("error".into())
	}

	fn fallback(_: Box<dyn std::error::Error + Send + Sync>) -> Result<i32> {
		Ok(0)
	}

	let result = fail().or_else(fallback);
	assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_result_unwrap_or() {
	fn fail() -> Result<i32> {
		Err("error".into())
	}

	let value = fail().unwrap_or(42);
	assert_eq!(value, 42);
}

#[test]
fn test_result_unwrap_or_else() {
	fn fail() -> Result<i32> {
		Err("error".into())
	}

	let value = fail().unwrap_or(42);
	assert_eq!(value, 42);
}
