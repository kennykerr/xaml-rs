fn main() {
    windows_bindgen::bindgen(
        "--out src/bindings.rs --filter Windows.UI.Xaml --reference windows,skip-root,Windows"
            .split_whitespace(),
    );
}
