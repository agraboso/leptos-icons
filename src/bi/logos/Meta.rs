#[cfg(feature = "BiLogosMeta")]
use leptos::*;
#[cfg(feature = "BiLogosMeta")]
///This icon requires the feature `BiLogosMeta` to be enabled.
#[component]
pub fn Meta(
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
        "M20.26 7.8a4.82 4.82 0 0 0-3.93-2.44 3.91 3.91 0 0 0-2.54 1.09 10.58 10.58 0 0 0-1.52 1.71 11 11 0 0 0-1.63-1.72 4.39 4.39 0 0 0-2.88-1.08A5 5 0 0 0 3.7 8 11.49 11.49 0 0 0 2 14a7 7 0 0 0 .18 1.64A4.44 4.44 0 0 0 2.71 17a3.23 3.23 0 0 0 3 1.6c1.25 0 2.19-.56 3.3-2a26.4 26.4 0 0 0 2.21-3.6l.63-1.12c.06-.09.11-.18.16-.27l.15.25 1.79 3A14.77 14.77 0 0 0 16 17.63a3.38 3.38 0 0 0 2.55 1 3 3 0 0 0 2.54-1.23 2.2 2.2 0 0 0 .18-.28 4.1 4.1 0 0 0 .31-.63l.12-.35A6.53 6.53 0 0 0 22 15a9 9 0 0 0 0-1 11.15 11.15 0 0 0-1.74-6.2zm-10.12 3.56c-.64 1-1.57 2.51-2.37 3.61-1 1.37-1.51 1.51-2.07 1.51a1.29 1.29 0 0 1-1.15-.66 3.3 3.3 0 0 1-.39-1.7A9.74 9.74 0 0 1 5.54 9a2.8 2.8 0 0 1 2.19-1.47A3 3 0 0 1 10 8.74a14.07 14.07 0 0 1 1 1.31zm8.42 5.12c-.48 0-.85-.19-1.38-.83A34.87 34.87 0 0 1 14.82 12l-.52-.86c-.36-.61-.71-1.16-1-1.65a2.46 2.46 0 0 1 .17-.27c.94-1.39 1.77-2.17 2.8-2.17a3.12 3.12 0 0 1 2.49 1.66 10.17 10.17 0 0 1 1.37 5.34c-.04 1.31-.34 2.43-1.57 2.43z"
        /> < title > { title } < / title > < / svg >
    }
}
