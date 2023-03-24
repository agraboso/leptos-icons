#[cfg(feature = "BiRegularPodcast")]
use leptos::*;
#[cfg(feature = "BiRegularPodcast")]
///This icon requires the feature `BiRegularPodcast` to be enabled.
#[component]
pub fn Podcast(
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
        width = { size.clone() } height = { size } > < circle xmlns =
        "http://www.w3.org/2000/svg" cx = "12.01" cy = "12" r = "2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11.01 22h2l.5-7h-3l.5 7z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 2a10 10 0 0 0-2.45 19.68l-.15-2.12a8 8 0 1 1 5.21 0l-.15 2.12A10 10 0 0 0 12 2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.32 9.61a3.44 3.44 0 0 1 .37.68 3.83 3.83 0 0 1 .23.75 3.57 3.57 0 0 1 .09.8 3.66 3.66 0 0 1-.09.81 3.83 3.83 0 0 1-.23.75 3.44 3.44 0 0 1-.37.68 4.7 4.7 0 0 1-.35.43l-.19 2.62a5.33 5.33 0 0 0 .58-.31A5.86 5.86 0 0 0 17 15.2a5.57 5.57 0 0 0 .55-1 5.89 5.89 0 0 0 .35-1.13 6.06 6.06 0 0 0 .1-1.23 6.22 6.22 0 0 0-.13-1.21A6.09 6.09 0 0 0 17 8.49a6.29 6.29 0 0 0-.73-.89 5.67 5.67 0 0 0-.89-.73 6.3 6.3 0 0 0-1-.56A6.17 6.17 0 0 0 13.21 6a6.11 6.11 0 0 0-2.41 0 5.51 5.51 0 0 0-1.13.36 5.57 5.57 0 0 0-1 .55 5.67 5.67 0 0 0-.89.73 6.29 6.29 0 0 0-.78.85 6.09 6.09 0 0 0-.9 2.14 6.21 6.21 0 0 0-.1 1.21 6.06 6.06 0 0 0 .12 1.21 5.89 5.89 0 0 0 .35 1.13 5.57 5.57 0 0 0 .55 1 6.24 6.24 0 0 0 1.62 1.62 5.33 5.33 0 0 0 .58.31L9 14.51a4.7 4.7 0 0 1-.35-.43 3.44 3.44 0 0 1-.37-.68 3.83 3.83 0 0 1-.23-.75 3.65 3.65 0 0 1-.05-.81 3.56 3.56 0 0 1 .08-.8 3.83 3.83 0 0 1 .23-.75 3.44 3.44 0 0 1 .37-.68 4 4 0 0 1 .5-.61 3.87 3.87 0 0 1 .59-.48 3.44 3.44 0 0 1 .68-.37 3.86 3.86 0 0 1 .75-.24 4.36 4.36 0 0 1 1.61 0 3.86 3.86 0 0 1 .75.24 3.58 3.58 0 0 1 1.27.85 3.49 3.49 0 0 1 .49.61z"
        /> < title > { title } < / title > < / svg >
    }
}
