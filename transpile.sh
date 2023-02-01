rm lib/src/*.rs
rm -rf lib/binding_rust/core_wrapper/core

c2rust transpile --emit-modules compile_commands.json

mv lib/src/lib.rs lib/src/api_raw.rs

sed -i "s/use ::libc;/use std::os;/g" lib/src/*.rs

sed -i "s/ as bool/ != 0/g" lib/src/*.rs

sed -i "s/type_0/type_/g" lib/src/*.rs

cargo run -p process_c2rust

mkdir lib/binding_rust/core_wrapper/core
mv lib/src/*.rs lib/binding_rust/core_wrapper/core/

output_path=lib/binding_rust/core_wrapper/core/defines.rs
header_path='lib/include/tree_sitter/api.h'

defines=(
  TREE_SITTER_LANGUAGE_VERSION
  TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION
)

for define in ${defines[@]}; do
  define_value=$(egrep "#define $define (.*)" $header_path | cut -d' ' -f3)
  echo "pub const $define: usize = $define_value;" >> $output_path
done

cargo fmt
