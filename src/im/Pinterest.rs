#[cfg(feature = "ImPinterest")]
use leptos::*;
#[cfg(feature = "ImPinterest")]
///This icon requires the feature `ImPinterest` to be enabled.
#[component]
pub fn Pinterest(
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
        "M8 1.069c-3.828 0-6.931 3.103-6.931 6.931 0 2.938 1.828 5.444 4.406 6.453-0.059-0.547-0.116-1.391 0.025-1.988 0.125-0.541 0.813-3.444 0.813-3.444s-0.206-0.416-0.206-1.028c0-0.963 0.559-1.684 1.253-1.684 0.591 0 0.878 0.444 0.878 0.975 0 0.594-0.378 1.484-0.575 2.306-0.166 0.691 0.344 1.253 1.025 1.253 1.231 0 2.178-1.3 2.178-3.175 0-1.659-1.194-2.819-2.894-2.819-1.972 0-3.128 1.478-3.128 3.009 0 0.597 0.228 1.234 0.516 1.581 0.056 0.069 0.066 0.128 0.047 0.2-0.053 0.219-0.169 0.691-0.194 0.787-0.031 0.128-0.1 0.153-0.231 0.094-0.866-0.403-1.406-1.669-1.406-2.684 0-2.188 1.587-4.194 4.578-4.194 2.403 0 4.272 1.712 4.272 4.003 0 2.388-1.506 4.313-3.597 4.313-0.703 0-1.362-0.366-1.588-0.797 0 0-0.347 1.322-0.431 1.647-0.156 0.603-0.578 1.356-0.862 1.816 0.65 0.2 1.337 0.309 2.053 0.309 3.828 0 6.931-3.103 6.931-6.931 0-3.831-3.103-6.934-6.931-6.934z"
        /> < title > { title } < / title > < / svg >
    }
}
