#[cfg(feature = "SiIcq")]
use leptos::*;
#[cfg(feature = "SiIcq")]
///This icon requires the feature `SiIcq` to be enabled.
#[component]
pub fn Icq(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M10.189 0a3.784 3.784 0 0 0-3.055 5.93l1.741 2.562a1.855 1.855 0 0 0 3.334-.13l1.454-2.929h-.006A3.784 3.784 0 0 0 10.189 0zm9.173 4.496a4.235 4.235 0 0 0-1.662.306 4.23 4.23 0 0 0-1.817 1.396l-2.214 2.837c-.025.032-.05.063-.074.096l-.01.012a2.088 2.088 0 0 0 1.509 3.306l3.614.536-.003-.007a4.259 4.259 0 0 0 4.532-5.807 4.264 4.264 0 0 0-3.875-2.675zM3.586 7.242A3.154 3.154 0 0 0 .55 9.628a3.151 3.151 0 0 0 3.903 3.804l2.539-.737a1.545 1.545 0 0 0 .742-2.673L5.79 8.118v.005a3.13 3.13 0 0 0-2.204-.881zM13.9 13.959a1.886 1.886 0 0 0-1.858 2.233l.387 3.263.005-.003a3.846 3.846 0 0 0 6.134 2.574 3.846 3.846 0 0 0-.9-6.645l-2.877-1.197a1.89 1.89 0 0 0-.89-.225zm-5.55.08c-.377 0-.75.104-1.076.3L4.06 16.018l.006.003a4.21 4.21 0 0 0-1.593 1.485 4.24 4.24 0 0 0 1.342 5.843 4.239 4.239 0 0 0 5.845-1.332 4.21 4.21 0 0 0 .647-2.172l.108-3.45a2.079 2.079 0 0 0-2.062-2.356z"
        /> < title > { title } < / title > < / svg >
    }
}
