use zhscript2_no_std::world_;

fn main() {
	world_::hello__("显示则算术1+2了换行");
	world_::hello__(r"
		赋予式1、式2以1加22减3乘4除5、(5模3)方2。
		显示
		‘式1’得则算术‘式1’了换行
		‘式2’得则算术‘式2’了换行");
}
