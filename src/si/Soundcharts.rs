#[cfg(feature = "SiSoundcharts")]
use leptos::*;
#[cfg(feature = "SiSoundcharts")]
///This icon requires the feature `SiSoundcharts` to be enabled.
#[component]
pub fn Soundcharts(
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
        "M16.038 0h6.052v17.229h-.702v-4.333h-.413v4.333h-.29v-2.521h-.412v2.52h-.29v-4.04h-.413v4.04h-.289V13.98h-.414v3.25h-.33v-2.521h-.371v2.52h-.331v-3.791h-.413v3.792h-.289v-4.333h-.414v4.333h-.701c.02-5.73.02-11.457.02-17.229ZM8.995 7.624h6.01v9.605h-.66v-2.23h-.414v2.23h-.288v-3.5h-.414v3.5h-.289v-3.5h-.413v3.5h-.33v-2.77h-.372v2.77h-.331v-2.77h-.413v2.77h-.289V14.23h-.414v3h-.289v-3.793h-.412v3.793h-.703c.02-2.687.02-6.958.02-9.605zM7.28 12.896h-.414v4.29h-.288v-1.978h-.414v1.98h-.33v-2.772h-.372v2.77h-.33v-4.04h-.414v4.04H4.43v-2.228h-.414v2.229h-.289v-3.25h-.414v3.25h-.289v-3.75h-.412v3.75H1.91V11.54h6.011v5.646h-.66c.02-1.395.02-2.833.02-4.291zm5.969 5.5h.371V24h-.371Zm4.213 0h.33v5.52h-.33zm-7.436 5.396h-.33v-5.398h.33v5.397zm7.064-5.397v5.397h-.33v-5.398h.33zm-4.544 0h.33v5.23h-.33zm6.342 0h.33v5.23h-.33zm-9.894 0h.33v4.897h-.33v-4.898ZM19.92 23.25h-.33v-4.856h.33v4.855zm-9.17-.042h-.33v-4.833h.33zm4.254-.042h-.33v-4.79h.33v4.791zm-3.511-.374h-.372v-4.417h.372zm9.852.208h-.33v-4.625h.33zm-9.191-.208h-.33v-4.417h.33zm3.883-4.397h.331v4.417h-.33Zm-9.852 4.063v-4.082h.371v4.083h-.371Zm11.98-.042v-4.04h.371v4.041h-.371zM4.8 18.396h.33v4.25H4.8v-4.252Zm16.916 0h.372v4h-.372v-4.002ZM6.91 22.415v-4.04h.33v4.041h-.33Zm-1.426-.042v-3.998h.33v4h-.33Zm15.162 0h-.331v-3.998h.33v4zm-16.96-.04h-.33v-3.917h.33zm-.701-.23h-.372v-3.709h.372Zm10.967-3.709h.372v3.625h-.372zm-5.97 3.793h-.33v-3.793h.33zm-6.009-3.794h.33v3.584h-.33zm2.128 0h.33v3.584H4.1z"
        /> < title > { title } < / title > < / svg >
    }
}
