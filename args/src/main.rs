use zhscript2_no_std::world_;

fn main() {
	world_::hello__(r#"
		定义哦以下代码
			显示“命令是 echo ”‘参数【2】【3】’换行。
		上代码。
		哦1、2、3"、4。
		定义啊以下代码
			显示‘参数0’‘参数’换行。
			哦‘参数栈【3】’。
		上代码。
		啊1、2、3、4、5、6、7。
	"#);
	world_::hello__(r#"
		显示换行循环【29】则-了换行换行。
		定义啊以下代码
			定义哦以下代码
				显示“命令是 echo ”‘参数【2】【3】’换行。
			上代码。
			哦1、2、3"、4。
			显示‘参数0’‘参数’换行。
			哦‘参数栈【3】’。
		上代码。
		啊1、2、3、4、5、6、7。
	"#);
}
