import { Ad4mClient } from '@coasys/ad4m';
import { ApolloClient, InMemoryCache } from '@apollo/client/core';
import { GraphQLWsLink } from "@apollo/client/link/subscriptions";
import WebSocket from 'ws';
import { createClient } from 'graphql-ws';

export function buildAd4mClient(port: number): Ad4mClient {
  const token = global.ad4mToken;

  const wsLink = new GraphQLWsLink(createClient({
    url: `ws://localhost:${port}/graphql`,
    webSocketImpl: WebSocket,
    connectionParams: () => {
        return {
            headers: {
                authorization: token
            }
        }
    },
}));

  const apolloClient = new ApolloClient({
      link: wsLink,
      cache: new InMemoryCache({ resultCaching: false, addTypename: false }),
      defaultOptions: {
          watchQuery: {
              fetchPolicy: "no-cache",
          },
          query: {
              fetchPolicy: "no-cache",
          }
      },
  });

  // @ts-ignore
  return new Ad4mClient(apolloClient, false)
}