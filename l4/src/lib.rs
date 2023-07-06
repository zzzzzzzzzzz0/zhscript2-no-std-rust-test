use zhscript2_no_std::{*, world_::{PathImpl_}};
use std::{os::raw::{c_char, c_void}, ffi::{CStr, CString}, fs::File, io::Read};

type Ret__ = extern fn(*const c_char, *const c_void);
type Ret2_ = *const c_void;

fn ret__(s:&str, ret:Ret__, ret2:Ret2_) {
	let s =
		CString::new(
			s
		).unwrap()
		;
	ret(s.as_ptr(), ret2);
}

struct Path_ {
}

impl PathImpl_ for Path_ {
	fn open__<'a, 'b>(&self, src:&'b str, src2:&'a str) -> Option<String> {
		let mut src = String::from(src);
		if let Some(i) = src2.rfind('/') {
			src.insert_str(0, &src2[0..i]);
		}
		if let Ok(mut file) = File::open(&src) {
			let mut buf = String::new();
			if file.read_to_string(&mut buf).is_ok() {
				return Some(buf)
			}
		}
		None
	}
}

fn z__(s:&str, src_is_file:bool, mut arg:Vec<Text_>, ret:Ret__, ret2:Ret2_) -> i32 {
	let mut dbg:world_::Debug_ = Default::default();
	let mut i = 0;
	while i < arg.len() {
		let mut b = true;
		let s = arg[i].s_.as_str();
		match s {
			"-zhscript-d-tree" => {dbg.tree = true;}
			"-zhscript-d-src" => {dbg.src = true;}
			"-zhscript-d-arg" => {dbg.args = true;}
			"-zhscript-d-def" => {dbg.def = true;}
			_ => b = s.starts_with("-zhscript-")
		}
		if b {
			arg.remove(i);
		} else {
			i += 1;
		}
	}
	match world_::hello4__(Text_::new3(s), src_is_file, arg, &Path_ {}, None, None, &dbg) {
		world_::Return_::Ok2(v) => {
			for i in v {
				ret__(&i.s_, ret, ret2);
			}
			0
		}
		world_::Return_::Err(s) => {
			ret__(&s, ret, ret2);
			255
		}
		_ => {
			255
		}
	}
}

#[no_mangle]
pub extern "C" fn hello__(s:*const c_char, src_is_file:bool, argc: u32, argv: *const *const c_char, from: u32, ret:Ret__, ret2:Ret2_) -> i32 {
	let s = unsafe {CStr::from_ptr(s)};
	if let Ok(s) = s.to_str() {
		let mut arg = vec![];
		for i in from as isize .. argc as isize {
			let s = unsafe {CStr::from_ptr(*argv.offset(i)).to_str()};
			let s = s.unwrap();
			arg.push(Text_::new(s.to_owned()));
		}
		z__(s, src_is_file, arg, ret, ret2)
	} else {255}
}

/*extern crate va_list;
use va_list::VaList;

#[no_mangle]
pub extern "C" fn hello2__(s:*const c_char, src_is_file:bool, argc: u32, mut argv:VaList, ret:Ret__, ret2:Ret2_) -> i32 {
	let s = unsafe {CStr::from_ptr(s)};
	if let Ok(s) = s.to_str() {
		let mut arg = vec![];
		for _ in 0 .. argc {
			let s = unsafe {CStr::from_ptr(argv.get::<*const c_char>()).to_str()};
			let s = s.unwrap();
			arg.push(Text_::new(s.to_owned()));
		}
		z__(s, src_is_file, arg, ret, ret2)
	} else {255}
}*/

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
