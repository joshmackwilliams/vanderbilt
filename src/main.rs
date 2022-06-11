use vanderbilt::ui::cli::CLI;
use vanderbilt::ui::UI;

fn main() {
    let ui: Box<dyn UI> = Box::new(CLI::new());
    ui.display_message("Hello world!");
}
