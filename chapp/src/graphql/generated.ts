import client from "./client";
import type {
        ApolloQueryResult, ObservableQuery, WatchQueryOptions, QueryOptions, MutationOptions, SubscriptionOptions
      } from "@apollo/client";
import { readable } from "svelte/store";
import type { Readable } from "svelte/store";
import gql from "graphql-tag"
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string | number; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  DateTime: { input: any; output: any; }
};

export type CreateMessageInput = {
  content: Scalars['String']['input'];
  repliedTo?: InputMaybe<Scalars['ID']['input']>;
  roomId: Scalars['ID']['input'];
};

export type Message = {
  __typename?: 'Message';
  content: Scalars['String']['output'];
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['ID']['output'];
  repliedTo?: Maybe<Message>;
  user?: Maybe<User>;
};

export type MessageConnection = {
  __typename?: 'MessageConnection';
  /** A list of edges. */
  edges: Array<MessageEdge>;
  /** A list of nodes. */
  nodes: Array<Message>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
};

/** An edge in a connection. */
export type MessageEdge = {
  __typename?: 'MessageEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String']['output'];
  /** The item at the end of the edge */
  node: Message;
};

export type Mutation = {
  __typename?: 'Mutation';
  createRoom: Room;
  createUser: User;
  sendMessage: Message;
  updateUser?: Maybe<User>;
};


export type MutationCreateRoomArgs = {
  roomName: Scalars['String']['input'];
};


export type MutationCreateUserArgs = {
  name?: InputMaybe<Scalars['String']['input']>;
};


export type MutationSendMessageArgs = {
  input: CreateMessageInput;
};


export type MutationUpdateUserArgs = {
  name: Scalars['String']['input'];
};

/** Information about pagination in a connection */
export type PageInfo = {
  __typename?: 'PageInfo';
  /** When paginating forwards, the cursor to continue. */
  endCursor?: Maybe<Scalars['String']['output']>;
  /** When paginating forwards, are there more items? */
  hasNextPage: Scalars['Boolean']['output'];
  /** When paginating backwards, are there more items? */
  hasPreviousPage: Scalars['Boolean']['output'];
  /** When paginating backwards, the cursor to continue. */
  startCursor?: Maybe<Scalars['String']['output']>;
};

export type Query = {
  __typename?: 'Query';
  me?: Maybe<User>;
  room?: Maybe<Room>;
  rooms: Array<Room>;
};


export type QueryRoomArgs = {
  roomId: Scalars['ID']['input'];
};

export type Room = {
  __typename?: 'Room';
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['ID']['output'];
  messages: MessageConnection;
  name: Scalars['String']['output'];
};


export type RoomMessagesArgs = {
  after?: InputMaybe<Scalars['String']['input']>;
  before?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};

export type Subscription = {
  __typename?: 'Subscription';
  roomMessages: Message;
};


export type SubscriptionRoomMessagesArgs = {
  roomId: Scalars['ID']['input'];
};

export type User = {
  __typename?: 'User';
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['ID']['output'];
  name?: Maybe<Scalars['String']['output']>;
};

export type CreateUserMutationVariables = Exact<{ [key: string]: never; }>;


export type CreateUserMutation = { __typename?: 'Mutation', createUser: { __typename?: 'User', id: string, name?: string | null } };

export type UpdateUserMutationVariables = Exact<{
  name: Scalars['String']['input'];
}>;


export type UpdateUserMutation = { __typename?: 'Mutation', updateUser?: { __typename?: 'User', id: string, name?: string | null } | null };

export type CreateRoomMutationVariables = Exact<{
  name: Scalars['String']['input'];
}>;


export type CreateRoomMutation = { __typename?: 'Mutation', createRoom: { __typename?: 'Room', id: string, name: string } };

export type SendMessageMutationVariables = Exact<{
  input: CreateMessageInput;
}>;


export type SendMessageMutation = { __typename?: 'Mutation', sendMessage: { __typename?: 'Message', id: string, content: string, createdAt: any, user?: { __typename?: 'User', id: string, name?: string | null, createdAt: any } | null, repliedTo?: { __typename?: 'Message', id: string, content: string, createdAt: any, user?: { __typename?: 'User', id: string, name?: string | null } | null } | null } };

export type MeQueryVariables = Exact<{ [key: string]: never; }>;


export type MeQuery = { __typename?: 'Query', me?: { __typename?: 'User', id: string, name?: string | null, createdAt: any } | null };

export type RoomsQueryVariables = Exact<{ [key: string]: never; }>;


export type RoomsQuery = { __typename?: 'Query', rooms: Array<{ __typename?: 'Room', id: string, name: string, createdAt: any }> };

export type RoomQueryVariables = Exact<{
  id: Scalars['ID']['input'];
  before?: InputMaybe<Scalars['String']['input']>;
  after?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
}>;


export type RoomQuery = { __typename?: 'Query', room?: { __typename?: 'Room', id: string, name: string, createdAt: any, messages: { __typename?: 'MessageConnection', pageInfo: { __typename?: 'PageInfo', hasPreviousPage: boolean, hasNextPage: boolean, startCursor?: string | null, endCursor?: string | null }, nodes: Array<{ __typename?: 'Message', id: string, content: string, createdAt: any, user?: { __typename?: 'User', id: string, name?: string | null, createdAt: any } | null, repliedTo?: { __typename?: 'Message', id: string, content: string, createdAt: any, user?: { __typename?: 'User', id: string, name?: string | null, createdAt: any } | null } | null }> } } | null };

export type RoomMessagesSubscriptionVariables = Exact<{
  id: Scalars['ID']['input'];
}>;


export type RoomMessagesSubscription = { __typename?: 'Subscription', roomMessages: { __typename?: 'Message', id: string, content: string, createdAt: any, user?: { __typename?: 'User', id: string, name?: string | null, createdAt: any } | null, repliedTo?: { __typename?: 'Message', id: string, content: string, createdAt: any, user?: { __typename?: 'User', id: string, name?: string | null, createdAt: any } | null } | null } };


export const CreateUserDoc = gql`
    mutation CreateUser {
  createUser {
    id
    name
  }
}
    `;
export const UpdateUserDoc = gql`
    mutation UpdateUser($name: String!) {
  updateUser(name: $name) {
    id
    name
  }
}
    `;
export const CreateRoomDoc = gql`
    mutation CreateRoom($name: String!) {
  createRoom(roomName: $name) {
    id
    name
  }
}
    `;
export const SendMessageDoc = gql`
    mutation SendMessage($input: CreateMessageInput!) {
  sendMessage(input: $input) {
    id
    content
    createdAt
    user {
      id
      name
      createdAt
    }
    repliedTo {
      id
      content
      createdAt
      user {
        id
        name
      }
    }
  }
}
    `;
export const MeDoc = gql`
    query Me {
  me {
    id
    name
    createdAt
  }
}
    `;
export const RoomsDoc = gql`
    query Rooms {
  rooms {
    id
    name
    createdAt
  }
}
    `;
export const RoomDoc = gql`
    query Room($id: ID!, $before: String, $after: String, $first: Int, $last: Int) {
  room(roomId: $id) {
    id
    name
    createdAt
    messages(before: $before, after: $after, first: $first, last: $last) {
      pageInfo {
        hasPreviousPage
        hasNextPage
        startCursor
        endCursor
      }
      nodes {
        id
        content
        createdAt
        user {
          id
          name
          createdAt
        }
        repliedTo {
          id
          content
          createdAt
          user {
            id
            name
            createdAt
          }
        }
      }
    }
  }
}
    `;
export const RoomMessagesDoc = gql`
    subscription RoomMessages($id: ID!) {
  roomMessages(roomId: $id) {
    id
    content
    createdAt
    user {
      id
      name
      createdAt
    }
    repliedTo {
      id
      content
      createdAt
      user {
        id
        name
        createdAt
      }
    }
  }
}
    `;
export const CreateUser = (
            options: Omit<
              MutationOptions<any, CreateUserMutationVariables>, 
              "mutation"
            >
          ) => {
            const m = client.mutate<CreateUserMutation, CreateUserMutationVariables>({
              mutation: CreateUserDoc,
              ...options,
            });
            return m;
          }
export const UpdateUser = (
            options: Omit<
              MutationOptions<any, UpdateUserMutationVariables>, 
              "mutation"
            >
          ) => {
            const m = client.mutate<UpdateUserMutation, UpdateUserMutationVariables>({
              mutation: UpdateUserDoc,
              ...options,
            });
            return m;
          }
export const CreateRoom = (
            options: Omit<
              MutationOptions<any, CreateRoomMutationVariables>, 
              "mutation"
            >
          ) => {
            const m = client.mutate<CreateRoomMutation, CreateRoomMutationVariables>({
              mutation: CreateRoomDoc,
              ...options,
            });
            return m;
          }
export const SendMessage = (
            options: Omit<
              MutationOptions<any, SendMessageMutationVariables>, 
              "mutation"
            >
          ) => {
            const m = client.mutate<SendMessageMutation, SendMessageMutationVariables>({
              mutation: SendMessageDoc,
              ...options,
            });
            return m;
          }
export const Me = (
            options: Omit<
              WatchQueryOptions<MeQueryVariables>, 
              "query"
            >
          ): Readable<
            ApolloQueryResult<MeQuery> & {
              query: ObservableQuery<
                MeQuery,
                MeQueryVariables
              >;
            }
          > => {
            const q = client.watchQuery({
              query: MeDoc,
              ...options,
            });
            var result = readable<
              ApolloQueryResult<MeQuery> & {
                query: ObservableQuery<
                  MeQuery,
                  MeQueryVariables
                >;
              }
            >(
              { data: {} as any, loading: true, error: undefined, networkStatus: 1, query: q },
              (set) => {
                q.subscribe((v: any) => {
                  set({ ...v, query: q });
                });
              }
            );
            return result;
          }
        
              export const AsyncMe = (
                options: Omit<
                  QueryOptions<MeQueryVariables>,
                  "query"
                >
              ) => {
                return client.query<MeQuery>({query: MeDoc, ...options})
              }
            
export const Rooms = (
            options: Omit<
              WatchQueryOptions<RoomsQueryVariables>, 
              "query"
            >
          ): Readable<
            ApolloQueryResult<RoomsQuery> & {
              query: ObservableQuery<
                RoomsQuery,
                RoomsQueryVariables
              >;
            }
          > => {
            const q = client.watchQuery({
              query: RoomsDoc,
              ...options,
            });
            var result = readable<
              ApolloQueryResult<RoomsQuery> & {
                query: ObservableQuery<
                  RoomsQuery,
                  RoomsQueryVariables
                >;
              }
            >(
              { data: {} as any, loading: true, error: undefined, networkStatus: 1, query: q },
              (set) => {
                q.subscribe((v: any) => {
                  set({ ...v, query: q });
                });
              }
            );
            return result;
          }
        
              export const AsyncRooms = (
                options: Omit<
                  QueryOptions<RoomsQueryVariables>,
                  "query"
                >
              ) => {
                return client.query<RoomsQuery>({query: RoomsDoc, ...options})
              }
            
export const Room = (
            options: Omit<
              WatchQueryOptions<RoomQueryVariables>, 
              "query"
            >
          ): Readable<
            ApolloQueryResult<RoomQuery> & {
              query: ObservableQuery<
                RoomQuery,
                RoomQueryVariables
              >;
            }
          > => {
            const q = client.watchQuery({
              query: RoomDoc,
              ...options,
            });
            var result = readable<
              ApolloQueryResult<RoomQuery> & {
                query: ObservableQuery<
                  RoomQuery,
                  RoomQueryVariables
                >;
              }
            >(
              { data: {} as any, loading: true, error: undefined, networkStatus: 1, query: q },
              (set) => {
                q.subscribe((v: any) => {
                  set({ ...v, query: q });
                });
              }
            );
            return result;
          }
        
              export const AsyncRoom = (
                options: Omit<
                  QueryOptions<RoomQueryVariables>,
                  "query"
                >
              ) => {
                return client.query<RoomQuery>({query: RoomDoc, ...options})
              }
            
export const RoomMessages = (
            options: Omit<SubscriptionOptions<RoomMessagesSubscriptionVariables>, "query">
          ) => {
            const q = client.subscribe<RoomMessagesSubscription, RoomMessagesSubscriptionVariables>(
              {
                query: RoomMessagesDoc,
                ...options,
              }
            )
            return q;
          }