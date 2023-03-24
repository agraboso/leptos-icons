#[cfg(feature = "FaBrandsSuse")]
use leptos::*;
#[cfg(feature = "FaBrandsSuse")]
///This icon requires the feature `FaBrandsSuse` to be enabled.
#[component]
pub fn Suse(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M471.08 102.66s-.3 18.3-.3 20.3c-9.1-3-74.4-24.1-135.7-26.3-51.9-1.8-122.8-4.3-223 57.3-19.4 12.4-73.9 46.1-99.6 109.7C7 277-.12 307 7 335.06a111 111 0 0 0 16.5 35.7c17.4 25 46.6 41.6 78.1 44.4 44.4 3.9 78.1-16 90-53.3 8.2-25.8 0-63.6-31.5-82.9-25.6-15.7-53.3-12.1-69.2-1.6-13.9 9.2-21.8 23.5-21.6 39.2.3 27.8 24.3 42.6 41.5 42.6a49 49 0 0 0 15.8-2.7c6.5-1.8 13.3-6.5 13.3-14.9 0-12.1-11.6-14.8-16.8-13.9-2.9.5-4.5 2-11.8 2.4-2-.2-12-3.1-12-14V316c.2-12.3 13.2-18 25.5-16.9 32.3 2.8 47.7 40.7 28.5 65.7-18.3 23.7-76.6 23.2-99.7-20.4-26-49.2 12.7-111.2 87-98.4 33.2 5.7 83.6 35.5 102.4 104.3h45.9c-5.7-17.6-8.9-68.3 42.7-68.3 56.7 0 63.9 39.9 79.8 68.3H460c-12.8-18.3-21.7-38.7-18.9-55.8 5.6-33.8 39.7-18.4 82.4-17.4 66.5.4 102.1-27 103.1-28 3.7-3.1 6.5-15.8 7-17.7 1.3-5.1-3.2-2.4-3.2-2.4-8.7 5.2-30.5 15.2-50.9 15.6-25.3.5-76.2-25.4-81.6-28.2-.3-.4.1 1.2-11-25.5 88.4 58.3 118.3 40.5 145.2 21.7.8-.6 4.3-2.9 3.6-5.7-13.8-48.1-22.4-62.7-34.5-69.6-37-21.6-125-34.7-129.2-35.3.1-.1-.9-.3-.9.7zm60.4 72.8a37.54 37.54 0 0 1 38.9-36.3c33.4 1.2 48.8 42.3 24.4 65.2-24.2 22.7-64.4 4.6-63.3-28.9zm38.6-25.3a26.27 26.27 0 1 0 25.4 27.2 26.19 26.19 0 0 0-25.4-27.2zm4.3 28.8c-15.4 0-15.4-15.6 0-15.6s15.4 15.64 0 15.64z"
        /> < title > { title } < / title > < / svg >
    }
}
