use leptos::*;

#[component]
pub fn Select() -> impl IntoView {
    let (value, set_value) = create_signal("B".to_string());

    view! {
        <details>
            <summary>Select</summary>
            <div>
                <select on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_value.set(new_value);
                }>
                    <SelectOption value is="A"/>
                    <SelectOption value is="B"/>
                    <SelectOption value is="C"/>
                </select>
            </div>
        </details>
    }
}

#[component]
fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option value=is selected=move || value.get() == is>
            {is}
        </option>
    }
}
