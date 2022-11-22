use std::io::Result;

fn main() -> Result<()> {
	// compile our proto files
	prost_build::compile_protos(&["src/proto/store.proto", "--experimental_allow_proto3_optional"], &["src/"])?;
	Ok(())
}