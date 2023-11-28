import type { Address, Agent, Expression, PublicSharing, LanguageContext, HolochainLanguageDelegate, ExpressionAdapter, AgentService } from "https://esm.sh/@perspect3vism/ad4m@0.5.0";
import axiod from "https://deno.land/x/axiod/mod.ts";

export default class ExpressionAdapterImpl implements ExpressionAdapter {
  #agent: AgentService;
  putAdapter: PublicSharing

  constructor(context: LanguageContext) {
    this.#agent = context.agent;
    this.putAdapter = new Sharing(context)
  }

  async get(did: Address): Promise<Expression> {
    console.log("Getting expression with did", did);

    const { expression } = await axiod.get("https://socket.ad4m.dev/agent", {
      params: {
        did
      }
    });

    return expression
  };
}

class Sharing implements PublicSharing {
  #agent: AgentService;

  constructor(context: LanguageContext) {
    this.#agent = context.agent; 
  }

  async createPublic(content: Agent): Promise<Address> {

    if(!content['did'] || !content['perspective'] || !content['perspective'].links)
      throw "Content must be an Agent object"

    const agent = content as Agent
    if(agent.did != this.#agent.did)
      throw "Can't set Agent Expression for foreign DID - only for self"

    if(!agent.directMessageLanguage)
      agent.directMessageLanguage = undefined

    agent.perspective!.links.forEach(link => {
      delete link.proof.valid
      delete link.proof.invalid
    })

    const expression = this.#agent.createSignedExpression(agent);
    
    await axiod.post("https://socket.ad4m.dev/agent", {
      params: {
        did: agent.did,
        expression
      }
    });

    return agent.did
  }
}