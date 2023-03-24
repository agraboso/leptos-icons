#[cfg(feature = "BiSolidPlanet")]
use leptos::*;
#[cfg(feature = "BiSolidPlanet")]
///This icon requires the feature `BiSolidPlanet` to be enabled.
#[component]
pub fn Planet(
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
        "M15.165 15.582c4.587-4.073 8.141-9.424 6.057-11.771-.661-.744-1.584-1.089-2.575-.983-.832.091-1.687.502-2.549 1.192a8.922 8.922 0 0 0-8.712.282 8.917 8.917 0 0 0-4.109 5.515 8.892 8.892 0 0 0 .306 5.325c-1.065 1.203-2.054 3.677-.823 5.063.517.581 1.257.841 2.147.841 2.707 0 6.808-2.399 10.258-5.464zm3.699-10.767c.358-.034.632.064.861.323.231.261.169.946-.252 1.929a9.059 9.059 0 0 0-1.617-1.853c.431-.262.776-.373 1.008-.399zM4.633 17.118a8.979 8.979 0 0 0 1.568 1.737c-1.025.303-1.714.283-1.945.021-.217-.243.002-1.069.377-1.758zm16.31-5.869c-1.215 1.797-2.906 3.671-4.778 5.333-1.934 1.719-4.066 3.208-6.05 4.202a9.082 9.082 0 0 0 1.874.212 8.986 8.986 0 0 0 4.616-1.282 8.915 8.915 0 0 0 4.338-8.465z"
        /> < title > { title } < / title > < / svg >
    }
}
