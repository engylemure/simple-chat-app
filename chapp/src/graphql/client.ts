import { ApolloClient, InMemoryCache, split, HttpLink } from '@apollo/client/core';
import { GraphQLWsLink } from '@apollo/client/link/subscriptions';
import { createClient } from 'graphql-ws';
import { getMainDefinition } from '@apollo/client/utilities';

const host = 'localhost:8000';
const uri = `http://${host}`;

const httpLink = new HttpLink({
	uri,
    credentials: 'same-origin'
});

const wsLink =
	typeof window !== 'undefined'
		? new GraphQLWsLink(
				createClient({
					url: `ws://${host}/subscriptions`
				})
		  )
		: undefined;

// The split function takes three parameters:
//
// * A function that's called for each operation to execute
// * The Link to use for an operation if the function returns a "truthy" value
// * The Link to use for an operation if the function returns a "falsy" value
const splitLink =
	typeof window !== 'undefined' && wsLink != null
		? split(
				({ query, getContext }) => {
					const definition = getMainDefinition(query);
					return (
						definition.kind === 'OperationDefinition' && definition.operation === 'subscription'
					);
				},
				wsLink,
				httpLink
		  )
		: httpLink;

const client = new ApolloClient({
	link: splitLink,
	cache: new InMemoryCache()
});

export default client;
