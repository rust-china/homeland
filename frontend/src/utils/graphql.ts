// https://v4.apollo.vuejs.org/guide/installation.html#compatibility
// https://v4.apollo.vuejs.org/guide-composable/query.html#usequery
import { split, ApolloClient, createHttpLink, InMemoryCache, gql } from '@apollo/client/core'
import { getMainDefinition } from '@apollo/client/utilities'
import { GraphQLWsLink } from '@apollo/client/link/subscriptions'
import { createClient } from 'graphql-ws'

export * from '@vue/apollo-composable'


const httpLink = createHttpLink({
	uri: import.meta.env.VITE_APP_GRAPHQL_HTTP_URL,
});

const wsLink = typeof window !== "undefined"
	? new GraphQLWsLink(createClient({
		url: () => {
			// return `${location.origin.replace(/^http/, 'ws')}${import.meta.env.VITE_APP_API_BASE_URL}/graphql/ws`
			return import.meta.env.VITE_APP_GRAPHQL_WEBSOCKET_URL
		},
		lazy: true,
		// connectionParams: {
		// 	authToken: user.authToken,
		// },
	})) : null;

const splitLink = typeof window !== "undefined" && wsLink != null ? split(
	({ query }) => {
		const definition = getMainDefinition(query);
		return (
			definition.kind === 'OperationDefinition' &&
			definition.operation === 'subscription'
		);
	},
	wsLink,
	httpLink,
) : httpLink

const client = new ApolloClient({
	link: splitLink,
	cache: new InMemoryCache(),
	connectToDevTools: false,
	ssrMode: import.meta.env.SSR,
});

export {
	client, gql, useQuery, useLazyQuery, useMutation, useSubscription
}