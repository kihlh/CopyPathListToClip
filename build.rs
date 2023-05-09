extern crate winres;

fn main() {
    // // 以下代码告诉 Cargo ，一旦指定的文件 `src/hello.c` 发生了改变，就重新运行当前的构建脚本
    // println!("cargo:rerun-if-changed=src/hello.c");
    // // 使用 `cc` 来构建一个 C 文件，然后进行静态链接
    // cc::Build::new()
    //     .file("src/hello.c")
    //     .compile("hello");
    let mut res = winres::WindowsResource::new();
    res.set_manifest(r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
            </requestedPrivileges>
        </security>
    </trustInfo>
    </assembly>
    "#);
}
