#[cfg(feature = "BiRegularMaleFemale")]
use leptos::*;
#[cfg(feature = "BiRegularMaleFemale")]
///This icon requires the feature `BiRegularMaleFemale` to be enabled.
#[component]
pub fn MaleFemale(
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
        width = { size.clone() } height = { size } > < circle xmlns =
        "http://www.w3.org/2000/svg" cx = "6" cy = "4" r = "2" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9 7H3a1 1 0 0 0-1 1v7h2v7h4v-7h2V8a1 1 0 0 0-1-1z" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "17" cy = "4" r = "2" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20.21 7.73a1 1 0 0 0-1-.73h-4.5a1 1 0 0 0-1 .73L12 14h2l-1 4h2v4h4v-4h2l-1-4h2z"
        /> < title > { title } < / title > < / svg >
    }
}
