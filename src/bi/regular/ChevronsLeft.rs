#[cfg(feature = "BiRegularChevronsLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularChevronsLeft")]
/// *This icon requires the feature* `BiRegularChevronsLeft` *to be enabled*.
#[component]
pub fn ChevronsLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m12.707 7.707-1.414-1.414L5.586 12l5.707 5.707 1.414-1.414L8.414 12z" /><path d="M16.293 6.293 10.586 12l5.707 5.707 1.414-1.414L13.414 12l4.293-4.293z" /></svg>
   }
}