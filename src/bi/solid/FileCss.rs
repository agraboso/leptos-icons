#[cfg(feature = "BiSolidFileCss")]
use leptos::*;
#[cfg(feature = "BiSolidFileCss")]
///This icon requires the feature `BiSolidFileCss` to be enabled.
#[component]
pub fn FileCss(
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
        "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6zM9.446 17.412c.275 0 .581-.061.762-.132l.138.713c-.168.084-.546.174-1.037.174-1.397 0-2.117-.869-2.117-2.021C7.191 14.768 8.175 14 9.398 14c.474 0 .833.096.995.18l-.186.726a1.98 1.98 0 0 0-.768-.149c-.726 0-1.29.438-1.29 1.337.001.808.482 1.318 1.297 1.318zm2.491.755c-.461 0-.917-.119-1.145-.245l.186-.756c.246.126.624.252 1.014.252.42 0 .642-.174.642-.438 0-.252-.192-.396-.678-.57-.672-.234-1.109-.605-1.109-1.193 0-.689.575-1.217 1.529-1.217.455 0 .791.096 1.031.203l-.204.738a1.919 1.919 0 0 0-.846-.192c-.396 0-.587.181-.587.39 0 .258.228.372.749.57.714.264 1.05.636 1.05 1.205-.001.678-.523 1.253-1.632 1.253zm3.24 0c-.461 0-.917-.119-1.145-.245l.186-.756c.246.126.624.252 1.014.252.42 0 .642-.174.642-.438 0-.252-.192-.396-.678-.57-.672-.234-1.109-.605-1.109-1.193 0-.689.575-1.217 1.529-1.217.455 0 .791.096 1.031.203l-.204.738a1.919 1.919 0 0 0-.846-.192c-.396 0-.587.181-.587.39 0 .258.228.372.749.57.714.264 1.05.636 1.05 1.205 0 .678-.523 1.253-1.632 1.253zM14 9h-1V4l5 5h-4z"
        /> < title > { title } < / title > < / svg >
    }
}
