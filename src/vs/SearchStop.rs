#[cfg(feature = "VsSearchStop")]
use leptos::*;
#[cfg(feature = "VsSearchStop")]
///This icon requires the feature `VsSearchStop` to be enabled.
#[component]
pub fn SearchStop(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M5.738 3.318a4.5 4.5 0 0 0-.877 5.123A4.48 4.48 0 0 0 6.1 10a4.62 4.62 0 0 0-.1 1v.17c-.16-.11-.32-.22-.47-.34L1.75 14.5 1 13.84l3.8-3.69a5.5 5.5 0 1 1 9.62-3.65c0 .268-.02.535-.06.8a5.232 5.232 0 0 0-.94-.68V6.5a4.5 4.5 0 0 0-7.682-3.182zm3.04 4.356a4 4 0 1 1 4.444 6.652 4 4 0 0 1-4.444-6.652zm.1 5.447A3 3 0 0 0 11 14a3 3 0 0 0 1.74-.55L8.55 9.26A3 3 0 0 0 8 11a3 3 0 0 0 .879 2.121zm.382-4.57l4.19 4.189A3 3 0 0 0 14 11a3 3 0 0 0-3-3 3 3 0 0 0-1.74.55z"
        /> < title > { title } < / title > < / svg >
    }
}
