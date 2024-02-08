use leptos::*;

#[component]
pub fn ControlledForm() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <details>
            <summary>Controlled Form</summary>
            <input
                type="text"
                on:input=move |ev| {
                    set_name.set(event_target_value(&ev));
                }

                prop:value=name
            />
            <p>"Name is: " {name}</p>
        </details>
    }
}
