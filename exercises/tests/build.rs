fn main() {
    // 设置测试7中的环境变量 TEST_FOO
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let your_command_tests7 = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", your_command_tests7);

    // 启用测试8中的 "pass" 功能
    let your_command_tests8 = "rustc-cfg=pass";
    println!("cargo:{}", your_command_tests8);
    
    // 告诉 Cargo 链接到外部库 mylib
    println!("cargo:rustc-link-lib=mylib");
}

