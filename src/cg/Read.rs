#[cfg(feature = "CgRead")]
use leptos::*;
#[cfg(feature = "CgRead")]
///This icon requires the feature `CgRead` to be enabled.
#[component]
pub fn Read(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M7 14C8.10457 14 9 13.1046 9 12C9 10.8954 8.10457 10 7 10C5.89543 10 5 10.8954 5 12C5 13.1046 5.89543 14 7 14ZM10.4649 10C9.77325 8.8044 8.48056 8 7 8C5.13616 8 3.57006 9.27477 3.12602 11H2C1.44772 11 1 11.4477 1 12C1 12.5523 1.44772 13 2 13H3.12602C3.57006 14.7252 5.13616 16 7 16C9.20914 16 11 14.2091 11 12H13C13 14.2091 14.7909 16 17 16C18.8638 16 20.4299 14.7252 20.874 13H22C22.5523 13 23 12.5523 23 12C23 11.4477 22.5523 11 22 11H20.874C20.4299 9.27477 18.8638 8 17 8C15.5194 8 14.2267 8.8044 13.5351 10H10.4649ZM15 12C15 13.1046 15.8954 14 17 14C18.1046 14 19 13.1046 19 12C19 10.8954 18.1046 10 17 10C15.8954 10 15 10.8954 15 12Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
