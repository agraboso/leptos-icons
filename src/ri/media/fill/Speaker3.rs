#[cfg(feature = "RiMediaFillSpeaker3")]
use leptos::*;
#[cfg(feature = "RiMediaFillSpeaker3")]
///This icon requires the feature `RiMediaFillSpeaker3` to be enabled.
#[component]
pub fn Speaker3(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M4 3h16a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm8 13a4 4 0 1 1 0-8 4 4 0 0 1 0 8zm0 2a6 6 0 1 0 0-12 6 6 0 0 0 0 12zM6 7a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm12 0a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm0 12a1 1 0 1 0 0-2 1 1 0 0 0 0 2zM6 19a1 1 0 1 0 0-2 1 1 0 0 0 0 2zm6-5.5a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
