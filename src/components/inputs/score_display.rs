use leptos::prelude::*;

#[component]
pub fn ScoreDisplay(
    points: ReadSignal<f64>,
    points_calculated: ReadSignal<bool>,
    parse_error: ReadSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <div class="mt-8 flex flex-col items-center">
            <button
                type="submit"
                class=move || {
                    if parse_error.get().is_some() {
                        "px-8 py-3 bg-gray-400 text-white text-lg font-medium rounded-md cursor-not-allowed transition-colors shadow-sm"
                    } else {
                        "px-8 py-3 bg-gray-900 text-white text-lg font-medium rounded-md hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 transition-colors shadow-sm"
                    }
                }
                disabled=move || parse_error.get().is_some()
            >
                "Calculate Score"
            </button>

            <Show
                when=move || points_calculated.get()
                fallback=|| {
                    view! {
                        <div class="mt-6 text-center text-gray-500 italic">
                            "Submit the form to calculate points"
                        </div>
                    }
                }
            >
                <div class="mt-6 text-center p-4 bg-gray-50 rounded-lg border border-gray-200 shadow-sm">
                    <h3 class="text-2xl font-bold text-gray-800">
                        {"Points: "}
                        <span class="text-gray-900">
                            {move || format!("{:.2}", points.get())}
                        </span>
                    </h3>
                    <p class="text-sm text-gray-600 mt-1">
                        Based on World Athletics scoring tables with adjustments for wind and elevation change. Due to how scores are calculated, you may see a discrepancy of +-1 point vs. your official World Athletics score.
                    </p>
                </div>
            </Show>
        </div>
    }
}
