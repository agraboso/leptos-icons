#[cfg(feature = "RiLogosFillPlaystation")]
use leptos::*;
#[cfg(feature = "RiLogosFillPlaystation")]
///This icon requires the feature `RiLogosFillPlaystation` to be enabled.
#[component]
pub fn Playstation(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M22.584 17.011c-.43.543-1.482.93-1.482.93l-7.833 2.817V18.68l5.764-2.057c.655-.234.755-.566.223-.74-.53-.175-1.491-.125-2.146.111l-3.84 1.354v-2.155l.22-.075s1.11-.394 2.671-.567c1.56-.172 3.472.024 4.972.593 1.69.535 1.88 1.323 1.451 1.866zm-8.57-3.537V8.162c0-.624-.114-1.198-.699-1.36-.447-.144-.725.272-.725.895V21l-3.584-1.139V4c1.524.283 3.744.953 4.937 1.355 3.035 1.043 4.064 2.342 4.064 5.267 0 2.851-1.758 3.932-3.992 2.852zm-11.583 4.99c-1.735-.49-2.024-1.51-1.233-2.097.731-.542 1.974-.95 1.974-.95l5.138-1.83v2.086l-3.697 1.325c-.653.234-.754.566-.223.74.531.175 1.493.125 2.147-.11l1.773-.644v1.865l-.353.06c-1.774.29-3.664.169-5.526-.445z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
