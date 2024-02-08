use leptos::{html::Input, *};
use web_sys::SubmitEvent;

#[component]
pub fn UncontrolledForm() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element
            .get()
            .expect("existing <input> element")
            .value();
        set_name.set(value);
    };

    view! {
        <details>
            <summary>Uncontrolled Form</summary>
            <form on:submit=on_submit>
                <input type="text" value=name node_ref=input_element/>
                <input type="submit" value="Submit"/>
            </form>
            <p>{name}</p>
        </details>
    }
}
