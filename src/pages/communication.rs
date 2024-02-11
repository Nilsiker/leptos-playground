use leptos::*;

#[component]
pub fn CommunicationPage() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    provide_context(set_toggled);
    view! {
        <details>
            <summary>Communication</summary>
            <hr/>
            <p>"Toggled: " {toggled}</p>
            <p>"By passing a signal " <ButtonA setter=set_toggled/></p>
            <p>
                "By using a callback "
                <ButtonB on:click=move |_| {
                    set_toggled.update(|value| *value = !*value)
                }/>
            </p>
            <p>"By providing a context " <ButtonC/></p>
            <hr/>
        </details>
    }
}

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button on:click=move |_| {
            setter.update(|value| *value = !*value)
        }>"Toggle"</button>
    }
}

#[component]
pub fn ButtonB() -> impl IntoView {
    view! { <button>"Toggle"</button> }
}

#[component]
pub fn ButtonC() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("context exists");

    view! {
        <button on:click=move |_| {
            setter.update(|value| *value = !*value)
        }>"Toggle"</button>
    }
}
