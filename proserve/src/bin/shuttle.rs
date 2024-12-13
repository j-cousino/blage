use proserve::build_router;

#[tracing::instrument]
#[shuttle_runtime::main]
async fn shuttle() -> shuttle_axum::ShuttleAxum {
    tracing::info!("Starting");

    Ok(build_router().into())
}




