enum ObjType {
	Integer,
	Boolean,
	Null,
}

pub trait Obj {
	fn get_type(&self) -> ObjType;
	fn inspect(&self) -> String;
}

struct Integer {
	val: i64,
}

impl Obj for Integer {
	fn get_type(&self) -> ObjType {
		ObjType::Integer
	}
	fn inspect(&self) -> String {
		format!("{0}", self.val)
	}
}

struct Boolean {
	val: bool,
}

impl Obj for Boolean {
	fn get_type(&self) -> ObjType {
		ObjType::Boolean
	}
	fn inspect(&self) -> String {
		format!("{0}", self.val)
	}
}

struct Null {}

impl Obj for Null {
	fn get_type(&self) -> ObjType {
		ObjType::Null
	}
	fn inspect(&self) -> String {
		String::from("null")
	}
}
