use zhscript2_no_std::world_;

fn main() {
	world_::hello__(r"
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
		了。");
}
