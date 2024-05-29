import * as path from "node:path";
import * as fs from "node:fs";
import {
  Language,
  Expression,
  PublicSharing,
  ReadOnlyLanguage,
} from "@coasys/ad4m";
import { Agent, ExpressionProof, AgentSignature } from "@coasys/ad4m";
import * as PubSubDefinitions from "../graphQL-interface/SubscriptionDefinitions";
import { resolver } from "@transmute/did-key.js";
import { getPubSub, tagExpressionSignatureStatus } from "../utils";


export default class AgentService {
  #agentLanguage?: Language;
  #pubSub: PubSub;

  constructor(rootConfigPath: string, adminCredential?: string) {
    this.#pubSub = getPubSub();
  }

  getTaggedAgentCopy(): Agent {
    const agent = AGENT.agent();
    if (!agent) throw new Error("No agent");
    const copy = JSON.parse(JSON.stringify(agent));
    if(copy.perspective) {
        for(let link of copy.perspective.links) {
            tagExpressionSignatureStatus(link)
        }
    }
    return copy;
  }

  createSignedExpression(data: any): Expression {
    return AGENT.createSignedExpression(data);
  }

  get did(): string {
    return AGENT.did();
  }

  get agent(): Agent {
    return AGENT.agent();
  }

  async updateAgent(a: Agent) {
    AGENT.save_agent_profile(a);
    await this.storeAgentProfile();
    await this.#pubSub.publish(PubSubDefinitions.AGENT_UPDATED, this.getTaggedAgentCopy());
  }

  setAgentLanguage(lang: Language) {
    this.#agentLanguage = lang;
  }

  getAgentLanguage(): Language {
    if (!this.#agentLanguage) {
      throw new Error("AgentService ERROR: No agent language");
    }
    return this.#agentLanguage;
  }

  async ensureAgentExpression() {
    const currentAgent = AGENT.agent();
    const agentDid = AGENT.did();
    if (!agentDid) throw Error("No agent did found");

    const agentLanguage = this.getAgentLanguage();

    if (!agentLanguage.expressionAdapter!) {
      throw Error("No expression adapter found");
    }

    const agentExpression = await agentLanguage.expressionAdapter!.get(
      agentDid
    );

    if (!agentExpression) {
      if (currentAgent) {
        await this.updateAgent(currentAgent);
      }
    }
  }

  async storeAgentProfile() {
    let agent = AGENT.agent();

    console.log("Storing agent profile", JSON.stringify(agent));

    const agentLanguage = this.getAgentLanguage();

    if (agent?.did) {
      let adapter = agentLanguage.expressionAdapter!.putAdapter;

      let isPublic = function isPublic(
        adapter: PublicSharing | ReadOnlyLanguage
      ): adapter is PublicSharing {
        return (adapter as PublicSharing).createPublic !== undefined;
      };

      try {
        if (isPublic(adapter)) {
          await adapter.createPublic(agent);
        } else {
          console.warn("Got a ReadOnlyLanguage for agent language");
        }
      } catch (e) {
        throw new Error(
          `Incompatible putAdapter in AgentLanguage}\nError was: ${e}`
        );
      }
    }
  }

  // TODO: need to be removed once runtime stuff gets merged
  isInitialized() {
    return AGENT.isInitialized();
  }

  // TODO: need to be removed once runtime stuff gets merged
  isUnlocked() {
    return AGENT.isUnlocked();
  }

  async unlock(password: string) {
    AGENT.unlock(password);
    AGENT.load();
    try {
      await this.storeAgentProfile();
    } catch (e) {
      console.debug(
        "Error when trying to store agent profile during unlock: ",
        e
      );
      console.debug("Continuing anyway...");
    }
  }
}

export function init(
  rootConfigPath: string,
  adminCredential?: string
): AgentService {
  const agent = new AgentService(rootConfigPath, adminCredential);
  AGENT.load();
  return agent;
}
