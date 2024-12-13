mod bindings;
use bindings::Windows::UI::Xaml as xaml;

fn main() {
    xaml::Application::new().unwrap();
}
