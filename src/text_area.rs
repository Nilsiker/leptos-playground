use leptos::*;

#[component]
pub fn TextArea() -> impl IntoView {
    let (value, set_value) = create_signal("sample text".to_string());

    view! {
        <details>
            <summary>Text Area</summary>
            <textarea
                prop:value=move || value.get()
                on:input=move |ev| { set_value.set(event_target_value(&ev)) }
            >
                {untrack(move || value.get())}
            </textarea>
            <p>"Text area content: " {value}</p>
        </details>
    }
}
