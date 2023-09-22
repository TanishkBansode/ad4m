#![allow(non_snake_case)]
use futures::stream;
use futures::stream::Stream;
use juniper::FieldResult;
use std::pin::Pin;

use crate::pubsub::{
    get_global_pubsub, subscribe_and_process, AGENT_STATUS_CHANGED_TOPIC, AGENT_UPDATED_TOPIC,
    APPS_CHANGED, EXCEPTION_OCCURRED_TOPIC, NEIGHBOURHOOD_SIGNAL_TOPIC, PERSPECTIVE_ADDED_TOPIC,
    PERSPECTIVE_LINK_ADDED_TOPIC, PERSPECTIVE_LINK_REMOVED_TOPIC, PERSPECTIVE_LINK_UPDATED_TOPIC,
    PERSPECTIVE_REMOVED_TOPIC, PERSPECTIVE_SYNC_STATE_CHANGE_TOPIC, PERSPECTIVE_UPDATED_TOPIC,
    RUNTIME_MESSAGED_RECEIVED_TOPIC,
};

use super::graphql_types::*;
use super::utils::{
    check_capabilities, get_capabilies, AGENT_SUBSCRIBE_CAPABILITY,
    PERSPECTIVE_SUBSCRIBE_CAPABILITY, RUNTIME_EXCEPTION_SUBSCRIBE_CAPABILITY,
    RUNTIME_MESSAGES_SUBSCRIBE_CAPABILITY,
};
use super::RequestContext;

pub struct Subscription;

#[juniper::graphql_subscription(context = RequestContext)]
impl Subscription {
    async fn agent_status_changed(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<AgentStatus>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }
        let pubsub = get_global_pubsub().await;
        let topic = &AGENT_STATUS_CHANGED_TOPIC;

        subscribe_and_process::<AgentStatus>(pubsub, topic.to_string(), None).await
    }

    async fn agent_apps_changed(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<Option<Apps>>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }
        let pubsub = get_global_pubsub().await;
        let topic = &APPS_CHANGED;

        subscribe_and_process::<Option<Apps>>(pubsub, topic.to_string(), None).await
    }

    async fn agent_updated(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<Agent>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }

        let cap_check = check_capabilities(
            context.js_handle.clone(),
            capabilities.unwrap(),
            serde_json::to_value(AGENT_SUBSCRIBE_CAPABILITY.clone()).unwrap(),
        )
        .await;

        if cap_check.is_err() {
            return Box::pin(stream::once(async move { Err(cap_check.err().unwrap()) }));
        }

        let pubsub = get_global_pubsub().await;
        let topic = &AGENT_UPDATED_TOPIC;

        subscribe_and_process::<Agent>(pubsub, topic.to_string(), None).await
    }

    async fn exception_occurred(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<ExceptionInfo>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }

        let cap_check = check_capabilities(
            context.js_handle.clone(),
            capabilities.unwrap(),
            serde_json::to_value(RUNTIME_EXCEPTION_SUBSCRIBE_CAPABILITY.clone()).unwrap(),
        )
        .await;

        if cap_check.is_err() {
            return Box::pin(stream::once(async move { Err(cap_check.err().unwrap()) }));
        }

        let pubsub = get_global_pubsub().await;
        let topic = &EXCEPTION_OCCURRED_TOPIC;

        subscribe_and_process::<ExceptionInfo>(pubsub, topic.to_string(), None).await
    }

    async fn neighbourhood_signal(
        &self,
        context: &RequestContext,
        perspectiveUUID: String,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<PerspectiveExpression>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }

        let cap_check = check_capabilities(
            context.js_handle.clone(),
            capabilities.unwrap(),
            serde_json::to_value(RUNTIME_MESSAGES_SUBSCRIBE_CAPABILITY.clone()).unwrap(),
        )
        .await;

        if cap_check.is_err() {
            return Box::pin(stream::once(async move { Err(cap_check.err().unwrap()) }));
        };

        let pubsub = get_global_pubsub().await;
        let topic = &NEIGHBOURHOOD_SIGNAL_TOPIC;

        subscribe_and_process::<NeighbourhoodSignalFilter>(
            pubsub,
            topic.to_string(),
            Some(perspectiveUUID),
        )
        .await
    }

    async fn perspective_added(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<PerspectiveHandle>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }

        let cap_check = check_capabilities(
            context.js_handle.clone(),
            capabilities.unwrap(),
            serde_json::to_value(PERSPECTIVE_SUBSCRIBE_CAPABILITY.clone()).unwrap(),
        )
        .await;

        if cap_check.is_err() {
            return Box::pin(stream::once(async move { Err(cap_check.err().unwrap()) }));
        };

        let pubsub = get_global_pubsub().await;
        let topic = &PERSPECTIVE_ADDED_TOPIC;

        subscribe_and_process::<PerspectiveHandle>(pubsub, topic.to_string(), None).await
    }

    async fn perspective_link_added(
        &self,
        context: &RequestContext,
        uuid: String,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<LinkExpression>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }

        let cap_check = check_capabilities(
            context.js_handle.clone(),
            capabilities.unwrap(),
            serde_json::to_value(PERSPECTIVE_SUBSCRIBE_CAPABILITY.clone()).unwrap(),
        )
        .await;

        if cap_check.is_err() {
            return Box::pin(stream::once(async move { Err(cap_check.err().unwrap()) }));
        };

        let pubsub = get_global_pubsub().await;
        let topic = &PERSPECTIVE_LINK_ADDED_TOPIC;

        subscribe_and_process::<PerspectiveLinkFilter>(pubsub, topic.to_string(), Some(uuid)).await
    }

    async fn perspective_link_removed(
        &self,
        context: &RequestContext,
        uuid: String,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<LinkExpression>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }

        let cap_check = check_capabilities(
            context.js_handle.clone(),
            capabilities.unwrap(),
            serde_json::to_value(PERSPECTIVE_SUBSCRIBE_CAPABILITY.clone()).unwrap(),
        )
        .await;

        if cap_check.is_err() {
            return Box::pin(stream::once(async move { Err(cap_check.err().unwrap()) }));
        };

        let pubsub = get_global_pubsub().await;
        let topic = &PERSPECTIVE_LINK_REMOVED_TOPIC;

        subscribe_and_process::<PerspectiveLinkFilter>(pubsub, topic.to_string(), Some(uuid)).await
    }

    async fn perspective_link_updated(
        &self,
        context: &RequestContext,
        uuid: String,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<LinkUpdated>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }
        let pubsub = get_global_pubsub().await;
        let topic = &PERSPECTIVE_LINK_UPDATED_TOPIC;

        subscribe_and_process::<PerspectiveLinkUpdatedFilter>(pubsub, topic.to_string(), Some(uuid))
            .await
    }

    async fn perspective_removed(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<String>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }

        let cap_check = check_capabilities(
            context.js_handle.clone(),
            capabilities.unwrap(),
            serde_json::to_value(PERSPECTIVE_SUBSCRIBE_CAPABILITY.clone()).unwrap(),
        )
        .await;

        if cap_check.is_err() {
            return Box::pin(stream::once(async move { Err(cap_check.err().unwrap()) }));
        };

        let pubsub = get_global_pubsub().await;
        let topic = &PERSPECTIVE_REMOVED_TOPIC;

        subscribe_and_process::<String>(pubsub, topic.to_string(), None).await
    }

    async fn perspective_sync_state_change(
        &self,
        context: &RequestContext,
        uuid: String,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<String>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }

        let cap_check = check_capabilities(
            context.js_handle.clone(),
            capabilities.unwrap(),
            serde_json::to_value(PERSPECTIVE_SUBSCRIBE_CAPABILITY.clone()).unwrap(),
        )
        .await;

        if cap_check.is_err() {
            return Box::pin(stream::once(async move { Err(cap_check.err().unwrap()) }));
        };

        let pubsub = get_global_pubsub().await;
        let topic = &PERSPECTIVE_SYNC_STATE_CHANGE_TOPIC;

        subscribe_and_process::<PerspectiveStateFilter>(pubsub, topic.to_string(), Some(uuid)).await
    }

    async fn perspective_updated(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<PerspectiveHandle>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }

        let cap_check = check_capabilities(
            context.js_handle.clone(),
            capabilities.unwrap(),
            serde_json::to_value(PERSPECTIVE_SUBSCRIBE_CAPABILITY.clone()).unwrap(),
        )
        .await;

        if cap_check.is_err() {
            return Box::pin(stream::once(async move { Err(cap_check.err().unwrap()) }));
        };

        let pubsub = get_global_pubsub().await;
        let topic = &PERSPECTIVE_UPDATED_TOPIC;

        subscribe_and_process::<PerspectiveHandle>(pubsub, topic.to_string(), None).await
    }

    async fn runtime_message_received(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<PerspectiveExpression>> + Send>> {
        let capabilities =
            get_capabilies(context.js_handle.clone(), context.capability.clone()).await;
        if capabilities.is_err() {
            return Box::pin(stream::once(
                async move { Err(capabilities.err().unwrap()) },
            ));
        }

        let cap_check = check_capabilities(
            context.js_handle.clone(),
            capabilities.unwrap(),
            serde_json::to_value(PERSPECTIVE_SUBSCRIBE_CAPABILITY.clone()).unwrap(),
        )
        .await;

        if cap_check.is_err() {
            return Box::pin(stream::once(async move { Err(cap_check.err().unwrap()) }));
        };

        let pubsub = get_global_pubsub().await;
        let topic = &RUNTIME_MESSAGED_RECEIVED_TOPIC;

        subscribe_and_process::<PerspectiveExpression>(pubsub, topic.to_string(), None).await
    }
}
