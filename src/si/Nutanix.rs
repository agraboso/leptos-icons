#[cfg(feature = "SiNutanix")]
use leptos::*;
#[cfg(feature = "SiNutanix")]
///This icon requires the feature `SiNutanix` to be enabled.
#[component]
pub fn Nutanix(
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
        "M.394 3.617a.395.395 0 0 0-.393.395c0 .12.054.225.14.297l8.506 7.404a.39.39 0 0 1-.013.588l-8.52 7.412a.393.393 0 0 0 .28.67h4.86a.39.39 0 0 0 .265-.104l9.17-7.98a.396.396 0 0 0 0-.596L5.52 3.721a.386.386 0 0 0-.264-.104zm18.358 0a.389.389 0 0 0-.273.113l-4.716 4.106a.392.392 0 0 0-.04.564l2.427 2.114a.393.393 0 0 0 .291.13.394.394 0 0 0 .278-.119l7.127-6.203a.389.389 0 0 0 .154-.31.395.395 0 0 0-.393-.395zm-2.31 9.742c-.116 0-.22.05-.292.13l-2.426 2.113a.392.392 0 0 0 .039.564l4.716 4.104c.07.07.166.113.273.113h4.855a.393.393 0 0 0 .239-.705l-7.127-6.203a.393.393 0 0 0-.278-.116z"
        /> < title > { title } < / title > < / svg >
    }
}
