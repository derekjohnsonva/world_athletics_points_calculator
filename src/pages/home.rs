use crate::components::world_athletics_score_form::WorldAthleticsScoreForm;
use leptos::prelude::*;
use leptos_meta::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <div class="min-h-screen bg-white flex flex-col items-center justify-center p-4">
                    <h1 class="text-3xl font-bold text-gray-900 mb-4">
                        "Uh oh! Something went wrong!"
                    </h1>

                    <p class="text-lg text-gray-700 mb-2">"Errors: "</p>
                    // Render a list of errors as strings - good for development purposes
                    <ul class="list-disc pl-5 text-gray-700">
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li class="mb-1">{e.to_string()}</li> })
                                .collect_view()
                        }}
                    </ul>
                </div>
            }
        }>
            <Title text="World Athletics Points Calculator" />
            <main class="min-h-screen bg-white flex flex-col items-center justify-center p-4">
                <div class="w-full max-w-2xl bg-white rounded-lg shadow-sm p-6 border border-gray-200">
                    <WorldAthleticsScoreForm />
                </div>
            </main>
        </ErrorBoundary>
    }
}
