#[cfg(feature = "BiLogosNetlify")]
use leptos::*;
#[cfg(feature = "BiLogosNetlify")]
///This icon requires the feature `BiLogosNetlify` to be enabled.
#[component]
pub fn Netlify(
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
        "M17.13 9.68 22 11.8v-.11c-.07-.24-.28-.45-.7-.87l-2-2-2.07.87zM12.2 7.24a1.31 1.31 0 0 1 .16.47l3 1.29a1.11 1.11 0 0 1 .29-.14l.48-3-2-2-1.95 3.3s0 .05.02.08zM10.61 7a1.1 1.1 0 0 1 1-.2h.05l2.16-3.34-.66-.66c-.42-.42-.63-.63-.87-.7a1 1 0 0 0-.63 0c-.24.07-.45.28-.87.7l-3 3c.31.14 2.5 1.06 2.78 1.19zm-3.12 5.29h.27a1.45 1.45 0 0 1 .29 0l2.38-3.68v-.06a1 1 0 0 1-.27-.95L9 7l-1.7-.73L5 8.61l2.51 3.66zm7.62-2.44v-.08a.59.59 0 0 1 0-.14l-2.86-1.27h-.05a1.05 1.05 0 0 1-.88.47h-.17s-.05 0-.06.05L8.73 12.5v.05l6.33-2.7s.05.02.05 0zm1.36-.91a.83.83 0 0 1 .26.19l2-.84-1.89-1.87-.4 2.47zm-2.29 5.54A.91.91 0 0 1 15 14v-.05l.54-3.37-.09-.08L9 13.15a2.76 2.76 0 0 1 .08.27zM2 11.95zm12.14 3.18L8.92 14v.06L13.47 21l.42-.43c.15-.92.71-4.33.78-4.72a.9.9 0 0 1-.53-.72zm1.65.32a.88.88 0 0 1-.36.27l-.64 3.94L18.43 16zm1.07-5.13a.07.07 0 0 0-.06 0 .92.92 0 0 1-.61.34v.06l-.55 3.4a.88.88 0 0 1 .34.62h.06l2.95.61 2.28-2.28a4.22 4.22 0 0 0 .65-.74zM8.3 14.59a1.27 1.27 0 0 1-.86 0l-1.22 1.9 4.69 4.68a2.68 2.68 0 0 0 .87.71 1 1 0 0 0 .63 0 1.86 1.86 0 0 0 .7-.54l-4.69-6.8zm4.71 6.82zm-6.62-7.82s.06 0 .06-.05 0 0-.06 0l-4.14-.86a7.07 7.07 0 0 0 .51.54l1.36 1.36 2.27-1zm.28-.66a1.05 1.05 0 0 1 .2-.27v-.06C6.1 11.4 5.3 10.24 4.5 9.07l-1.75 1.75c-.42.42-.63.63-.7.87A.75.75 0 0 0 2 12l4.67 1zm.17 1.33a.61.61 0 0 1-.07-.1L4.59 15l1.08 1.1 1.17-1.82s.01-.02 0-.02z"
        /> < title > { title } < / title > < / svg >
    }
}
