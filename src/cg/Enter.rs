#[cfg(feature = "CgEnter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgEnter")]
/// *This icon requires the feature* `CgEnter` *to be enabled*.
#[component]
pub fn Enter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M20 5H8V9H6V3H22V21H6V15H8V19H20V5Z" fill="currentColor" /><path d="M13.0743 16.9498L11.6601 15.5356L14.1957 13H2V11H14.1956L11.6601 8.46451L13.0743 7.05029L18.024 12L13.0743 16.9498Z" fill="currentColor" /></svg>
   }
}