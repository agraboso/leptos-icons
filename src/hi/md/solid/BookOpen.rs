#[cfg(feature = "HiMdSolidBookOpen")]
use leptos::*;
#[cfg(feature = "HiMdSolidBookOpen")]
///This icon requires the feature `HiMdSolidBookOpen` to be enabled.
#[component]
pub fn BookOpen(
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
        "M10.75 16.8195C11.9579 15.9871 13.4212 15.5 15 15.5C15.7103 15.5 16.3964 15.5985 17.0459 15.7822C17.272 15.8462 17.515 15.8005 17.7024 15.6587C17.8899 15.5169 18 15.2955 18 15.0605V4.06055C18 3.72495 17.7771 3.4302 17.4541 3.33886C16.6731 3.11796 15.8497 3 15 3C13.4636 3 12.016 3.38549 10.75 4.06487V16.8195Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.25 4.06487C7.98396 3.38549 6.5364 3 5 3C4.15029 3 3.32689 3.11796 2.54588 3.33886C2.22295 3.4302 2 3.72495 2 4.06055V15.0605C2 15.2955 2.11014 15.5169 2.29756 15.6587C2.48497 15.8005 2.728 15.8462 2.95412 15.7822C3.60361 15.5985 4.28967 15.5 5 15.5C6.57884 15.5 8.04208 15.9871 9.25 16.8195V4.06487Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
