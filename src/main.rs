// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
#![allow(clippy::duplicate_mod)]
use dioxus::prelude::*;

use views::{Home, Navbar};

/// Define a components module that contains all shared components for our app.
mod components;
mod models;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;
#[cfg(feature = "server")]
mod server;

pub use models::ContactEntry;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Navbar)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Home {},
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[cfg(feature = "server")]
static NATS_MANAGER: std::sync::OnceLock<std::sync::Mutex<Option<server::nats::NatsManager>>> =
    std::sync::OnceLock::new();

#[cfg(feature = "server")]
async fn ensure_nats_initialized() {
    if NATS_MANAGER.get().is_some() {
        return;
    }

    let manager = server::nats::NatsManager::start()
        .await
        .expect("Failed to start NATS server");

    server::kv::init_kv_bucket()
        .await
        .expect("Failed to initialize NATS KV bucket");

    let _ = NATS_MANAGER.set(std::sync::Mutex::new(Some(manager)));
}

#[cfg(feature = "server")]
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }
}

#[cfg(feature = "server")]
fn main() {
    serve(|| async {
        ensure_nats_initialized().await;

        tokio::spawn(async {
            shutdown_signal().await;
            eprintln!("[server] Shutdown signal received, cleaning up...");
            let manager = NATS_MANAGER
                .get()
                .and_then(|mutex| mutex.lock().ok())
                .and_then(|mut guard| guard.take());
            if let Some(manager) = manager {
                manager.shutdown().await;
            }
            std::process::exit(0);
        });

        Ok(dioxus::server::router(App))
    });
}

#[cfg(not(feature = "server"))]
fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}
