use anyhow::{Context, Error};
use axum::Router;
use leptos::{get_configuration, IntoView};
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::services::ServeDir;

pub async fn run<IV>(app_fn: impl Fn() -> IV + Clone + Send + 'static) -> Result<(), Error>
where
    IV: IntoView + 'static,
{
    let conf = get_configuration(None)
        .await
        .context("failed to initialize configuration for Leptos")?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let assets = ServeDir::new(leptos_options.site_root.clone());
    let routes = generate_route_list(app_fn.clone());

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, app_fn)
        .fallback_service(assets)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context(format!("failed to bind to address {}", addr))?;

    axum::serve(listener, app.into_make_service())
        .await
        .context("failed to launch Leptos app")?;

    Ok(())
}
