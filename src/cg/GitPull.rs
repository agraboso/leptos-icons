#[cfg(feature = "CgGitPull")]
use leptos::*;
#[cfg(feature = "CgGitPull")]
///This icon requires the feature `CgGitPull` to be enabled.
#[component]
pub fn GitPull(
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
        "M7 5C7 3.89543 7.89543 3 9 3C10.1046 3 11 3.89543 11 5C11 5.34168 10.9143 5.66336 10.7633 5.9447H11.3438C13.5529 5.9447 15.3438 7.73556 15.3438 9.9447V11.2244C15.9301 11.5731 16.323 12.213 16.323 12.9447C16.323 14.0493 15.4276 14.9447 14.323 14.9447C13.2184 14.9447 12.323 14.0493 12.323 12.9447C12.323 12.1959 12.7345 11.5432 13.3438 11.2004V9.9447C13.3438 8.84013 12.4483 7.9447 11.3438 7.9447H10V17.2676C10.5978 17.6134 11 18.2597 11 19C11 20.1046 10.1046 21 9 21C7.89543 21 7 20.1046 7 19C7 18.2597 7.4022 17.6134 8 17.2676V6.73244C7.4022 6.38663 7 5.74028 7 5Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
