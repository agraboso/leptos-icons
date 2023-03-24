#[cfg(feature = "SiDgraph")]
use leptos::*;
#[cfg(feature = "SiDgraph")]
///This icon requires the feature `SiDgraph` to be enabled.
#[component]
pub fn Dgraph(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M18.22 4.319c.226-.414.349-.892.349-1.403A2.917 2.917 0 0015.653 0c-1.37 0-2.522.944-2.838 2.218-.272-.013-.544-.033-.815-.033-5.58 0-10.1 4.513-10.1 10.1 0 2.74 1.1 5.23 2.871 7.047a2.916 2.916 0 00-.588 1.752A2.917 2.917 0 007.1 24c1.241 0 2.295-.782 2.728-1.869a10.092 10.092 0 0012.272-9.86 9.982 9.982 0 00-3.88-7.952zm-2.554.381c-.162 0-.304-.013-.446-.064l-1.21 3.523 1.772-.284-2.489 4.067 2.075-.511-7.002 8.34c.35.317.556.783.556 1.307a1.78 1.78 0 01-1.784 1.784c-.99 0-1.785-.795-1.785-1.784s.796-1.785 1.785-1.785c.226 0 .446.045.653.13l1.978-4.326-1.933.524 3.142-4.5-1.933.465L14.521 4.3c-.4-.337-.64-.828-.64-1.371 0-.99.796-1.785 1.785-1.785s1.784.796 1.784 1.785c.007.97-.795 1.771-1.784 1.771z"
        /> < title > { title } < / title > < / svg >
    }
}
