use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod components;
pub mod models;
mod pages;
pub mod scoring_logic;

// Top-Level pages
use crate::pages::home::Home;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" attr:class="h-full" />

        // sets the document title
        <Title text="World Athletics Points Calculator" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        // <Body class="h-full bg-white text-gray-900 antialiased" />

        <Router>
            <div class="min-h-screen flex flex-col">
                <header class="bg-gray-900 text-white py-4 shadow-md">
                    <div class="container mx-auto px-4">
                        <h1 class="text-2xl font-bold">World Athletics Points Calculator</h1>
                    </div>
                </header>

                <main class="flex-grow">
                    <Routes fallback=|| view! { NotFound }>
                        <Route path=path!("/") view=Home />
                    </Routes>
                </main>

                <footer class="bg-gray-100 py-4 border-t border-gray-200">
                    <div class="container mx-auto px-4 text-center text-gray-600">
                        <p>2025 World Athletics Points Calculator</p>
                    </div>
                </footer>
            </div>
        </Router>
    }
}
