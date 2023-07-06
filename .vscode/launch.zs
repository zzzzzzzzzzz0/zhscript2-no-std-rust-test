#!/usr/lib/zhscript2-rust/l --。

加载lib/file4。
赋予主名、目录/以
	先调用‘得文件主名’、‘参数0’了、
	先调用‘得目录名’、‘参数0’了。

赋予出以。
加载lib/doscmd4。
赋予搜以调用‘dir.begin’、‘目录/’..、、a-f ad o0。
循环先
	赋予名以调用‘dir.next’、‘搜’。
	如果不‘名’那么跳出。
	分叉‘名’先
		target再来
	了。
	显示‘名’换行。
	赋予--bin、kind以分叉‘名’先
		l4先下文本"test",
                    "--no-run",上文本、cdylib了。
		先下文本"build",
                    "--bin=‘名’",上文本、bin了
	了。
	赋予出以‘出’先如果‘出’那么,了下文本
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug '‘名’'",
            "cargo": {
                "args": [
                    ‘--bin’
                    "--package=‘名’"
                ],
                "filter": {
                    "name": "‘名’",
                    "kind": "‘kind’"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }上文本（下文本,
        {
            "type": "lldb",
            "request": "launch",
            "name": "test '‘名’'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=‘名’",
                    "--package=‘名’"
                ],
                "filter": {
                    "name": "‘名’",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }上文本）。
了。
调用‘dir.end’、‘搜’。

赋予临时文件以/tmp/‘主名’.txt。
调用‘echo’、下文本{
    // 使用 IntelliSense 了解相关属性。 
    // 悬停以查看现有属性的描述。
    // 欲了解更多信息，请访问: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [‘出’
    ]
}
上文本、‘临时文件’。
执行管道接显示“meld ‘临时文件’ ‘目录/’‘主名’.json”换行。
