import { Expression } from "@perspect3vism/ad4m";
import faker from 'faker'

export function createMockExpression(did: string, data: object): Expression {
    return {
        author: did,
        timestamp: faker.date.recent().toISOString(),
        data,
        proof: {
            signature: "abcdefgh",
            key: `${did}#primary`
        }
    }
}
