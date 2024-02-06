import { TestContext } from './integration.test'
import { sleep } from '../utils/utils'
import { expect } from "chai";

export default function agentLanguageTests(testContext: TestContext) {
    return () => {
        it.skip("works across remote agents", async () => {
            const alice = testContext.alice!
            const didAlice = (await alice.agent.status()).did!
            const bob = testContext.bob!    
            const didBob = (await bob.agent.status()).did!

            const aliceHerself = await alice.agent.me()
            const bobHimself = await bob.agent.me()

            await sleep(5000)

            const bobSeenFromAlice = await alice.agent.byDID(didBob)
            expect(bobSeenFromAlice).not.to.be.undefined
            expect(bobSeenFromAlice).to.be.eql(bobHimself)

            const aliceSeenFromBob = await bob.agent.byDID(didAlice)
            expect(aliceSeenFromBob).not.to.be.undefined
            expect(aliceSeenFromBob).to.be.eql(aliceHerself)
        })    
    }
}