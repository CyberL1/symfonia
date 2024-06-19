use chorus::types::{PermissionFlags, Snowflake};
use poem::{Endpoint, Middleware, Request};
use sqlx::MySqlPool;

use crate::database::entities::User;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionCheckType {
    ChannelView,
}

pub struct PermissionGuardMiddleware(PermissionCheckType);

impl<E: Endpoint> Middleware<E> for PermissionGuardMiddleware {
    type Output = PermissionGuardMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        Self::Output {
            ep,
            check_type: self.0,
        }
    }
}

pub struct PermissionGuardMiddlewareImpl<E> {
    ep: E,
    check_type: PermissionCheckType,
}

impl<E: Endpoint> Endpoint for PermissionGuardMiddlewareImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        let db = req
            .data::<MySqlPool>()
            .expect("Failed to get database connection");

        if let Some(user) = req.data::<User>() {
            match self.check_type {
                PermissionCheckType::ChannelView => {
                    let channel_id = req
                        .raw_path_param("channel_id")
                        .map(|s| Snowflake(s.parse().unwrap()))
                        .unwrap();
                    check_channel_permissions(db, channel_id, PermissionFlags::VIEW_CHANNEL)
                        .await?;
                }
            }
        }

        self.ep.call(req).await
    }
}

async fn check_channel_permissions(
    db: &MySqlPool,
    channel_id: Snowflake,
    permissions: PermissionFlags,
) -> poem::Result<()> {
    todo!("Check channel permissions");
    Ok(())
}
