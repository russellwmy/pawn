use pawn_square::square::run_wasm_module;

fn main() -> anyhow::Result<()> {
    run_wasm_module("module.wasm")
}
