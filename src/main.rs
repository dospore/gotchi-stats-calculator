mod app;
mod gotchi;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
