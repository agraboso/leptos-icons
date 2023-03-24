#[cfg(feature = "BiRegularDice3")]
use leptos::*;
#[cfg(feature = "BiRegularDice3")]
///This icon requires the feature `BiRegularDice3` to be enabled.
#[component]
pub fn Dice3(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19 3H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zM5 19V5h14l.002 14H5z"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "8" cy = "8" r = "1.5" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "12" r = "1.5" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "16" cy = "16" r = "1.5" /> <
        title > { title } < / title > < / svg >
    }
}
