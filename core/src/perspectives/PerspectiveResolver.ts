import { Arg, Mutation, PubSub, Query, Resolver, Subscription } from "type-graphql";
import { LinkExpression, LinkExpressionInput, LinkExpressionMutations, LinkExpressionUpdated, LinkInput, LinkMutations } from "../links/Links";
import { Neighbourhood } from "../neighbourhood/Neighbourhood";
import { LinkQuery } from "./LinkQuery";
import { Perspective } from "./Perspective";
import { LinkStatus } from "./PerspectiveProxy";
import { PerspectiveHandle, PerspectiveState } from "./PerspectiveHandle";
import { LINK_ADDED_TOPIC, LINK_REMOVED_TOPIC, LINK_UDATED_TOPIC, PERSPECTIVE_ADDED_TOPIC, PERSPECTIVE_REMOVED_TOPIC, PERSPECTIVE_UPDATED_TOPIC, PERSPECTIVE_SYNC_STATE_CHANGE } from '../PubSub'

export const testLink = new LinkExpression()
testLink.author = "did:ad4m:test"
testLink.timestamp = Date.now()
testLink.data = {
    source: 'root',
    target: 'neighbourhood://Qm12345'
}
testLink.proof = {
    signature: '',
    key: '',
    valid: true
}

/**
 * Resolver classes are used here to define the GraphQL schema
 * (through the type-graphql annotations)
 * and are spawned in the client tests in Ad4mClient.test.ts.
 * For the latter, they return test fixtures.
 */
@Resolver()
export default class PerspectiveResolver {
    @Query(returns => [PerspectiveHandle])
    perspectives(): PerspectiveHandle[] {
        const p1 = new PerspectiveHandle()
        p1.name = 'test-perspective-1'
        p1.uuid = '00001'
        const p2 = new PerspectiveHandle()
        p2.name = 'test-perspective-2'
        p2.uuid = '00002'
        p2.sharedUrl = 'neighbourhood://Qm12345'
        p2.neighbourhood = new Neighbourhood("language://Qm12345", new Perspective())
        p2.state = PerspectiveState.Synced
        return [p1, p2]
    }

    @Query(returns => PerspectiveHandle, {nullable: true})
    perspective(@Arg('uuid') uuid: string): PerspectiveHandle|null {
        return new PerspectiveHandle(uuid, 'test-perspective-1')
    }

    @Query(returns => Perspective, {nullable: true})
    perspectiveSnapshot(@Arg('uuid') uuid: string): Perspective|null {
        return new Perspective([testLink])
    }

    @Mutation(returns => String, {nullable: true})
    perspectivePublishSnapshot(@Arg('uuid') uuid: string): String|null {
        return 'perspective://Qm12345'
    }

    @Query(returns => [LinkExpression], {nullable: true})
    perspectiveQueryLinks(@Arg('uuid') uuid: string, @Arg('query') query: LinkQuery): LinkExpression[] {
        return [testLink]
    }

    @Query(returns => String)
    perspectiveQueryProlog(@Arg('uuid') uuid: string, @Arg('query') query: String): string {
        return `[{"X": 1}]`
    }

    @Mutation(returns => PerspectiveHandle)
    perspectiveAdd(@Arg('name') name: string, @PubSub() pubSub: any): PerspectiveHandle {
        const perspective = new PerspectiveHandle('00006', name);
        pubSub.publish(PERSPECTIVE_ADDED_TOPIC, { perspective })
        return new PerspectiveHandle('00006', name)
    }

    @Mutation(returns => PerspectiveHandle, {nullable: true})
    perspectiveUpdate(@Arg('uuid') uuid: string, @Arg('name') name: string, @PubSub() pubSub: any): PerspectiveHandle {
        const perspective = new PerspectiveHandle(uuid, name);
        pubSub.publish(PERSPECTIVE_UPDATED_TOPIC, { perspective })
        return new PerspectiveHandle(uuid, name)
    }

    @Mutation(returns => Boolean)
    perspectiveRemove(@Arg('uuid') uuid: string, @PubSub() pubSub: any): boolean {
        const perspective = new PerspectiveHandle(uuid);
        pubSub.publish(PERSPECTIVE_REMOVED_TOPIC, { perspective })
        return true
    }

    @Mutation(returns => LinkExpression)
    perspectiveAddLink(@Arg('uuid') uuid: string, @Arg('link') link: LinkInput, @Arg('status', { nullable: true, defaultValue: 'shared'}) status: LinkStatus, @PubSub() pubSub: any): LinkExpression {
        const l = new LinkExpression()
        l.author = 'did:ad4m:test'
        l.timestamp = Date.now()
        l.proof = testLink.proof
        l.data = link
        l.status = status

        pubSub.publish(LINK_ADDED_TOPIC, { link: l })
        pubSub.publish(PERSPECTIVE_SYNC_STATE_CHANGE, PerspectiveState.LinkLanguageInstalledButNotSynced)
        return l
    }

    @Mutation(returns => [LinkExpression])
    perspectiveAddLinks(@Arg('uuid') uuid: string, @Arg('links', type => [LinkInput]) links: LinkInput[], @Arg('status', { nullable: true}) status: string, @PubSub() pubSub: any): LinkExpression[] {
        const l = new LinkExpression()
        l.author = 'did:ad4m:test'
        l.timestamp = Date.now()
        l.proof = testLink.proof
        l.data = links[0]

        const l2 = new LinkExpression()
        l2.author = 'did:ad4m:test'
        l2.timestamp = Date.now()
        l2.proof = testLink.proof
        l2.data = links[0]

        pubSub.publish(LINK_ADDED_TOPIC, { link: l })
        pubSub.publish(LINK_ADDED_TOPIC, { link: l2 })
        return [l, l2]
    }

    @Mutation(returns => [LinkExpression])
    perspectiveRemoveLinks(@Arg('uuid') uuid: string, @Arg('links', type => [LinkExpressionInput]) links: LinkExpressionInput[], @PubSub() pubSub: any): LinkExpression[] {
        const l = new LinkExpression()
        l.author = 'did:ad4m:test'
        l.timestamp = Date.now()
        l.proof = testLink.proof
        l.data = links[0].data

        const l2 = new LinkExpression()
        l2.author = 'did:ad4m:test'
        l2.timestamp = Date.now()
        l2.proof = testLink.proof
        l2.data = links[0].data

        pubSub.publish(LINK_REMOVED_TOPIC, { link: l })
        pubSub.publish(LINK_ADDED_TOPIC, { link: l2 })
        return [l, l2]
    }

    @Mutation(returns => LinkExpressionMutations)
    perspectiveLinkMutations(@Arg('uuid') uuid: string, @Arg('mutations') mutations: LinkMutations, @Arg('status', { nullable: true}) status: LinkStatus, @PubSub() pubSub: any): LinkExpressionMutations {
        const perspectiveAddLinks = this.perspectiveAddLinks(uuid, mutations.additions, status, pubSub);
        const perspectiveRemoveLinks = this.perspectiveRemoveLinks(uuid, mutations.removals, pubSub);
        return new LinkExpressionMutations(perspectiveAddLinks, perspectiveRemoveLinks)
    }

    @Mutation(returns => LinkExpression)
    perspectiveAddLinkExpression(@Arg('uuid') uuid: string, @Arg('link') link: LinkExpressionInput, @Arg('status', { nullable: true}) status: LinkStatus, @PubSub() pubSub: any): LinkExpression {
        pubSub.publish(LINK_ADDED_TOPIC, { link })
        link.status = status;
        return link as LinkExpression
    }

    @Mutation(returns => LinkExpression)
    perspectiveUpdateLink(@Arg('uuid') uuid: string, @Arg('oldLink') oldlink: LinkExpressionInput, @Arg('newLink') newlink: LinkInput, @PubSub() pubSub: any): LinkExpression {
        const l = new LinkExpression()
        l.author = 'did:ad4m:test'
        l.timestamp = Date.now()
        l.proof = testLink.proof
        l.data = newlink

        pubSub.publish(LINK_REMOVED_TOPIC, { link: l })

        return l
    }

    @Mutation(returns => Boolean)
    perspectiveRemoveLink(@Arg('uuid') uuid: string, @Arg('link') link: LinkExpressionInput, @PubSub() pubSub: any): Boolean {
        pubSub.publish(LINK_REMOVED_TOPIC)
        return true
    }

    @Subscription({topics: PERSPECTIVE_ADDED_TOPIC, nullable: true})
    perspectiveAdded(): PerspectiveHandle {
        const perspective = new PerspectiveHandle('00001', 'New Perspective');
        return perspective
    }

    @Subscription({topics: PERSPECTIVE_UPDATED_TOPIC, nullable: true})
    perspectiveUpdated(): PerspectiveHandle {
        return new PerspectiveHandle('00001', 'New Perspective')
    }

    @Subscription({topics: PERSPECTIVE_REMOVED_TOPIC, nullable: true})
    perspectiveRemoved(): string {
        return '00006'
    }

    @Subscription({topics: LINK_ADDED_TOPIC, nullable: true})
    perspectiveLinkAdded(@Arg('uuid') uuid: string): LinkExpression {
        return testLink
    }

    @Subscription({topics: LINK_REMOVED_TOPIC, nullable: true})
    perspectiveLinkRemoved(@Arg('uuid') uuid: string): LinkExpression {
        return testLink
    }

    @Subscription({topics: LINK_UDATED_TOPIC, nullable: true})
    perspectiveLinkUpdated(@Arg('uuid') uuid: string): LinkExpressionUpdated {
        return {oldLink: testLink, newLink: testLink}
    }

    @Subscription({topics: PERSPECTIVE_SYNC_STATE_CHANGE, nullable: false})
    perspectiveSyncStateChange(@Arg('uuid') uuid: string): PerspectiveState {
        return PerspectiveState.Synced
    }
}