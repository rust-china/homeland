// https://v4.apollo.vuejs.org/guide/installation.html#compatibility
// https://v4.apollo.vuejs.org/guide-composable/query.html#usequery
import { split, ApolloClient, createHttpLink, InMemoryCache, gql } from '@apollo/client/core'
import { getMainDefinition } from '@apollo/client/utilities';
import { GraphQLWsLink } from '@apollo/client/link/subscriptions';
import { createClient } from 'graphql-ws'

import { useQuery as useVueQuery, useMutation as useVueMutation, useSubscription as useVueSubscription } from '@vue/apollo-composable'


const httpLink = createHttpLink({
	uri: `${import.meta.env.VITE_APP_API_BASE_URL}/graphql`,
});

const wsLink = typeof window !== "undefined"
	? new GraphQLWsLink(createClient({
		url: `${import.meta.env.VITE_APP_API_BASE_URL}/graphql/ws`,
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
	cache: new InMemoryCache()
});

export const useQuery = (...args: any[]) => {
	return import.meta.env.SSR ? { result: ref(null) } : useVueQuery(...args)
}

export const useMutation = (...args: any[]) => {
	return import.meta.env.SSR ? { mutate: async () => {} } : useVueMutation(...args)
}

export const useSubscription= (...args: any[]) => {
	return import.meta.env.SSR ? { result: reactive({}) } : useVueSubscription(...args)
}

export {
	client, gql
}