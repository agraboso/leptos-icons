#[cfg(feature = "SiSolus")]
use leptos::*;
#[cfg(feature = "SiSolus")]
///This icon requires the feature `SiSolus` to be enabled.
#[component]
pub fn Solus(
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
        "M7.453 0c-.18.587-.369 1.167-.565 1.75A11.638 11.638 0 0 0 0 12.364a11.638 11.638 0 0 0 .516 3.403l-.339.598L0 16.73l.279.143a3.448 3.448 0 0 0 .741.222A11.638 11.638 0 0 0 2 18.868c4.034.343 8.55.512 12.446-.056 3.192-.463 5.94-1.423 7.735-3.117.252-.233.474-.474.674-.722.019-.038.037-.053.06-.076.011 0 .026-.037.038-.052.015 0 .03-.038.041-.057.008 0 .015-.038.023-.038.33-.444.587-.892.801-1.31l.181-.365-.365-.365a5.936 5.936 0 0 0-.361-.35A11.638 11.638 0 0 0 11.635.722a11.638 11.638 0 0 0-3.211.463C7.96.508 7.596.041 7.453 0zm.365 1.637C9.06 3.82 10.13 5.06 11.454 7.457c.132 1.524.67 9.45.727 10.181-.392-.037-2.485-.24-5.104-.515-1.43-.147-2.899-.316-4.092-.49l-1.9-.447c2.149-3.787 5.551-9.727 6.737-14.548zm4.543 6.18s4.991 3.927 7.092 8.73c-2.56 1.26-4.916 1.098-6.361 1.09 1.023-2.634 1.023-6.21-.73-9.82zm3.456 2.184a45.14 45.14 0 0 1 2.91.907c1.768.629 3.417 1.49 4.365 2.364a6.956 6.956 0 0 1-2.91 2.91c.151-1.495-.39-2.933-1.456-4.002-.787-.787-1.822-1.453-2.91-2.183zm6.707 6.478c-2.352 1.667-5.126 2.68-7.965 3.112a41.026 41.026 0 0 1-3.715.34h-.323a53.48 53.48 0 0 1-3.727 0 85.763 85.763 0 0 1-4.178-.23h-.003c2.555 3.255 6.993 4.893 11.092 4.102a11.367 11.367 0 0 0 4.498-1.852 11.638 11.638 0 0 0 .007 0c.312-.214.614-.444.903-.685a11.638 11.638 0 0 0 .038-.037 11.555 11.555 0 0 0 3.376-4.762zM2.511 19.584a11.638 11.638 0 0 0 .023.038c-.008 0-.015-.038-.023-.038z"
        /> < title > { title } < / title > < / svg >
    }
}
