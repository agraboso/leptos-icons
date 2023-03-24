#[cfg(feature = "FaSolidSunPlantWilt")]
use leptos::*;
#[cfg(feature = "FaSolidSunPlantWilt")]
///This icon requires the feature `FaSolidSunPlantWilt` to be enabled.
#[component]
pub fn SunPlantWilt(
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
        "M160 0c-6.3 0-12 3.7-14.6 9.5L120.6 64.9 63.9 43.2c-5.9-2.3-12.6-.8-17 3.6s-5.9 11.1-3.6 17l21.7 56.7L9.5 145.4C3.7 148 0 153.7 0 160s3.7 12 9.5 14.6l55.4 24.8L43.2 256.1c-2.3 5.9-.8 12.6 3.6 17s11.1 5.9 17 3.6l56.7-21.7 24.8 55.4c2.6 5.8 8.3 9.5 14.6 9.5s12-3.7 14.6-9.5l24.8-55.4 56.7 21.7c5.9 2.3 12.6 .8 17-3.6s5.9-11.1 3.6-17l-21.7-56.7 55.4-24.8c5.8-2.6 9.5-8.3 9.5-14.6s-3.7-12-9.5-14.6l-55.4-24.8 21.7-56.7c2.3-5.9 .8-12.6-3.6-17s-11.1-5.9-17-3.6L199.4 64.9 174.6 9.5C172 3.7 166.3 0 160 0zm0 96a64 64 0 1 1 0 128 64 64 0 1 1 0-128zm32 64a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm312 16c0-17.7 14.3-32 32-32s32 14.3 32 32v53.4c-14.8 7.7-24 23.1-24 44.6c0 16.8 16 44 37.4 67.2c5.8 6.2 15.5 6.2 21.2 0C624 318 640 290.7 640 274c0-21.5-9.2-37-24-44.6V176c0-44.2-35.8-80-80-80s-80 35.8-80 80v22.7c-9.8-4.3-20.6-6.7-32-6.7c-44.2 0-80 35.8-80 80v21.4c-14.8 7.7-24 23.1-24 44.6c0 16.8 16 44 37.4 67.2c5.8 6.2 15.5 6.2 21.2 0C400 382 416 354.7 416 338c0-21.5-9.2-37-24-44.6V272c0-17.7 14.3-32 32-32s32 14.3 32 32v8V448H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H608c17.7 0 32-14.3 32-32s-14.3-32-32-32H504V280v-8V176z"
        /> < title > { title } < / title > < / svg >
    }
}
