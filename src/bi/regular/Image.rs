#[cfg(feature = "BiRegularImage")]
use leptos::*;
#[cfg(feature = "BiRegularImage")]
///This icon requires the feature `BiRegularImage` to be enabled.
#[component]
pub fn Image(
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
        "http://www.w3.org/2000/svg" cx = "7.499" cy = "9.5" r = "1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "m10.499 14-1.5-2-3 4h12l-4.5-6z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M19.999 4h-16c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2zm-16 14V6h16l.002 12H3.999z"
        /> < title > { title } < / title > < / svg >
    }
}
