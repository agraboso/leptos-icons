#[cfg(feature = "BiRegularAlarmSnooze")]
use leptos::*;
#[cfg(feature = "BiRegularAlarmSnooze")]
///This icon requires the feature `BiRegularAlarmSnooze` to be enabled.
#[component]
pub fn AlarmSnooze(
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
        "M12 4c-4.879 0-9 4.121-9 9s4.121 9 9 9 9-4.121 9-9-4.121-9-9-9zm0 16c-3.794 0-7-3.206-7-7s3.206-7 7-7 7 3.206 7 7-3.206 7-7 7zm8.292-13.292-3.01-3 1.412-1.417 3.01 3zM6.698 3.707l-2.99 2.999L2.29 5.294l2.99-3z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.832 10.555A1 1 0 0 0 14 9H9v2h3.132l-2.964 4.445A1 1 0 0 0 10 17h5v-2h-3.132l2.964-4.445z"
        /> < title > { title } < / title > < / svg >
    }
}
