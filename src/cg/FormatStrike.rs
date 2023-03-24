#[cfg(feature = "CgFormatStrike")]
use leptos::*;
#[cfg(feature = "CgFormatStrike")]
///This icon requires the feature `CgFormatStrike` to be enabled.
#[component]
pub fn FormatStrike(
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
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 7H17V5H7V7H11V10H13V7Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 19V14H13V19H11Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 13H19V11H5V13Z" fill = "currentColor" /> <
        title > { title } < / title > < / svg >
    }
}
