#[cfg(feature = "SiTutanota")]
use leptos::*;
#[cfg(feature = "SiTutanota")]
///This icon requires the feature `SiTutanota` to be enabled.
#[component]
pub fn Tutanota(
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
        "M2.158.934C.978.934.025 1.895.023 3.08.017 9.74.005 16.413 0 23.066c.793-.297 1.67-.56 2.56-.918 6.188-2.485 11.249-4.598 11.253-6.983a1.66 1.66 0 0 0-.016-.23c-.32-2.356-5.916-3.087-5.908-4.166a.37.37 0 0 1 .05-.177c.673-1.184 3.336-1.128 4.316-1.212.982-.085 3.285-.067 3.397-.773a.44.44 0 0 0 .005-.065c.003-.656-1.584-.913-1.584-.913s1.925.29 1.92 1.042a.445.445 0 0 1-.015.114c-.207.81-1.901.962-3.021 1.017-1.06.054-2.673.175-2.679.695 0 .03.005.062.015.095.253.76 6.167 1.127 9.95 3.102 2.178 1.136 3.26 3.004 3.757 4.974V3.08A2.14 2.14 0 0 0 21.866.934H2.158Z"
        /> < title > { title } < / title > < / svg >
    }
}
