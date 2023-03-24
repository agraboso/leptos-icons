#[cfg(feature = "HiLgSolidSquare3Stack3d")]
use leptos::*;
#[cfg(feature = "HiLgSolidSquare3Stack3d")]
///This icon requires the feature `HiLgSolidSquare3Stack3d` to be enabled.
#[component]
pub fn Square3Stack3d(
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
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M11.6444 1.58965C11.8664 1.47012 12.1336 1.47012 12.3556 1.58965L22.1056 6.83965C22.3485 6.97046 22.5 7.22409 22.5 7.5C22.5 7.77591 22.3485 8.02954 22.1056 8.16035L12.3556 13.4104C12.1336 13.5299 11.8664 13.5299 11.6444 13.4104L1.89443 8.16035C1.65149 8.02954 1.5 7.77591 1.5 7.5C1.5 7.22409 1.65149 6.97046 1.89443 6.83965L11.6444 1.58965Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.26468 10.6018L10.9333 14.731C11.5992 15.0896 12.4008 15.0896 13.0667 14.7311L20.7353 10.6018L22.1056 11.3396C22.3485 11.4704 22.5 11.7241 22.5 12C22.5 12.2759 22.3485 12.5295 22.1056 12.6603L12.3556 17.9103C12.1336 18.0299 11.8664 18.0299 11.6444 17.9103L1.89443 12.6603C1.65149 12.5295 1.5 12.2759 1.5 12C1.5 11.7241 1.65149 11.4704 1.89443 11.3396L3.26468 10.6018Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.9333 19.231L3.26468 15.1018L1.89443 15.8396C1.65149 15.9704 1.5 16.2241 1.5 16.5C1.5 16.7759 1.65149 17.0295 1.89443 17.1603L11.6444 22.4103C11.8664 22.5299 12.1336 22.5299 12.3556 22.4103L22.1056 17.1603C22.3485 17.0295 22.5 16.7759 22.5 16.5C22.5 16.2241 22.3485 15.9704 22.1056 15.8396L20.7353 15.1018L13.0667 19.2311C12.4008 19.5896 11.5992 19.5896 10.9333 19.231Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
