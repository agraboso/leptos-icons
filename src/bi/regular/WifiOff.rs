#[cfg(feature = "BiRegularWifiOff")]
use leptos::*;
#[cfg(feature = "BiRegularWifiOff")]
///This icon requires the feature `BiRegularWifiOff` to be enabled.
#[component]
pub fn WifiOff(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "m1.293 8.395 1.414 1.414c.504-.504 1.052-.95 1.622-1.359L2.9 7.021c-.56.422-1.104.87-1.607 1.374zM6.474 5.06 3.707 2.293 2.293 3.707l18 18 1.414-1.414-5.012-5.012.976-.975a7.86 7.86 0 0 0-4.099-2.148L11.294 9.88c2.789-.191 5.649.748 7.729 2.827l1.414-1.414c-2.898-2.899-7.061-3.936-10.888-3.158L8.024 6.61A13.366 13.366 0 0 1 12 6c3.537 0 6.837 1.353 9.293 3.809l1.414-1.414C19.874 5.561 16.071 4 12 4a15.198 15.198 0 0 0-5.526 1.06zm-2.911 6.233 1.414 1.414a9.563 9.563 0 0 1 2.058-1.551L5.576 9.697c-.717.451-1.395.979-2.013 1.596zm2.766 3.014 1.414 1.414c.692-.692 1.535-1.151 2.429-1.428l-1.557-1.557a7.76 7.76 0 0 0-2.286 1.571zm7.66 3.803-2.1-2.1a1.996 1.996 0 1 0 2.1 2.1z"
        /> < title > { title } < / title > < / svg >
    }
}
