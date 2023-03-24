#[cfg(feature = "SiAdobelightroomclassic")]
use leptos::*;
#[cfg(feature = "SiAdobelightroomclassic")]
///This icon requires the feature `SiAdobelightroomclassic` to be enabled.
#[component]
pub fn Adobelightroomclassic(
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
        "M19.75.3H4.25C1.9.3 0 2.2 0 4.55v14.9c0 2.35 1.9 4.25 4.25 4.25h15.5c2.35 0 4.25-1.9 4.25-4.25V4.55C24 2.2 22.1.3 19.75.3zM8.57 16.689c-.01.061-.03.101-.06.121-.03.02-.06.029-.09.029H2.71c-.1 0-.14-.061-.14-.18V6.44c0-.1.04-.14.13-.14h1.77c.07 0 .1.04.1.11v8.4h4.16c.09 0 .12.049.1.14l-.26 1.739zm5.6-5.919c0 .08-.05.11-.141.11-.319-.02-.639 0-.949.07-.26.06-.51.15-.75.27-.18.09-.35.22-.49.37v5.1c0 .101-.04.141-.12.141H9.98c-.1 0-.14-.051-.14-.16v-5.54c0-.24 0-.49-.01-.75 0-.26-.01-.52-.02-.78-.01-.221-.03-.441-.06-.661 0-.03 0-.06.02-.09.03-.01.05-.02.08-.01h1.58c.09 0 .15.05.19.16.03.07.06.15.07.23.02.1.03.21.04.31.01.11.01.23.01.36.26-.34.59-.64.96-.86.399-.24.87-.37 1.34-.36.09 0 .13.05.13.14v1.95zm7.2-1.61c.01.06-.021.11-.06.15-.041.02-.09.02-.131 0-.229-.12-.47-.2-.72-.24-.31-.06-.63-.08-.94-.08-.51-.01-1.02.12-1.459.38-.41.25-.73.62-.94 1.05-.229.5-.341 1.05-.33 1.6-.011.4.05.791.16 1.169.1.311.25.601.44.86.17.229.379.431.629.58.24.14.49.25.76.32.25.069.521.11.781.11.289 0 .58-.011.869-.041.24-.029.48-.09.7-.17.08-.06.13-.029.16-.01.04.04.06.1.05.15v1.49c.01.119-.05.22-.15.27-.26.1-.529.17-.81.2-.339.052-.679.072-1.029.072-.49 0-.99-.069-1.459-.199-.461-.12-.891-.33-1.271-.6-.38-.271-.71-.601-.979-.99-.291-.42-.5-.881-.641-1.371-.15-.58-.23-1.17-.221-1.759 0-.98.191-1.86.58-2.6.381-.73.951-1.34 1.66-1.75.711-.41 1.57-.62 2.551-.62.34 0 .68.02 1.02.06.23.03.46.08.67.17.08.05.12.14.11.24V9.16z"
        /> < title > { title } < / title > < / svg >
    }
}
