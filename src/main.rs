use leptos::prelude::*;
stylance::import_crate_style!(
    #[allow(dead_code)]
    style,
    "src/styles/bulma.module.css"
);

fn main() {
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
