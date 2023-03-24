#[cfg(feature = "ImGrin2")]
use leptos::*;
#[cfg(feature = "ImGrin2")]
///This icon requires the feature `ImGrin2` to be enabled.
#[component]
pub fn Grin2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8c4.418 0 8-3.582 8-8s-3.582-8-8-8zM11 3.688c0.999 0 1.813 0.813 1.813 1.813 0 0.1-0.009 0.201-0.025 0.302-0.025 0.151-0.156 0.261-0.308 0.261s-0.283-0.11-0.308-0.261c-0.096-0.573-0.589-0.833-1.171-0.833s-1.074 0.26-1.171 0.833c-0.025 0.151-0.156 0.261-0.308 0.261-0 0 0 0-0 0-0.153 0-0.283-0.11-0.308-0.261-0.017-0.101-0.025-0.202-0.025-0.302 0-0.999 0.813-1.813 1.813-1.813zM5 3.688c0.999 0 1.813 0.813 1.813 1.813 0 0.1-0.009 0.201-0.025 0.302-0.025 0.151-0.156 0.261-0.308 0.261s-0.283-0.11-0.308-0.261c-0.096-0.573-0.589-0.833-1.171-0.833s-1.074 0.26-1.171 0.833c-0.025 0.151-0.156 0.261-0.308 0.261 0 0 0 0 0 0-0.153 0-0.283-0.11-0.308-0.261-0.017-0.101-0.025-0.202-0.025-0.302 0-0.999 0.813-1.813 1.813-1.813zM3 9h3v3.873c-1.72-0.447-3-2.018-3-3.873zM7 13v-4h2v4h-2zM10 12.873v-3.873h3c0 1.855-1.28 3.426-3 3.873z"
        /> < title > { title } < / title > < / svg >
    }
}
