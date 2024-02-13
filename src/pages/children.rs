use leptos::*;

#[component]
pub fn ChildrenPage() -> impl IntoView {
    view! {
        <details>
            <summary>"Children"</summary>
            <TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
                <fieldset>
                    <label for="">"Some Input" <input type="öu"/></label>
                </fieldset>
                <button>"Submit"</button>
            </TakesChildren>
            <WrapsChildren>"A" "B" "C"</WrapsChildren>
        </details>
    }
}

#[component]
fn TakesChildren<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h2>"Render Prop"</h2>
        {render_prop()}

        <h2>"Children"</h2>
        {children()}
    }
}

#[component]
fn WrapsChildren(children: Children) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect_view();
    view! { <ul>{children}</ul> }
}
