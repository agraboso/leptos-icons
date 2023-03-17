#[cfg(feature = "BiRegularFork")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFork")]
/// *This icon requires the feature* `BiRegularFork` *to be enabled*.
#[component]
pub fn Fork(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m14.47 13.77-1.41-1.42 5.66-5.65-1.42-1.42-5.65 5.66-1.42-1.41 5.66-5.66-1.42-1.42-6.36 6.37a3 3 0 0 0 0 4.24l.71.71-6.37 6.36 1.42 1.42 6.36-6.37.71.71a3 3 0 0 0 4.24 0l6.37-6.36-1.42-1.42z" /></svg>
   }
}