#[cfg(feature = "IoExit")]
use leptos::*;
#[cfg(feature = "IoExit")]
///This icon requires the feature `IoExit` to be enabled.
#[component]
pub fn Exit(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M336,376V272H191a16,16,0,0,1,0-32H336V136a56.06,56.06,0,0,0-56-56H88a56.06,56.06,0,0,0-56,56V376a56.06,56.06,0,0,0,56,56H280A56.06,56.06,0,0,0,336,376Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M425.37,272l-52.68,52.69a16,16,0,0,0,22.62,22.62l80-80a16,16,0,0,0,0-22.62l-80-80a16,16,0,0,0-22.62,22.62L425.37,240H336v32Z"
        /> < title > { title } < / title > < / svg >
    }
}
