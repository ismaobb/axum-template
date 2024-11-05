## Axum中间件路由规则
```rust
use axum::{routing::get, Router};
async fn handler() {}
let app = Router::new()
        .route("/users", get(handler))
        .layer(role_auth)
        .route("/", get(handler))
        .layer(bearer_auth)
        .route("/whites",get(handler))
        .layer(public_layer)
```

```
> middleware
        requests
           |
           v
+----- public_layer -----+(响应头修改，日志打印)
| +---- bearer_auth ----+ |
| | +-- role_auth --+ | |
| | |               | | |
| | |    handler    | | |
| | |               | | |
| | +-- role_auth --+ | |
| +---- bearer_auth ----+ |
+----- public_layer -----+
           |
           v
        responses
```