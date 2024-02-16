use super::types::{Capability, Resource};
pub const WILD_CARD: &str = "*";

// capabilities operations
pub const READ: &str = "READ";
pub const CREATE: &str = "CREATE";
pub const UPDATE: &str = "UPDATE";
pub const DELETE: &str = "DELETE";
pub const SUBSCRIBE: &str = "SUBSCRIBE";

// capabilities domains
pub const AGENT: &str = "agent";
pub const EXPRESSION: &str = "expression";
pub const LANGUAGE: &str = "language";
pub const PERSPECTIVE: &str = "perspective";
pub const NEIGHBOURHOOD: &str = "neighbourhood";
pub const RUNTIME: &str = "runtime";
pub const RUNTIME_TRUSTED_AGENTS: &str = "runtime.trusted_agents";
pub const RUNTIME_KNOWN_LINK_LANGUAGES: &str = "runtime.known_link_languages";
pub const RUNTIME_FRIENDS: &str = "runtime.friends";
pub const RUNTIME_MESSAGES: &str = "runtime.messages";


// admin capabilities
lazy_static! {
    pub static ref ALL_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: WILD_CARD.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![WILD_CARD.to_string()],
    };
    // agent related capabilities
    pub static ref AGENT_AUTH_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: AGENT.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec!["AUTHENTICATE".to_string()],
    };
    pub static ref AGENT_READ_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: AGENT.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![READ.to_string()],
    };
    pub static ref AGENT_CREATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: AGENT.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![CREATE.to_string()],
    };
    pub static ref AGENT_UPDATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: AGENT.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![UPDATE.to_string()],
    };
    pub static ref AGENT_LOCK_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: AGENT.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec!["LOCK".to_string()],
    };
    pub static ref AGENT_UNLOCK_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: AGENT.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec!["UNLOCK".to_string()],
    };
    pub static ref AGENT_PERMIT_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: AGENT.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec!["PERMIT".to_string()],
    };
    pub static ref AGENT_SUBSCRIBE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: AGENT.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![SUBSCRIBE.to_string()],
    };
    pub static ref AGENT_SIGN_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: AGENT.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec!["SIGN".to_string()],
    };

    // expression related capabilities
    pub static ref EXPRESSION_READ_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: EXPRESSION.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![READ.to_string()],
    };
    pub static ref EXPRESSION_CREATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: EXPRESSION.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![CREATE.to_string()],
    };
    pub static ref EXPRESSION_UPDATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: EXPRESSION.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![UPDATE.to_string()],
    };

    // language related capabilities
    pub static ref LANGUAGE_READ_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: LANGUAGE.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![READ.to_string()],
    };
    pub static ref LANGUAGE_CREATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: LANGUAGE.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![CREATE.to_string()],
    };
    pub static ref LANGUAGE_UPDATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: LANGUAGE.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![UPDATE.to_string()],
    };
    pub static ref LANGUAGE_DELETE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: LANGUAGE.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![DELETE.to_string()],
    };

    // perspective related capabilities
    pub static ref PERSPECTIVE_CREATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: PERSPECTIVE.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![CREATE.to_string()],
    };
}

#[allow(dead_code)]
pub fn perspective_query_capability(pointers: Vec<String>) -> Capability {
    Capability {
        with: Resource {
            domain: PERSPECTIVE.to_string(),
            pointers,
        },
        can: vec![READ.to_string()],
    }
}

#[allow(dead_code)]
pub fn perspective_update_capability(pointers: Vec<String>) -> Capability {
    Capability {
        with: Resource {
            domain: PERSPECTIVE.to_string(),
            pointers,
        },
        can: vec![UPDATE.to_string()],
    }
}

#[allow(dead_code)]
pub fn perspective_delete_capability(pointers: Vec<String>) -> Capability {
    Capability {
        with: Resource {
            domain: PERSPECTIVE.to_string(),
            pointers,
        },
        can: vec![DELETE.to_string()],
    }
}

lazy_static! {
    pub static ref PERSPECTIVE_SUBSCRIBE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: PERSPECTIVE.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![SUBSCRIBE.to_string()],
    };

    // neighbourhood related capabilities
    pub static ref NEIGHBOURHOOD_CREATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: NEIGHBOURHOOD.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![CREATE.to_string()],
    };

    pub static ref NEIGHBOURHOOD_READ_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: NEIGHBOURHOOD.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![READ.to_string()],
    };

    pub static ref NEIGHBOURHOOD_UPDATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: NEIGHBOURHOOD.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![UPDATE.to_string()],
    };

    // runtime related capabilities
    pub static ref RUNTIME_TRUSTED_AGENTS_READ_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_TRUSTED_AGENTS.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![READ.to_string()],
    };

    pub static ref RUNTIME_TRUSTED_AGENTS_CREATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_TRUSTED_AGENTS.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![CREATE.to_string()],
    };

    pub static ref RUNTIME_TRUSTED_AGENTS_DELETE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_TRUSTED_AGENTS.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![DELETE.to_string()],
    };

    pub static ref RUNTIME_KNOWN_LINK_LANGUAGES_READ_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_KNOWN_LINK_LANGUAGES.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![READ.to_string()],
    };

    pub static ref RUNTIME_KNOWN_LINK_LANGUAGES_CREATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_KNOWN_LINK_LANGUAGES.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![CREATE.to_string()],
    };

    pub static ref RUNTIME_KNOWN_LINK_LANGUAGES_DELETE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_KNOWN_LINK_LANGUAGES.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![DELETE.to_string()],
    };

    pub static ref RUNTIME_FRIENDS_READ_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_FRIENDS.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![READ.to_string()],
    };

    pub static ref RUNTIME_FRIENDS_CREATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_FRIENDS.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![CREATE.to_string()],
    };

    pub static ref RUNTIME_FRIENDS_DELETE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_FRIENDS.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![DELETE.to_string()],
    };

    pub static ref RUNTIME_FRIEND_STATUS_READ_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: "runtime.friend_status".to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![READ.to_string()],
    };

    pub static ref RUNTIME_MY_STATUS_UPDATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: "runtime.my_status".to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![UPDATE.to_string()],
    };

    pub static ref RUNTIME_HC_AGENT_INFO_READ_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: "runtime.hc_agent_info".to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![READ.to_string()],
    };

    pub static ref RUNTIME_HC_AGENT_INFO_CREATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: "runtime.hc_agent_info".to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![CREATE.to_string()],
    };

    pub static ref RUNTIME_MESSAGES_READ_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_MESSAGES.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![READ.to_string()],
    };

    pub static ref RUNTIME_MESSAGES_CREATE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_MESSAGES.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![CREATE.to_string()],
    };

    pub static ref RUNTIME_MESSAGES_SUBSCRIBE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME_MESSAGES.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![SUBSCRIBE.to_string()],
    };

    pub static ref RUNTIME_QUIT_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: RUNTIME.to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec!["QUIT".to_string()],
    };

    pub static ref RUNTIME_EXCEPTION_SUBSCRIBE_CAPABILITY: Capability = Capability {
        with: Resource {
            domain: "runtime.exception".to_string(),
            pointers: vec![WILD_CARD.to_string()],
        },
        can: vec![SUBSCRIBE.to_string()],
    };
}
