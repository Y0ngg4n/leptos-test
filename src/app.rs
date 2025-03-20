use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
                <link data-trunk rel="stylesheet" href="/styles/bulma.min.css" />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="public/styles/home4strays.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=CreatePetSitter />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn PetSitterList() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {}
}

#[server]
pub async fn savePetSitter(
    name: String,
    description: String,
    capacity: i32,
    status: String,
    duration: i32,
) -> Result<(), ServerFnError> {
    use crate::models::{NewPetSitters, PetSitters};
    use crate::schema::pet_sitters;
    use axum::Extension;
    use diesel::SqliteConnection;
    use diesel_async::pooled_connection::bb8::*;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use diesel_async::sync_connection_wrapper::*;
    use diesel_async::AsyncConnection;
    use diesel_async::RunQueryDsl;
    let Extension(mut db_connection): Extension<Pool<SyncConnectionWrapper<SqliteConnection>>> =
        leptos_axum::extract().await?;
    let mut db_connection = db_connection.get().await?;
    let petSitter = NewPetSitters {
        name,
        description,
        capacity,
        status,
        duration,
    };
    diesel::insert_into(pet_sitters::table)
        .values(&petSitter)
        .execute(&mut db_connection)
        .await
        .expect("Could not insert pet sitter");
    Ok(())
}

/// Renders the home page of your application.
#[component]
fn CreatePetSitter() -> impl IntoView {
    // Creates a reactive value to update the button
    let (name, set_name) = signal(String::new());
    let description = RwSignal::new(String::new());
    let (capacity, set_capacity) = signal(String::new());
    let (status, set_status) = signal(String::new());
    let (duration, set_duration) = signal(String::new());

    let onSavePetSitter = move |e: SubmitEvent| {
        e.prevent_default();

        spawn_local(async move {
            savePetSitter(
                name.get_untracked(),
                description.get_untracked(),
                capacity.get_untracked().parse().unwrap(),
                status.get_untracked(),
                duration.get_untracked().parse().unwrap(),
            )
            .await
            .unwrap();
        })
    };
    view! {
        <div class="container">
            <h1 class="title">Register as Pet Sitter</h1>
            <form on:submit=onSavePetSitter>
                <div class="field">
                    <label class="label">Name</label>
                    <div class="control">
                        <input
                            class="input"
                            type="text"
                            placeholder="Text input"
                            required
                            bind:value=(name, set_name)
                        />
                    </div>
                </div>

                <div class="field">
                    <label class="label">Description</label>
                    <div class="control">
                        <textarea
                            prop:value=move || description.get()
                            on:input:target=move |ev| description.set(ev.target().value())
                            rows=10
                            required
                            class="textarea"
                            placeholder="Describe yourself"
                        ></textarea>
                    </div>
                </div>

                <div class="field">
                    <label class="label">Capacity</label>
                    <div class="control">
                        <input
                            class="input"
                            type="number"
                            min="0"
                            required
                            placeholder="How many?"
                            bind:value=(capacity, set_capacity)
                        />
                    </div>
                </div>

                <div class="field">
                    <label class="label">Duration</label>
                    <div class="control">
                        <input
                            class="input"
                            type="number"
                            min="0"
                            required
                            placeholder="How many days?"
                            bind:value=(duration, set_duration)
                        />
                    </div>
                </div>
                <button class="button is-primary" type="submit">
                    Speichern
                </button>
            </form>
        </div>
    }
}
