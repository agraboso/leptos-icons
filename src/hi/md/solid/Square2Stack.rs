#[cfg(feature = "HiMdSolidSquare2Stack")]
use leptos::*;
#[cfg(feature = "HiMdSolidSquare2Stack")]
///This icon requires the feature `HiMdSolidSquare2Stack` to be enabled.
#[component]
pub fn Square2Stack(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M2 4.25C2 3.00736 3.00736 2 4.25 2H10.75C11.9926 2 13 3.00736 13 4.25V5.5H9.25C7.17893 5.5 5.5 7.17893 5.5 9.25V13H4.25C3.00736 13 2 11.9926 2 10.75V4.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.25 7C8.00736 7 7 8.00736 7 9.25V15.75C7 16.9926 8.00736 18 9.25 18H15.75C16.9926 18 18 16.9926 18 15.75V9.25C18 8.00736 16.9926 7 15.75 7H9.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
