use leptos::prelude::*;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col items-center justify-center p-4 bg-white">
            <div class="text-center p-8 max-w-md border border-gray-200 rounded-lg shadow-sm">
                <h1 class="text-4xl font-bold text-gray-900 mb-4">"404"</h1>
                <p class="text-xl mb-2 text-gray-800">"Uh oh!"</p>
                <p class="text-lg text-gray-700">"We couldn't find that page!"</p>
                <a
                    href="/"
                    class="mt-6 inline-block px-6 py-2 bg-gray-900 text-white font-medium rounded-md hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 transition-colors"
                >
                    "Return Home"
                </a>
            </div>
        </div>
    }
}
