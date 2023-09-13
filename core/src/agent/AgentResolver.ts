import {
  Arg,
  Mutation,
  Query,
  Resolver,
  Subscription,
  PubSub,
} from "type-graphql";
import { Perspective, PerspectiveInput } from "../perspectives/Perspective";
import {
  Agent,
  AgentSignature,
  Apps,
  AuthInfoInput,
  EntanglementProof,
  EntanglementProofInput,
} from "./Agent";
import { AgentStatus } from "./AgentStatus";
import { AGENT_STATUS_CHANGED, AGENT_UPDATED, APPS_CHANGED } from "../PubSub";

export const TEST_AGENT_DID = "did:ad4m:test";

@Resolver()
export default class AgentResolver {
  @Query((returns) => Agent)
  agent(): Agent {
    return new Agent(TEST_AGENT_DID);
  }

  @Query((returns) => AgentStatus)
  agentStatus(): AgentStatus {
    return new AgentStatus({ did: TEST_AGENT_DID });
  }

  @Mutation((returns) => AgentStatus)
  agentGenerate(
    @Arg("passphrase") passphrase: string,
    @PubSub() pubSub: any
  ): AgentStatus {
    const status = new AgentStatus({
      did: TEST_AGENT_DID,
      didDocument: "did document",
      isInitialized: true,
      isUnlocked: true,
    });
    pubSub.publish(AGENT_STATUS_CHANGED, { status });
    return status;
  }

  @Mutation((returns) => AgentStatus)
  agentImport(
    @Arg("did") did: string,
    @Arg("didDocument") didDocument: string,
    @Arg("keystore") keystore: string,
    @Arg("passphrase") passphrase: string
  ): AgentStatus {
    return new AgentStatus({
      did,
      didDocument,
      isInitialized: true,
      isUnlocked: true,
    });
  }

  @Mutation((returns) => AgentStatus)
  agentLock(
    @Arg("passphrase") passphrase: string,
    @PubSub() pubSub: any
  ): AgentStatus {
    const status = new AgentStatus({
      isInitialized: true,
      did: TEST_AGENT_DID,
    });
    pubSub.publish(AGENT_STATUS_CHANGED, { status });
    return status;
  }

  @Mutation((returns) => AgentStatus)
  agentUnlock(
    @Arg("passphrase") passphrase: string,
    @PubSub() pubSub: any
  ): AgentStatus {
    const status = new AgentStatus({
      isInitialized: true,
      isUnlocked: true,
      did: TEST_AGENT_DID,
    });
    pubSub.publish(AGENT_STATUS_CHANGED, { status });
    return status;
  }

  @Query((returns) => Agent, { nullable: true })
  agentByDID(@Arg("did") did: string): Agent {
    return new Agent(did);
  }

  @Mutation((returns) => Agent)
  agentUpdatePublicPerspective(
    @Arg("perspective") perspective: PerspectiveInput,
    @PubSub() pubSub: any
  ): Agent {
    const agent = new Agent(TEST_AGENT_DID, perspective as Perspective);
    agent.directMessageLanguage = "lang://test";
    pubSub.publish(AGENT_UPDATED, { agent });
    return agent;
  }

  @Mutation((returns) => Agent)
  agentUpdateDirectMessageLanguage(
    @Arg("directMessageLanguage") directMessageLanguage: string,
    @PubSub() pubSub: any
  ): Agent {
    const agent = new Agent(TEST_AGENT_DID, new Perspective());
    agent.directMessageLanguage = directMessageLanguage;
    pubSub.publish(AGENT_UPDATED, { agent });
    return agent;
  }

  @Subscription({ topics: AGENT_UPDATED, nullable: true })
  agentUpdated(): Agent {
    return new Agent(TEST_AGENT_DID);
  }

  @Subscription({ topics: AGENT_STATUS_CHANGED, nullable: true })
  agentStatusChanged(): AgentStatus {
    return new AgentStatus({ isInitialized: true, did: TEST_AGENT_DID });
  }

  @Mutation((returns) => [EntanglementProof])
  agentAddEntanglementProofs(
    @Arg("proofs", (type) => [EntanglementProofInput])
    proofs: EntanglementProofInput[]
  ): EntanglementProof[] {
    return [
      new EntanglementProof(
        "did:key:hash",
        "did-key-id",
        "ethereum",
        "ethAddr",
        "sig",
        "sig2"
      ),
    ];
  }

  @Mutation((returns) => [EntanglementProof])
  agentDeleteEntanglementProofs(
    @Arg("proofs", (type) => [EntanglementProofInput])
    proofs: EntanglementProofInput[]
  ): EntanglementProof[] {
    return [
      new EntanglementProof(
        "did:key:hash",
        "did-key-id",
        "ethereum",
        "ethAddr",
        "sig",
        "sig2"
      ),
    ];
  }

  @Query((returns) => [EntanglementProof])
  agentGetEntanglementProofs(): EntanglementProof[] {
    return [
      new EntanglementProof(
        "did:key:hash",
        "did-key-id",
        "ethereum",
        "ethAddr",
        "sig",
        "sig2"
      ),
    ];
  }

  @Mutation((returns) => EntanglementProof)
  agentEntanglementProofPreFlight(
    @Arg("deviceKey") deviceKey: string,
    @Arg("deviceKeyType") deviceKeyType: string
  ): EntanglementProof {
    return new EntanglementProof(
      "did:key:hash",
      "did-key-id",
      deviceKeyType,
      deviceKey,
      "sig"
    );
  }

  @Mutation((returns) => String)
  agentRequestCapability(@Arg("authInfo") authInfo: AuthInfoInput): String {
    return "test-request-id";
  }

  @Mutation((returns) => String)
  agentPermitCapability(@Arg("auth") auth: string): String {
    return "123";
  }

  @Mutation((returns) => String)
  agentGenerateJwt(
    @Arg("requestId") requestId: string,
    @Arg("rand") rand: string
  ): String {
    return "test-jwt";
  }

  @Query((returns) => Boolean)
  agentIsLocked(): Boolean {
    return false;
  }

  @Query((returns) => [Apps])
  agentGetApps(): [] {
    return [];
  }

  
  @Subscription((returns) => [Apps], { topics: APPS_CHANGED, nullable: true })
  agentAppsChanged(): null {
    return null;
  }


  @Mutation((returns) => [Apps])
  agentRemoveApp(@Arg("requestId") requestId: string, @PubSub() pubSub: any): [] {
    pubSub.publish(APPS_CHANGED, { });
    return [];
  }

  @Mutation((returns) => [Apps])
  agentRevokeToken(@Arg("requestId") requestId: string): any[] {
    return [
      {
        revoked: true,
        requestId: "test-request-id",
        auth: {
          appName: "test-app",
          appDesc: "-",
          appUrl: "-",
          appIconPath: "_",
          capabilities: [
            {
              with: {
                domain: "*",
                pointers: ["*"],
              },
              can: ["*"],
            },
          ],
        },
        token: "test-token",
      },
    ];
  }

  @Mutation((returns) => AgentSignature)
  agentSignMessage(@Arg("message") message: string): AgentSignature {
    return new AgentSignature("test-message-signature", "test-public-key");
  }
}
