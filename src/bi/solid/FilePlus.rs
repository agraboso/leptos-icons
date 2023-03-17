#[cfg(feature = "BiSolidFilePlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFilePlus")]
/// *This icon requires the feature* `BiSolidFilePlus` *to be enabled*.
#[component]
pub fn FilePlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 22h12a2 2 0 0 0 2-2V8l-6-6H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2zm7-18 5 5h-5V4zM8 14h3v-3h2v3h3v2h-3v3h-2v-3H8v-2z" /></svg>
   }
}