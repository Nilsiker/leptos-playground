use leptos::*;

#[component]
pub fn ShowPage() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    println!("hehe");
    view! {
        <details>
            <summary>Show</summary>
            <button on:click=move |_| set_value.update(|value| *value += 1)>+</button>
            <Show when=move || { value.get() > 5 } fallback=|| view! { <p>Small</p> }>
                <p>BIG</p>
            </Show>
        </details>
    }
}
