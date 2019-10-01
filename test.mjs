import init, {add_points_in_rust, new_point} from "./pkg/wasm_lib_with_c.mjs"

run();

async function run() {
  await init('./pkg/wasm_lib_with_c_bg.wasm');

  let p = add_points_in_rust(new_point(4, 5), new_point(1, 10));
  console.log(p.line, p.column);

  p = add_points_in_rust(new_point(4, 5), new_point(0, 10));
  console.log(p.line, p.column);
}
