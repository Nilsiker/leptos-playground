use leptos::*;

use crate::progress_bar::ProgressBar;

#[component]
pub fn DerivedSignalExample() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let increment = move |_| set_count.update(|count| *count += 1);
    let double_count = move || count.get() * 2;
    view! {
        <details>
            <summary>Derived Signals</summary>
            <p>
                <button on:click=increment class:red=move || count.get() % 2 == 1>
                    "Click me "
                    {count}
                </button>
            </p>
            <ProgressBar progress=count/>
            <ProgressBar progress=double_count/>
        </details>
    }
}
