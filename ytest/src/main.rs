use zhscript2_no_std::{*, };

fn main() {
	/*

定义显、名以下代码
	显示‘名’<‘‘名’’>换行。
上代码。
赋予1以。（显1。）显示<‘1’>换行。
赋予以甲。（显。）显示‘’换行。

分叉（。）
算术。
显示。
赋予。
定义。
解释。
解释【啊】下代码
	显示‘参数0’ “ - ‘参数’-”换行
上代码、1、22、333、4444。

加载_arg.zs。

显示
循环【第】则
	循环【‘第’】【次】
		分叉‘次’则
			‘第’分叉‘第’则
				循环【7】【4】【序】则‘序’、了则ESC[0;36m了。
				11、12 则ESC[1;33;41m了。
				1“h”。
				2、10“e”。
				3、4、7“l”。
				5、11“o”。
				6“,”。
				8“i”。
				9“t”。
				12“s”。
				大于等于7则ESC[m了。
				13 则
					\换行。
					跳出第。
				了
			了。
			\。
		了。
	换行
了。

循环【2】则
	1、2。。。。。。3、4
了。
则
	1、2。。。。。。3、4
了。
1、2。。。。。。3、4
*/
	let mut dbg:world_::Debug_ = Default::default();
	//dbg.args = true;
	pr__!("{:?}\n", world_::hello4__(
		Text_::new3(
r#"
定义啊以下代码
	定义哦以下代码
		显示“命令是 echo ”‘参数【2】【3】’换行。
	上代码。
	哦1、2、3"、4。
	显示‘参数0’‘参数’换行。
	哦‘参数栈【3】’。
上代码。
啊1。
啊1、2、3、4、5、6、7。
"#), false, vec![
		/*Text_::new3("1"),
		Text_::new3("22"),
		Text_::new3("333"),
		Text_::new3("4444"),*/
	], &
	world_::Path_
	//Path1_
	{}, None, None, &dbg));

}

struct Path1_ {}
impl world_::PathImpl_ for Path1_ {
	fn open__<'a, 'b>(&self, _src:&'b str, _src2:&'a str) -> Option<String> {
		Some(String::from("显示‘参数0’--‘参数’换行"))
	}
}

struct Qu_ {
	up_:Option<T_<Qu_>>,
}
impl Qu_ {
	fn up__(sel:T_<Qu_>) -> Option<T_<Qu_>> {l_r__!(sel).up_.clone()}
}
/*impl For_ for Qu_ {
	fn up__<Qu_>(sel:T_<Qu_>) -> Option<T_<Qu_>> {l_r__!(sel).up_}
}*/

pub trait For_ {
	fn for__<Q, T>(qu:T_<Q>, f:impl Fn(&Q) -> Option<T>) -> Option<T> {
		let mut qu = Some(qu);
		while qu.is_some() {
			let sel = qu.unwrap();
			let ret = f(&l_r__!(sel));
			if ret.is_some() {
				return ret
			}
			qu = Self::up__(sel).clone();
		}
		None
	}
	fn up__<Q>(sel:T_<Q>) -> Option<T_<Q>>;
}
/*pub trait For_: Sized {
	type Item;
	fn for__<T>(&self, f:impl Fn(&Item) -> Option<T>) -> Option<T> {
		let mut qu = Some(self);
		while qu.is_some() {
			let sel = qu.unwrap();
			let ret = f(&l_r__!(sel));
			if ret.is_some() {
				return ret
			}
			qu = Self::up__().clone();
		}
		None
	}
	fn up__() -> Option<Self>;
}*/

	/*impl For_ for Qu_ {
		fn up__<Qu_>(sel:T_<Qu_>) -> Option<T_<Qu_>> {l_r__!(sel).up_}
	}
	impl For_ for T_<Qu_> {
		type Item = Qu_;
		fn up__(&self) -> Option<Self> {l_r__!(self).up_}
	}*/
