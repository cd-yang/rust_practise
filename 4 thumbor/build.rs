// build.rs 可以在编译 cargo 项目时，做额外的编译处理

fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
