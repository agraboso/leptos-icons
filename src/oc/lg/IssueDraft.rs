#[cfg(feature = "OcLgIssueDraft")]
use leptos::*;
#[cfg(feature = "OcLgIssueDraft")]
///This icon requires the feature `OcLgIssueDraft` to be enabled.
#[component]
pub fn IssueDraft(
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
        "M17.32 3.205a.75.75 0 0 1 1.046-.177 11.056 11.056 0 0 1 2.605 2.606.75.75 0 1 1-1.222.869 9.554 9.554 0 0 0-2.252-2.252.75.75 0 0 1-.177-1.046Zm3.475 14.115a.75.75 0 0 1 .176 1.046 11.07 11.07 0 0 1-2.605 2.605.75.75 0 1 1-.869-1.222 9.554 9.554 0 0 0 2.252-2.252.75.75 0 0 1 1.046-.177ZM2.018 9.543a.75.75 0 0 1 .615.864 9.571 9.571 0 0 0 0 3.186.75.75 0 1 1-1.48.25 11.07 11.07 0 0 1 0-3.686.75.75 0 0 1 .865-.614Zm7.525 12.439a.75.75 0 0 1 .864-.615 9.571 9.571 0 0 0 3.186 0 .75.75 0 1 1 .25 1.48 11.07 11.07 0 0 1-3.686 0 .75.75 0 0 1-.614-.865ZM6.68 3.205a.75.75 0 0 1-.177 1.046A9.558 9.558 0 0 0 4.25 6.503a.75.75 0 1 1-1.223-.87 11.056 11.056 0 0 1 2.606-2.605.75.75 0 0 1 1.046.177ZM3.205 17.32a.75.75 0 0 1 1.046.177 9.554 9.554 0 0 0 2.252 2.252.75.75 0 1 1-.87 1.223 11.056 11.056 0 0 1-2.605-2.606.75.75 0 0 1 .177-1.046Zm6.952-16.166a11.07 11.07 0 0 1 3.686 0 .75.75 0 0 1-.25 1.479 9.571 9.571 0 0 0-3.186 0 .75.75 0 1 1-.25-1.48Zm11.825 8.389a.75.75 0 0 1 .864.614 11.07 11.07 0 0 1 0 3.686.75.75 0 0 1-1.479-.25 9.571 9.571 0 0 0 0-3.186.75.75 0 0 1 .615-.864Z"
        /> < title > { title } < / title > < / svg >
    }
}
