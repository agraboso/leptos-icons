#[cfg(feature = "FaSolidHouseFloodWaterCircleArrowRight")]
use leptos::*;
#[cfg(feature = "FaSolidHouseFloodWaterCircleArrowRight")]
///This icon requires the feature `FaSolidHouseFloodWaterCircleArrowRight` to be enabled.
#[component]
pub fn HouseFloodWaterCircleArrowRight(
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
        "M288 144A144 144 0 1 0 0 144a144 144 0 1 0 288 0zM140.7 76.7c6.2-6.2 16.4-6.2 22.6 0l56 56c6.2 6.2 6.2 16.4 0 22.6l-56 56c-6.2 6.2-16.4 6.2-22.6 0s-6.2-16.4 0-22.6L169.4 160H80c-8.8 0-16-7.2-16-16s7.2-16 16-16h89.4L140.7 99.3c-6.2-6.2-6.2-16.4 0-22.6zM320 144c0 57.3-27.4 108.2-69.8 140.3c11.8-3.6 23-9.4 33-16.2c22.1-15.5 51.6-15.5 73.7 0c18.4 12.7 39.6 20.3 59.2 20.3c19 0 41.2-7.9 59.2-20.3c23.8-16.7 55.8-15.4 78.1 3.4c2.1 1.7 4.2 3.3 6.5 4.9l-.3-84.4H576c13.9 0 26.1-8.9 30.4-22.1s-.4-27.6-11.6-35.8l-176-128C407.6-2 392.4-2 381.2 6.1L301 64.4c12.1 23.9 19 50.9 19 79.6zm18.5 165.9c-11.1-7.9-25.9-7.9-37 0C279 325.4 251.5 336 224 336c-26.9 0-55.3-10.8-77.4-26.1l0 0c-11.9-8.5-28.1-7.8-39.2 1.7c-14.4 11.9-32.5 21-50.6 25.2c-17.2 4-27.9 21.2-23.9 38.4s21.2 27.9 38.4 23.9c24.5-5.7 44.9-16.5 58.2-25C158.5 389.7 191 400 224 400c31.9 0 60.6-9.9 80.4-18.9c5.8-2.7 11.1-5.3 15.6-7.7c4.5 2.4 9.7 5.1 15.6 7.7c19.8 9 48.6 18.9 80.4 18.9c33 0 65.5-10.3 94.5-25.8c13.4 8.4 33.7 19.3 58.2 25c17.2 4 34.4-6.7 38.4-23.9s-6.7-34.4-23.9-38.4c-18.1-4.2-36.2-13.3-50.6-25.2c-11.1-9.5-27.3-10.1-39.2-1.7l0 0C471.4 325.2 442.9 336 416 336c-27.5 0-55-10.6-77.5-26.1zm0 112c-11.1-7.9-25.9-7.9-37 0C279 437.4 251.5 448 224 448c-26.9 0-55.3-10.8-77.4-26.1l0 0c-11.9-8.5-28.1-7.8-39.2 1.7c-14.4 11.9-32.5 21-50.6 25.2c-17.2 4-27.9 21.2-23.9 38.4s21.2 27.9 38.4 23.9c24.5-5.7 44.9-16.5 58.2-25C158.5 501.7 191 512 224 512c31.9 0 60.6-9.9 80.4-18.9c5.8-2.7 11.1-5.3 15.6-7.7c4.5 2.4 9.7 5.1 15.6 7.7c19.8 9 48.6 18.9 80.4 18.9c33 0 65.5-10.3 94.5-25.8c13.4 8.4 33.7 19.3 58.2 25c17.2 4 34.4-6.7 38.4-23.9s-6.7-34.4-23.9-38.4c-18.1-4.2-36.2-13.3-50.6-25.2c-11.1-9.4-27.3-10.1-39.2-1.7l0 0C471.4 437.2 442.9 448 416 448c-27.5 0-55-10.6-77.5-26.1z"
        /> < title > { title } < / title > < / svg >
    }
}
