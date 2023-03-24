#[cfg(feature = "SiSurrealdb")]
use leptos::*;
#[cfg(feature = "SiSurrealdb")]
///This icon requires the feature `SiSurrealdb` to be enabled.
#[component]
pub fn Surrealdb(
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
        "m12 6.314 5.714 3.165v-1.27L12 5.054c-.85.47-4.957 2.74-5.714 3.157.703.39 8.085 4.467 12.572 6.946v1.264L12 20.209c-1.71-.943-5.15-2.844-6.858-3.79v-3.788L12 16.42l1.144-.632-9.146-5.05v6.316L12 21.472l8-4.42v-2.526L8.57 8.21Zm-8.002.632v2.528l11.428 6.316-3.428 1.896-5.714-3.165v1.27l5.714 3.156c.85-.47 4.957-2.74 5.714-3.157-.703-.39-8.083-4.467-12.57-6.948V7.578L12 3.789c1.707.945 5.148 2.846 6.858 3.789v3.789L12 7.577l-1.144.633L20 13.263V6.946L12 2.526c-.791.438-7.416 4.1-8.002 4.42zM12 0 1.713 5.685v12.63L12 24l10.287-5.682V5.685Zm9.14 17.683L12 22.736l-9.143-5.053V6.317L12 1.264l9.143 5.053z"
        /> < title > { title } < / title > < / svg >
    }
}
