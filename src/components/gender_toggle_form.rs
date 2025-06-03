use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Men,
    Women,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Men => write!(f, "Men"),
            Gender::Women => write!(f, "Women"),
        }
    }
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Men
    }
}

#[component]
pub fn GenderToggleForm() -> impl IntoView {
    let (selected_gender, set_selected_gender) = signal(Gender::Men);

    let toggle_gender = move |_| {
        set_selected_gender.update(|gender| {
            *gender = match *gender {
                Gender::Men => Gender::Women,
                Gender::Women => Gender::Men,
            }
        });
    };

    let submit_form = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let current_gender = selected_gender.get();
        log::info!("Form submitted with gender: {:?}", current_gender);
        // Handle form submission here
    };

    view! {
        <div class="max-w-md mx-auto p-6 bg-white rounded-lg shadow-md">
            <h2 class="text-2xl font-bold mb-6 text-center text-gray-800">"Gender Selection"</h2>

            <form on:submit=submit_form class="space-y-6">
                <div class="flex flex-col items-center space-y-4">
                    <label class="text-lg font-medium text-gray-700">"Select Gender:"</label>

                    // Toggle Button
                    <button
                        type="button"
                        on:click=toggle_gender
                        class="relative inline-flex h-12 w-32 items-center justify-center rounded-full bg-gray-200 p-1 transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                    >
                        <span class=move || {
                            format!(
                                "absolute h-10 w-14 transform rounded-full bg-blue-500 shadow-md transition-transform duration-200 ease-in-out {}",
                                if selected_gender.get() == Gender::Men {
                                    "translate-x-[-32px]"
                                } else {
                                    "translate-x-[32px]"
                                },
                            )
                        }></span>

                        <span class="relative z-10 flex w-full justify-between px-3 text-sm font-medium">
                            <span class=move || {
                                format!(
                                    "transition-colors duration-200 {}",
                                    if selected_gender.get() == Gender::Men {
                                        "text-white"
                                    } else {
                                        "text-gray-600"
                                    },
                                )
                            }>"Men"</span>
                            <span class=move || {
                                format!(
                                    "transition-colors duration-200 {}",
                                    if selected_gender.get() == Gender::Women {
                                        "text-white"
                                    } else {
                                        "text-gray-600"
                                    },
                                )
                            }>"Women"</span>
                        </span>
                    </button>

                    // Display current selection
                    <div class="text-center">
                        <p class="text-gray-600 text-sm">
                            "Selected: "
                            <span class="font-semibold text-blue-600">
                                {move || selected_gender.get().to_string()}
                            </span>
                        </p>
                    </div>
                </div>

                // Alternative: Radio buttons (commented out)

                <div class="flex justify-center">
                    <button
                        type="submit"
                        class="px-6 py-2 bg-blue-600 text-white font-medium rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors duration-200"
                    >
                        "Submit"
                    </button>
                </div>
            </form>
        </div>
    }
}
