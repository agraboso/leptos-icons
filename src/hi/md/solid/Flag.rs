#[cfg(feature = "HiMdSolidFlag")]
use leptos::*;
#[cfg(feature = "HiMdSolidFlag")]
///This icon requires the feature `HiMdSolidFlag` to be enabled.
#[component]
pub fn Flag(
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
        "M3.5 2.75C3.5 2.33579 3.16421 2 2.75 2C2.33579 2 2 2.33579 2 2.75V17.25C2 17.6642 2.33579 18 2.75 18C3.16421 18 3.5 17.6642 3.5 17.25V12.8585L5.15742 12.5096C6.60458 12.2049 8.11248 12.407 9.42842 13.0818C11.2723 14.0274 13.4125 14.2155 15.393 13.6061L17.4706 12.9668C17.7853 12.87 18 12.5793 18 12.25V3.75C18 3.52398 17.8981 3.31001 17.7226 3.1676C17.547 3.02519 17.3167 2.96952 17.0955 3.01609L14.7157 3.51709C13.3151 3.81195 11.8585 3.68564 10.5296 3.15406L10.0275 2.95322C8.42359 2.31166 6.66566 2.15921 4.97524 2.51509L3.5 2.82566V2.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
