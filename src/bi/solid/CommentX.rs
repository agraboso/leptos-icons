#[cfg(feature = "BiSolidCommentX")]
use leptos::*;
#[cfg(feature = "BiSolidCommentX")]
///This icon requires the feature `BiSolidCommentX` to be enabled.
#[component]
pub fn CommentX(
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
        "M20 2H4c-1.103 0-2 .897-2 2v18l4-4h14c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm-3.294 11.543-1.414 1.414-3.293-3.292-3.292 3.292-1.414-1.414 3.292-3.292-3.292-3.293 1.414-1.414 3.292 3.292 3.293-3.292 1.414 1.414-3.292 3.293 3.292 3.292z"
        /> < title > { title } < / title > < / svg >
    }
}
